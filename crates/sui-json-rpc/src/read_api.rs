// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use futures::future::join_all;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_core_types::annotated_value::{MoveStruct, MoveStructLayout, MoveValue};
use move_core_types::language_storage::StructTag;
use tracing::{error, info, instrument, warn};

use mysten_metrics::spawn_monitored_task;
use sui_json_rpc_api::{JsonRpcMetrics, ReadApiOpenRpc, ReadApiServer, QUERY_MAX_RESULT_LIMIT};
use sui_json_rpc_types::{
    Checkpoint, CheckpointId, CheckpointPage, DisplayFieldsResponse, EventFilter,
    ProtocolConfigResponse, SuiEvent, SuiGetPastObjectRequest, SuiMoveStruct, SuiMoveValue,
    SuiObjectDataOptions, SuiObjectResponse, SuiPastObjectResponse, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};
use sui_json_rpc_types::{SuiLoadedChildObject, SuiLoadedChildObjectsResponse};
use sui_open_rpc::Module;
use sui_types::base_types::{ObjectID, SequenceNumber, TransactionDigest};
use sui_types::collection_types::VecMap;
use sui_types::display::DisplayVersionUpdatedEvent;
use sui_types::error::{SuiError, SuiObjectResponseError};
use sui_types::object::{Object, ObjectRead, PastObjectRead};
use sui_types::sui_serde::BigInt;

use crate::error::{Error, SuiRpcInputError};
use crate::state::{StateRead, StateReadError};
use crate::with_tracing;
use crate::SuiRpcModule;

const MAX_DISPLAY_NESTED_LEVEL: usize = 10;

// An implementation of the read portion of the JSON-RPC interface intended for use in
// Fullnodes.
#[derive(Clone)]
pub struct ReadApi {
    pub state: Arc<dyn StateRead>,
    pub metrics: Arc<JsonRpcMetrics>,
}

impl ReadApi {
    pub fn new(state: Arc<dyn StateRead>, metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { state, metrics }
    }
}

#[async_trait]
impl ReadApiServer for ReadApi {
    #[instrument(skip(self))]
    async fn get_object(
        &self,
        object_id: ObjectID,
        options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<SuiObjectResponse> {
        with_tracing!(async move {
            let state = self.state.clone();
            let object_read = spawn_monitored_task!(async move {
                state.get_object_read(&object_id).map_err(|e| {
                    warn!(?object_id, "Failed to get object: {:?}", e);
                    Error::from(e)
                })
            })
            .await
            .map_err(Error::from)??;
            let options = options.unwrap_or_default();

            match object_read {
                ObjectRead::NotExists(id) => Ok(SuiObjectResponse::new_with_error(
                    SuiObjectResponseError::NotExists { object_id: id },
                )),
                ObjectRead::Exists(object_ref, o, layout) => {
                    let mut display_fields = None;
                    if options.show_display {
                        match get_display_fields(self, &o, &layout).await {
                            Ok(rendered_fields) => display_fields = Some(rendered_fields),
                            Err(e) => {
                                return Ok(SuiObjectResponse::new(
                                    Some((object_ref, o, layout, options, None).try_into()?),
                                    Some(SuiObjectResponseError::DisplayError {
                                        error: e.to_string(),
                                    }),
                                ));
                            }
                        }
                    }
                    Ok(SuiObjectResponse::new_with_data(
                        (object_ref, o, layout, options, display_fields).try_into()?,
                    ))
                }
                ObjectRead::Deleted((object_id, version, digest)) => Ok(
                    SuiObjectResponse::new_with_error(SuiObjectResponseError::Deleted {
                        object_id,
                        version,
                        digest,
                    }),
                ),
            }
        })
    }

    #[instrument(skip(self))]
    async fn multi_get_objects(
        &self,
        object_ids: Vec<ObjectID>,
        options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<Vec<SuiObjectResponse>> {
        with_tracing!(async move {
            if object_ids.len() <= *QUERY_MAX_RESULT_LIMIT {
                self.metrics
                    .get_objects_limit
                    .report(object_ids.len() as u64);
                let mut futures = vec![];
                for object_id in object_ids {
                    futures.push(self.get_object(object_id, options.clone()));
                }
                let results = join_all(futures).await;

                let objects_result: Result<Vec<SuiObjectResponse>, String> = results
                    .into_iter()
                    .map(|result| match result {
                        Ok(response) => Ok(response),
                        Err(error) => {
                            error!("Failed to fetch object with error: {error:?}");
                            Err(format!("Error: {}", error))
                        }
                    })
                    .collect();

                let objects = objects_result.map_err(|err| {
                    Error::UnexpectedError(format!("Failed to fetch objects with error: {}", err))
                })?;

                self.metrics
                    .get_objects_result_size
                    .report(objects.len() as u64);
                self.metrics
                    .get_objects_result_size_total
                    .inc_by(objects.len() as u64);
                Ok(objects)
            } else {
                Err(SuiRpcInputError::SizeLimitExceeded(
                    QUERY_MAX_RESULT_LIMIT.to_string(),
                ))?
            }
        })
    }

    #[instrument(skip(self))]
    async fn try_get_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
        options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<SuiPastObjectResponse> {
        with_tracing!(async move {
            let state = self.state.clone();
            let past_read = spawn_monitored_task!(async move {
            state.get_past_object_read(&object_id, version)
            .map_err(|e| {
                error!("Failed to call try_get_past_object for object: {object_id:?} version: {version:?} with error: {e:?}");
                Error::from(e)
            })}).await.map_err(Error::from)??;
            let options = options.unwrap_or_default();
            match past_read {
                PastObjectRead::ObjectNotExists(id) => {
                    Ok(SuiPastObjectResponse::ObjectNotExists(id))
                }
                PastObjectRead::VersionFound(object_ref, o, layout) => {
                    let display_fields = if options.show_display {
                        // TODO (jian): api breaking change to also modify past objects.
                        Some(get_display_fields(self, &o, &layout).await.map_err(|e| {
                            Error::UnexpectedError(format!(
                                "Unable to render object at version {version}: {e}"
                            ))
                        })?)
                    } else {
                        None
                    };
                    Ok(SuiPastObjectResponse::VersionFound(
                        (object_ref, o, layout, options, display_fields).try_into()?,
                    ))
                }
                PastObjectRead::ObjectDeleted(oref) => {
                    Ok(SuiPastObjectResponse::ObjectDeleted(oref.into()))
                }
                PastObjectRead::VersionNotFound(id, seq_num) => {
                    Ok(SuiPastObjectResponse::VersionNotFound(id, seq_num))
                }
                PastObjectRead::VersionTooHigh {
                    object_id,
                    asked_version,
                    latest_version,
                } => Ok(SuiPastObjectResponse::VersionTooHigh {
                    object_id,
                    asked_version,
                    latest_version,
                }),
            }
        })
    }

    #[instrument(skip(self))]
    async fn try_multi_get_past_objects(
        &self,
        past_objects: Vec<SuiGetPastObjectRequest>,
        options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<Vec<SuiPastObjectResponse>> {
        with_tracing!(async move {
            if past_objects.len() <= *QUERY_MAX_RESULT_LIMIT {
                let mut futures = vec![];
                for past_object in past_objects {
                    futures.push(self.try_get_past_object(
                        past_object.object_id,
                        past_object.version,
                        options.clone(),
                    ));
                }
                let results = join_all(futures).await;

                let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
                let success = oks.into_iter().filter_map(Result::ok).collect();
                let errors: Vec<_> = errs.into_iter().filter_map(Result::err).collect();
                if !errors.is_empty() {
                    let error_string = errors
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join("; ");
                    Err(anyhow!("{error_string}").into()) // Collects errors not related to SuiPastObjectResponse variants
                } else {
                    Ok(success)
                }
            } else {
                Err(SuiRpcInputError::SizeLimitExceeded(
                    QUERY_MAX_RESULT_LIMIT.to_string(),
                ))?
            }
        })
    }

    #[instrument(skip(self))]
    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt<u64>> {
        with_tracing!(async move {
            Ok(self
                .state
                .get_total_transaction_blocks()
                .map_err(Error::from)?
                .into()) // converts into BigInt<u64>
        })
    }

    #[instrument(skip(self))]
    async fn get_transaction_block(
        &self,
        _digest: TransactionDigest,
        _opts: Option<SuiTransactionBlockResponseOptions>,
    ) -> RpcResult<SuiTransactionBlockResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn multi_get_transaction_blocks(
        &self,
        _digests: Vec<TransactionDigest>,
        _opts: Option<SuiTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<SuiTransactionBlockResponse>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_events(&self, _transaction_digest: TransactionDigest) -> RpcResult<Vec<SuiEvent>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<BigInt<u64>> {
        with_tracing!(async move {
            Ok(self
                .state
                .get_latest_checkpoint_sequence_number()
                .map_err(|e| {
                    SuiRpcInputError::GenericNotFound(format!(
                        "Latest checkpoint sequence number was not found with error :{e}"
                    ))
                })?
                .into())
        })
    }

    #[instrument(skip(self))]
    async fn get_checkpoint(&self, _id: CheckpointId) -> RpcResult<Checkpoint> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_checkpoints(
        &self,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<BigInt<u64>>,
        _limit: Option<usize>,
        _descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_checkpoints_deprecated_limit(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<BigInt<u64>>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        with_tracing!(async move {
            self.get_checkpoints(cursor, limit.map(|l| *l as usize), descending_order)
                .await
                .map_err(Error::from)
        })
    }

    #[instrument(skip(self))]
    async fn get_loaded_child_objects(
        &self,
        digest: TransactionDigest,
    ) -> RpcResult<SuiLoadedChildObjectsResponse> {
        with_tracing!(async move {
            Ok(SuiLoadedChildObjectsResponse {
                loaded_child_objects: match self
                    .state
                    .loaded_child_object_versions(&digest)
                    .map_err(|e| {
                        error!(
                            "Failed to get loaded child objects at {digest:?} with error: {e:?}"
                        );
                        Error::StateReadError(e)
                    })? {
                    Some(v) => v
                        .into_iter()
                        .map(|q| SuiLoadedChildObject::new(q.0, q.1))
                        .collect::<Vec<_>>(),
                    None => vec![],
                },
            })
        })
    }

    #[instrument(skip(self))]
    async fn get_protocol_config(
        &self,
        _version: Option<BigInt<u64>>,
    ) -> RpcResult<ProtocolConfigResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_chain_identifier(&self) -> RpcResult<String> {
        with_tracing!(async move {
            let ci = self.state.get_chain_identifier()?;
            Ok(ci.to_string())
        })
    }
}

impl SuiRpcModule for ReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        ReadApiOpenRpc::module_doc()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ObjectDisplayError {
    #[error("Not a move struct")]
    NotMoveStruct,

    #[error("Failed to extract layout")]
    Layout,

    #[error("Failed to extract Move object")]
    MoveObject,

    #[error(transparent)]
    Deserialization(#[from] SuiError),

    #[error("Failed to deserialize 'VersionUpdatedEvent': {0}")]
    Bcs(#[from] bcs::Error),

    #[error(transparent)]
    StateReadError(#[from] StateReadError),
}

async fn get_display_fields(
    fullnode_api: &ReadApi,
    original_object: &Object,
    original_layout: &Option<MoveStructLayout>,
) -> Result<DisplayFieldsResponse, ObjectDisplayError> {
    let Some((object_type, layout)) = get_object_type_and_struct(original_object, original_layout)?
    else {
        return Ok(DisplayFieldsResponse {
            data: None,
            error: None,
        });
    };
    if let Some(display_object) = get_display_object_by_type(fullnode_api, &object_type).await? {
        return get_rendered_fields(display_object.fields, &layout);
    }
    Ok(DisplayFieldsResponse {
        data: None,
        error: None,
    })
}

async fn get_display_object_by_type(
    fullnode_api: &ReadApi,
    object_type: &StructTag,
    // TODO: add query version support
) -> Result<Option<DisplayVersionUpdatedEvent>, ObjectDisplayError> {
    let mut events = fullnode_api
        .state
        .query_events(
            EventFilter::MoveEventType(DisplayVersionUpdatedEvent::type_(object_type)),
            None,
            1,
            true,
        )
        .await?;

    // If there's any recent version of Display, give it to the client.
    // TODO: add support for version query.
    if let Some(event) = events.pop() {
        let display: DisplayVersionUpdatedEvent = bcs::from_bytes(&event.bcs[..])?;
        Ok(Some(display))
    } else {
        Ok(None)
    }
}

pub fn get_object_type_and_struct(
    o: &Object,
    layout: &Option<MoveStructLayout>,
) -> Result<Option<(StructTag, MoveStruct)>, ObjectDisplayError> {
    if let Some(object_type) = o.type_() {
        let move_struct = get_move_struct(o, layout)?;
        Ok(Some((object_type.clone().into(), move_struct)))
    } else {
        Ok(None)
    }
}

fn get_move_struct(
    o: &Object,
    layout: &Option<MoveStructLayout>,
) -> Result<MoveStruct, ObjectDisplayError> {
    let layout = layout.as_ref().ok_or_else(|| ObjectDisplayError::Layout)?;
    Ok(o.data
        .try_as_move()
        .ok_or_else(|| ObjectDisplayError::MoveObject)?
        .to_move_struct(layout)?)
}

pub fn get_rendered_fields(
    fields: VecMap<String, String>,
    move_struct: &MoveStruct,
) -> Result<DisplayFieldsResponse, ObjectDisplayError> {
    let sui_move_value: SuiMoveValue = MoveValue::Struct(move_struct.clone()).into();
    if let SuiMoveValue::Struct(move_struct) = sui_move_value {
        let fields =
            fields
                .contents
                .iter()
                .map(|entry| match parse_template(&entry.value, &move_struct) {
                    Ok(value) => Ok((entry.key.clone(), value)),
                    Err(e) => Err(e),
                });
        let (oks, errs): (Vec<_>, Vec<_>) = fields.partition(Result::is_ok);
        let success = oks.into_iter().filter_map(Result::ok).collect();
        let errors: Vec<_> = errs.into_iter().filter_map(Result::err).collect();
        let error_string = errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        let error = if !error_string.is_empty() {
            Some(SuiObjectResponseError::DisplayError {
                error: anyhow!("{error_string}").to_string(),
            })
        } else {
            None
        };

        return Ok(DisplayFieldsResponse {
            data: Some(success),
            error,
        });
    }
    Err(ObjectDisplayError::NotMoveStruct)?
}

fn parse_template(template: &str, move_struct: &SuiMoveStruct) -> Result<String, Error> {
    let mut output = template.to_string();
    let mut var_name = String::new();
    let mut in_braces = false;
    let mut escaped = false;

    for ch in template.chars() {
        match ch {
            '\\' => {
                escaped = true;
                continue;
            }
            '{' if !escaped => {
                in_braces = true;
                var_name.clear();
            }
            '}' if !escaped => {
                in_braces = false;
                let value = get_value_from_move_struct(move_struct, &var_name)?;
                output = output.replace(&format!("{{{}}}", var_name), &value.to_string());
            }
            _ if !escaped => {
                if in_braces {
                    var_name.push(ch);
                }
            }
            _ => {}
        }
        escaped = false;
    }

    Ok(output.replace('\\', ""))
}

fn get_value_from_move_struct(
    move_struct: &SuiMoveStruct,
    var_name: &str,
) -> Result<String, Error> {
    let parts: Vec<&str> = var_name.split('.').collect();
    if parts.is_empty() {
        Err(anyhow!("Display template value cannot be empty"))?;
    }
    if parts.len() > MAX_DISPLAY_NESTED_LEVEL {
        Err(anyhow!(
            "Display template value nested depth cannot exist {}",
            MAX_DISPLAY_NESTED_LEVEL
        ))?;
    }
    let mut current_value = &SuiMoveValue::Struct(move_struct.clone());
    // iterate over the parts and try to access the corresponding field
    for part in parts {
        match current_value {
            SuiMoveValue::Struct(move_struct) => {
                if let SuiMoveStruct::WithTypes { type_: _, fields }
                | SuiMoveStruct::WithFields(fields) = move_struct
                {
                    if let Some(value) = fields.get(part) {
                        current_value = value;
                    } else {
                        Err(anyhow!(
                            "Field value {} cannot be found in struct",
                            var_name
                        ))?;
                    }
                } else {
                    Err(Error::UnexpectedError(format!(
                        "Unexpected move struct type for field {}",
                        var_name
                    )))?;
                }
            }
            _ => {
                return Err(Error::UnexpectedError(format!(
                    "Unexpected move value type for field {}",
                    var_name
                )))?
            }
        }
    }

    match current_value {
        SuiMoveValue::Option(move_option) => match move_option.as_ref() {
            Some(move_value) => Ok(move_value.to_string()),
            None => Ok("".to_string()),
        },
        SuiMoveValue::Vector(_) => Err(anyhow!(
            "Vector is not supported as a Display value {}",
            var_name
        ))?,

        _ => Ok(current_value.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use sui_types::messages_checkpoint::CheckpointSequenceNumber;

    fn calculate_checkpoint_numbers(
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<CheckpointSequenceNumber>,
        limit: u64,
        descending_order: bool,
        max_checkpoint: CheckpointSequenceNumber,
    ) -> Vec<CheckpointSequenceNumber> {
        let (start_index, end_index) = match cursor {
            Some(t) => {
                if descending_order {
                    let start = std::cmp::min(t.saturating_sub(1), max_checkpoint);
                    let end = start.saturating_sub(limit - 1);
                    (end, start)
                } else {
                    let start =
                        std::cmp::min(t.checked_add(1).unwrap_or(max_checkpoint), max_checkpoint);
                    let end = std::cmp::min(
                        start.checked_add(limit - 1).unwrap_or(max_checkpoint),
                        max_checkpoint,
                    );
                    (start, end)
                }
            }
            None => {
                if descending_order {
                    (max_checkpoint.saturating_sub(limit - 1), max_checkpoint)
                } else {
                    (0, std::cmp::min(limit - 1, max_checkpoint))
                }
            }
        };

        if descending_order {
            (start_index..=end_index).rev().collect()
        } else {
            (start_index..=end_index).collect()
        }
    }

    #[test]
    fn test_calculate_checkpoint_numbers() {
        let cursor = Some(10);
        let limit = 5;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![9, 8, 7, 6, 5]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_descending_no_cursor() {
        let cursor = None;
        let limit = 5;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![15, 14, 13, 12, 11]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_no_cursor() {
        let cursor = None;
        let limit = 5;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_with_cursor() {
        let cursor = Some(10);
        let limit = 5;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![11, 12, 13, 14, 15]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_limit_exceeds_max() {
        let cursor = None;
        let limit = 20;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, (0..=15).collect::<Vec<_>>());
    }

    #[test]
    fn test_calculate_checkpoint_numbers_descending_limit_exceeds_max() {
        let cursor = None;
        let limit = 20;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, (0..=15).rev().collect::<Vec<_>>());
    }
}
