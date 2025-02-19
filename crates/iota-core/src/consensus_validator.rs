// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use consensus_core::{TransactionVerifier, ValidationError};
use eyre::WrapErr;
use fastcrypto_tbls::dkg;
use iota_metrics::monitored_scope;
use iota_types::{
    error::IotaError,
    messages_consensus::{ConsensusTransaction, ConsensusTransactionKind},
};
use prometheus::{IntCounter, Registry, register_int_counter_with_registry};
use tap::TapFallible;
use tracing::{info, warn};

use crate::{
    authority::authority_per_epoch_store::AuthorityPerEpochStore,
    checkpoints::CheckpointServiceNotify, transaction_manager::TransactionManager,
};

/// Allows verifying the validity of transactions
#[derive(Clone)]
pub struct IotaTxValidator {
    epoch_store: Arc<AuthorityPerEpochStore>,
    checkpoint_service: Arc<dyn CheckpointServiceNotify + Send + Sync>,
    _transaction_manager: Arc<TransactionManager>,
    metrics: Arc<IotaTxValidatorMetrics>,
}

impl IotaTxValidator {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        checkpoint_service: Arc<dyn CheckpointServiceNotify + Send + Sync>,
        transaction_manager: Arc<TransactionManager>,
        metrics: Arc<IotaTxValidatorMetrics>,
    ) -> Self {
        info!(
            "IotaTxValidator constructed for epoch {}",
            epoch_store.epoch()
        );
        Self {
            epoch_store,
            checkpoint_service,
            _transaction_manager: transaction_manager,
            metrics,
        }
    }

    fn validate_transactions(
        &self,
        txs: Vec<ConsensusTransactionKind>,
    ) -> Result<(), eyre::Report> {
        let mut cert_batch = Vec::new();
        let mut ckpt_messages = Vec::new();
        let mut ckpt_batch = Vec::new();
        for tx in txs.into_iter() {
            match tx {
                ConsensusTransactionKind::UserTransaction(certificate) => {
                    cert_batch.push(*certificate);

                    // if !certificate.contains_shared_object() {
                    //     // new_unchecked safety: we do not use the certs in
                    // this list until all     // have had
                    // their signatures verified.
                    //     owned_tx_certs.
                    // push(VerifiedCertificate::new_unchecked(*certificate));
                    // }
                }
                ConsensusTransactionKind::CheckpointSignature(signature) => {
                    ckpt_messages.push(signature.clone());
                    ckpt_batch.push(signature.summary);
                }
                ConsensusTransactionKind::RandomnessDkgMessage(_, bytes) => {
                    if bytes.len() > dkg::DKG_MESSAGES_MAX_SIZE {
                        warn!("batch verification error: DKG Message too large");
                        return Err(IotaError::InvalidDkgMessageSize.into());
                    }
                }
                ConsensusTransactionKind::RandomnessDkgConfirmation(_, bytes) => {
                    if bytes.len() > dkg::DKG_MESSAGES_MAX_SIZE {
                        warn!("batch verification error: DKG Confirmation too large");
                        return Err(IotaError::InvalidDkgMessageSize.into());
                    }
                }

                ConsensusTransactionKind::EndOfPublish(_)
                | ConsensusTransactionKind::NewJWKFetched(_, _, _)
                | ConsensusTransactionKind::CapabilityNotificationV1(_) => {}
            }
        }

        // verify the certificate signatures as a batch
        let cert_count = cert_batch.len();
        let ckpt_count = ckpt_batch.len();

        self.epoch_store
            .signature_verifier
            .verify_certs_and_checkpoints(cert_batch, ckpt_batch)
            .tap_err(|e| warn!("batch verification error: {}", e))
            .wrap_err("Malformed batch (failed to verify)")?;

        // All checkpoint sigs have been verified, forward them to the checkpoint
        // service
        for ckpt in ckpt_messages {
            self.checkpoint_service
                .notify_checkpoint_signature(&self.epoch_store, &ckpt)?;
        }

        self.metrics
            .certificate_signatures_verified
            .inc_by(cert_count as u64);
        self.metrics
            .checkpoint_signatures_verified
            .inc_by(ckpt_count as u64);
        Ok(())

        // todo - we should un-comment line below once we have a way to revert
        // those transactions at the end of epoch all certificates had
        // valid signatures, schedule them for execution prior to sequencing
        // which is unnecessary for owned object transactions.
        // It is unnecessary to write to pending_certificates table because the
        // certs will be written via consensus output.
        // self.transaction_manager
        //     .enqueue_certificates(owned_tx_certs, &self.epoch_store)
        //     .wrap_err("Failed to schedule certificates for execution")
    }
}

fn tx_from_bytes(tx: &[u8]) -> Result<ConsensusTransaction, eyre::Report> {
    bcs::from_bytes::<ConsensusTransaction>(tx)
        .wrap_err("Malformed transaction (failed to deserialize)")
}

impl TransactionVerifier for IotaTxValidator {
    fn verify_batch(&self, batch: &[&[u8]]) -> Result<(), ValidationError> {
        let _scope = monitored_scope("ValidateBatch");

        let txs = batch
            .iter()
            .map(|tx| {
                tx_from_bytes(tx)
                    .map(|tx| tx.kind)
                    .map_err(|e| ValidationError::InvalidTransaction(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.validate_transactions(txs)
            .map_err(|e| ValidationError::InvalidTransaction(e.to_string()))
    }
}

pub struct IotaTxValidatorMetrics {
    certificate_signatures_verified: IntCounter,
    checkpoint_signatures_verified: IntCounter,
}

impl IotaTxValidatorMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        Arc::new(Self {
            certificate_signatures_verified: register_int_counter_with_registry!(
                "certificate_signatures_verified",
                "Number of certificates verified in consensus batch verifier",
                registry
            )
            .unwrap(),
            checkpoint_signatures_verified: register_int_counter_with_registry!(
                "checkpoint_signatures_verified",
                "Number of checkpoint verified in consensus batch verifier",
                registry
            )
            .unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use consensus_core::TransactionVerifier as _;
    use iota_macros::sim_test;
    use iota_types::{
        crypto::Ed25519IotaSignature, messages_consensus::ConsensusTransaction, object::Object,
        signature::GenericSignature,
    };

    use crate::{
        authority::test_authority_builder::TestAuthorityBuilder,
        checkpoints::CheckpointServiceNoop,
        consensus_adapter::consensus_tests::{test_certificates, test_gas_objects},
        consensus_validator::{IotaTxValidator, IotaTxValidatorMetrics},
    };

    #[sim_test]
    async fn accept_valid_transaction() {
        // Initialize an authority with a (owned) gas object and a shared object; then
        // make a test certificate.
        let mut objects = test_gas_objects();
        let shared_object = Object::shared_for_testing();
        objects.push(shared_object.clone());

        let network_config =
            iota_swarm_config::network_config_builder::ConfigBuilder::new_with_temp_dir()
                .with_objects(objects.clone())
                .build();

        let state = TestAuthorityBuilder::new()
            .with_network_config(&network_config, 0)
            .build()
            .await;
        let name1 = state.name;
        let certificates = test_certificates(&state, shared_object).await;

        let first_transaction = certificates[0].clone();
        let first_transaction_bytes: Vec<u8> = bcs::to_bytes(
            &ConsensusTransaction::new_certificate_message(&name1, first_transaction),
        )
        .unwrap();

        let metrics = IotaTxValidatorMetrics::new(&Default::default());
        let validator = IotaTxValidator::new(
            state.epoch_store_for_testing().clone(),
            Arc::new(CheckpointServiceNoop {}),
            state.transaction_manager().clone(),
            metrics,
        );
        let res = validator.verify_batch(&[&first_transaction_bytes]);
        assert!(res.is_ok(), "{res:?}");

        let transaction_bytes: Vec<_> = certificates
            .clone()
            .into_iter()
            .map(|cert| {
                bcs::to_bytes(&ConsensusTransaction::new_certificate_message(&name1, cert)).unwrap()
            })
            .collect();

        let batch: Vec<_> = transaction_bytes.iter().map(|t| t.as_slice()).collect();
        let res_batch = validator.verify_batch(&batch);
        assert!(res_batch.is_ok(), "{res_batch:?}");

        let bogus_transaction_bytes: Vec<_> = certificates
            .into_iter()
            .map(|mut cert| {
                // set it to an all-zero user signature
                cert.tx_signatures_mut_for_testing()[0] = GenericSignature::Signature(
                    iota_types::crypto::Signature::Ed25519IotaSignature(
                        Ed25519IotaSignature::default(),
                    ),
                );
                bcs::to_bytes(&ConsensusTransaction::new_certificate_message(&name1, cert)).unwrap()
            })
            .collect();

        let batch: Vec<_> = bogus_transaction_bytes
            .iter()
            .map(|t| t.as_slice())
            .collect();
        let res_batch = validator.verify_batch(&batch);
        assert!(res_batch.is_err());
    }
}
