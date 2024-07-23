// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! call_args {
        ($($value:expr),*) => {
        Ok::<_, anyhow::Error>(vec![$(iota_json::call_arg!($value)?,)*])
    };
    }

#[macro_export]
macro_rules! call_arg {
    ($value:expr) => {{
        use iota_json::IotaJsonValue;
        trait IotaJsonArg {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue>;
        }
        // TODO: anyway to condense this?
        impl IotaJsonArg for &str {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_str(self)
            }
        }
        impl IotaJsonArg for String {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_str(&self)
            }
        }
        impl IotaJsonArg for iota_types::base_types::ObjectID {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_str(&self.to_string())
            }
        }
        impl IotaJsonArg for iota_types::base_types::IotaAddress {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_str(&self.to_string())
            }
        }
        impl IotaJsonArg for u64 {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_bcs_bytes(
                    Some(&iota_json::MoveTypeLayout::U64),
                    &bcs::to_bytes(self)?,
                )
            }
        }
        impl IotaJsonArg for Vec<u8> {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_bcs_bytes(None, &self)
            }
        }
        impl IotaJsonArg for &[u8] {
            fn to_iota_json(&self) -> anyhow::Result<IotaJsonValue> {
                IotaJsonValue::from_bcs_bytes(None, self)
            }
        }
        $value.to_iota_json()
    }};
}

#[macro_export]
macro_rules! type_args {
    ($($value:expr), *) => {{
        use iota_json_rpc_types::IotaTypeTag;
        use iota_types::TypeTag;
        trait IotaJsonTypeArg {
            fn to_iota_json(&self) -> anyhow::Result<IotaTypeTag>;
        }
        impl <T: core::fmt::Display> IotaJsonTypeArg for T {
            fn to_iota_json(&self) -> anyhow::Result<IotaTypeTag> {
                Ok(iota_types::parse_iota_type_tag(&self.to_string())?.into())
            }
        }
        Ok::<_, anyhow::Error>(vec![$($value.to_iota_json()?,)*])
    }};
    }
