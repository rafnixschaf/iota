// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::BTreeMap,
    fmt,
    fmt::{Display, Formatter, Write},
};

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::{
    file_format::{Ability, AbilitySet, StructTypeParameter, Visibility},
    normalized::{
        Field as NormalizedField, Function as IOTANormalizedFunction, Module as NormalizedModule,
        Struct as NormalizedStruct, Type as NormalizedType,
    },
};
use move_core_types::{
    annotated_value::{MoveStruct, MoveValue},
    identifier::Identifier,
    language_storage::StructTag,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use iota_macros::EnumVariantOrder;
use iota_types::{
    base_types::{ObjectID, IOTAAddress},
    iota_serde::IOTAStructTag,
};
use tracing::warn;

pub type IOTAMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/iota_move_tests.rs"]
mod iota_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IOTAMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IOTAMoveAbilitySet {
    pub abilities: Vec<IOTAMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IOTAMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IOTAMoveStructTypeParameter {
    pub constraints: IOTAMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IOTAMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IOTAMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IOTAMoveNormalizedStruct {
    pub abilities: IOTAMoveAbilitySet,
    pub type_parameters: Vec<IOTAMoveStructTypeParameter>,
    pub fields: Vec<IOTAMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IOTAMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    #[serde(rename_all = "camelCase")]
    Struct {
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<IOTAMoveNormalizedType>,
    },
    Vector(Box<IOTAMoveNormalizedType>),
    TypeParameter(IOTAMoveTypeParameterIndex),
    Reference(Box<IOTAMoveNormalizedType>),
    MutableReference(Box<IOTAMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IOTAMoveNormalizedFunction {
    pub visibility: IOTAMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<IOTAMoveAbilitySet>,
    pub parameters: Vec<IOTAMoveNormalizedType>,
    pub return_: Vec<IOTAMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IOTAMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IOTAMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<IOTAMoveModuleId>,
    pub structs: BTreeMap<String, IOTAMoveNormalizedStruct>,
    pub exposed_functions: BTreeMap<String, IOTAMoveNormalizedFunction>,
}

impl PartialEq for IOTAMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for IOTAMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| IOTAMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<IOTAMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), IOTAMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, IOTAMoveNormalizedStruct>>(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), IOTAMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, IOTAMoveNormalizedFunction>>(),
        }
    }
}

impl From<IOTANormalizedFunction> for IOTAMoveNormalizedFunction {
    fn from(function: IOTANormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => IOTAMoveVisibility::Private,
                Visibility::Public => IOTAMoveVisibility::Public,
                Visibility::Friend => IOTAMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<IOTAMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(IOTAMoveNormalizedType::from)
                .collect::<Vec<IOTAMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(IOTAMoveNormalizedType::from)
                .collect::<Vec<IOTAMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for IOTAMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(IOTAMoveStructTypeParameter::from)
                .collect::<Vec<IOTAMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(IOTAMoveNormalizedField::from)
                .collect::<Vec<IOTAMoveNormalizedField>>(),
        }
    }
}

impl From<StructTypeParameter> for IOTAMoveStructTypeParameter {
    fn from(type_parameter: StructTypeParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for IOTAMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: IOTAMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for IOTAMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => IOTAMoveNormalizedType::Bool,
            NormalizedType::U8 => IOTAMoveNormalizedType::U8,
            NormalizedType::U16 => IOTAMoveNormalizedType::U16,
            NormalizedType::U32 => IOTAMoveNormalizedType::U32,
            NormalizedType::U64 => IOTAMoveNormalizedType::U64,
            NormalizedType::U128 => IOTAMoveNormalizedType::U128,
            NormalizedType::U256 => IOTAMoveNormalizedType::U256,
            NormalizedType::Address => IOTAMoveNormalizedType::Address,
            NormalizedType::Signer => IOTAMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => IOTAMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(IOTAMoveNormalizedType::from)
                    .collect::<Vec<IOTAMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                IOTAMoveNormalizedType::Vector(Box::new(IOTAMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => IOTAMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                IOTAMoveNormalizedType::Reference(Box::new(IOTAMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                IOTAMoveNormalizedType::MutableReference(Box::new(IOTAMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for IOTAMoveAbilitySet {
    fn from(set: AbilitySet) -> IOTAMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => IOTAMoveAbility::Copy,
                    Ability::Drop => IOTAMoveAbility::Drop,
                    Ability::Key => IOTAMoveAbility::Key,
                    Ability::Store => IOTAMoveAbility::Store,
                })
                .collect::<Vec<IOTAMoveAbility>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum IOTAMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(IOTAAddress),
    Vector(Vec<IOTAMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(IOTAMoveStruct),
    Option(Box<Option<IOTAMoveValue>>),
}

impl IOTAMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            IOTAMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            IOTAMoveValue::Vector(values) => IOTAMoveStruct::Runtime(values).to_json_value(),
            IOTAMoveValue::Number(v) => json!(v),
            IOTAMoveValue::Bool(v) => json!(v),
            IOTAMoveValue::Address(v) => json!(v),
            IOTAMoveValue::String(v) => json!(v),
            IOTAMoveValue::UID { id } => json!({ "id": id }),
            IOTAMoveValue::Option(v) => json!(v),
        }
    }
}

impl Display for IOTAMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IOTAMoveValue::Number(value) => write!(writer, "{}", value)?,
            IOTAMoveValue::Bool(value) => write!(writer, "{}", value)?,
            IOTAMoveValue::Address(value) => write!(writer, "{}", value)?,
            IOTAMoveValue::String(value) => write!(writer, "{}", value)?,
            IOTAMoveValue::UID { id } => write!(writer, "{id}")?,
            IOTAMoveValue::Struct(value) => write!(writer, "{}", value)?,
            IOTAMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            IOTAMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for IOTAMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => IOTAMoveValue::Number(value.into()),
            MoveValue::U16(value) => IOTAMoveValue::Number(value.into()),
            MoveValue::U32(value) => IOTAMoveValue::Number(value),
            MoveValue::U64(value) => IOTAMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => IOTAMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => IOTAMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => IOTAMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                IOTAMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort IOTA core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                IOTAMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                IOTAMoveValue::Address(IOTAAddress::from(ObjectID::from(value)))
            }
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum IOTAMoveStruct {
    Runtime(Vec<IOTAMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "IOTAStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, IOTAMoveValue>,
    },
    WithFields(BTreeMap<String, IOTAMoveValue>),
}

impl IOTAMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            IOTAMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the
            // client side.
            IOTAMoveStruct::WithTypes { type_: _, fields } | IOTAMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn read_dynamic_field_value(&self, field_name: &str) -> Option<IOTAMoveValue> {
        match self {
            IOTAMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            IOTAMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for IOTAMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IOTAMoveStruct::Runtime(_) => {}
            IOTAMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            IOTAMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<IOTAMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(IOTAMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(IOTAMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(IOTAMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(IOTAMoveValue::from);
            if let Some(IOTAMoveValue::Address(address)) = id {
                return Some(IOTAMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(IOTAMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(IOTAMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(IOTAMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to IOTAMoveValue"
    );
    None
}

impl From<MoveStruct> for IOTAMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        IOTAMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
