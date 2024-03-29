use alloy_primitives::aliases::*;
use serde::{ser::Error, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub mod bytes;

pub use bytes::{bytes_as_string, bytes_from_string};

pub mod macros;

// exports serialize
alloy_serde_serialize!(U0, usize);
alloy_serde_serialize!(U0, u8);
alloy_serde_serialize!(U0, u16);
alloy_serde_serialize!(U0, u32);
alloy_serde_serialize!(U0, u64);
alloy_serde_serialize!(U0, u128);
alloy_serde_serialize!(U0, String);
alloy_serde_serialize!(U1, usize);
alloy_serde_serialize!(U1, u8);
alloy_serde_serialize!(U1, u16);
alloy_serde_serialize!(U1, u32);
alloy_serde_serialize!(U1, u64);
alloy_serde_serialize!(U1, u128);
alloy_serde_serialize!(U1, String);
alloy_serde_serialize!(U8, usize);
alloy_serde_serialize!(U8, u8);
alloy_serde_serialize!(U8, u16);
alloy_serde_serialize!(U8, u32);
alloy_serde_serialize!(U8, u64);
alloy_serde_serialize!(U8, u128);
alloy_serde_serialize!(U8, String);
alloy_serde_serialize!(U16, usize);
alloy_serde_serialize!(U16, u8);
alloy_serde_serialize!(U16, u16);
alloy_serde_serialize!(U16, u32);
alloy_serde_serialize!(U16, u64);
alloy_serde_serialize!(U16, u128);
alloy_serde_serialize!(U16, String);
alloy_serde_serialize!(U32, usize);
alloy_serde_serialize!(U32, u8);
alloy_serde_serialize!(U32, u16);
alloy_serde_serialize!(U32, u32);
alloy_serde_serialize!(U32, u64);
alloy_serde_serialize!(U32, u128);
alloy_serde_serialize!(U32, String);
alloy_serde_serialize!(U64, usize);
alloy_serde_serialize!(U64, u8);
alloy_serde_serialize!(U64, u16);
alloy_serde_serialize!(U64, u32);
alloy_serde_serialize!(U64, u64);
alloy_serde_serialize!(U64, u128);
alloy_serde_serialize!(U64, String);
alloy_serde_serialize!(U128, usize);
alloy_serde_serialize!(U128, u8);
alloy_serde_serialize!(U128, u16);
alloy_serde_serialize!(U128, u32);
alloy_serde_serialize!(U128, u64);
alloy_serde_serialize!(U128, u128);
alloy_serde_serialize!(U128, String);
alloy_serde_serialize!(U256, usize);
alloy_serde_serialize!(U256, u8);
alloy_serde_serialize!(U256, u16);
alloy_serde_serialize!(U256, u32);
alloy_serde_serialize!(U256, u64);
alloy_serde_serialize!(U256, u128);
alloy_serde_serialize!(U256, String);

// export deserialize
alloy_serde_deserialize!(U0, usize);
alloy_serde_deserialize!(U0, u8);
alloy_serde_deserialize!(U0, u16);
alloy_serde_deserialize!(U0, u32);
alloy_serde_deserialize!(U0, u64);
alloy_serde_deserialize!(U0, u128);
alloy_serde_deserialize!(U0, String);
alloy_serde_deserialize!(U1, usize);
alloy_serde_deserialize!(U1, u8);
alloy_serde_deserialize!(U1, u16);
alloy_serde_deserialize!(U1, u32);
alloy_serde_deserialize!(U1, u64);
alloy_serde_deserialize!(U1, u128);
alloy_serde_deserialize!(U1, String);
alloy_serde_deserialize!(U8, usize);
alloy_serde_deserialize!(U8, u8);
alloy_serde_deserialize!(U8, u16);
alloy_serde_deserialize!(U8, u32);
alloy_serde_deserialize!(U8, u64);
alloy_serde_deserialize!(U8, u128);
alloy_serde_deserialize!(U8, String);
alloy_serde_deserialize!(U16, usize);
alloy_serde_deserialize!(U16, u8);
alloy_serde_deserialize!(U16, u16);
alloy_serde_deserialize!(U16, u32);
alloy_serde_deserialize!(U16, u64);
alloy_serde_deserialize!(U16, u128);
alloy_serde_deserialize!(U16, String);
alloy_serde_deserialize!(U32, usize);
alloy_serde_deserialize!(U32, u8);
alloy_serde_deserialize!(U32, u16);
alloy_serde_deserialize!(U32, u32);
alloy_serde_deserialize!(U32, u64);
alloy_serde_deserialize!(U32, u128);
alloy_serde_deserialize!(U32, String);
alloy_serde_deserialize!(U64, usize);
alloy_serde_deserialize!(U64, u8);
alloy_serde_deserialize!(U64, u16);
alloy_serde_deserialize!(U64, u32);
alloy_serde_deserialize!(U64, u64);
alloy_serde_deserialize!(U64, u128);
alloy_serde_deserialize!(U64, String);
alloy_serde_deserialize!(U128, usize);
alloy_serde_deserialize!(U128, u8);
alloy_serde_deserialize!(U128, u16);
alloy_serde_deserialize!(U128, u32);
alloy_serde_deserialize!(U128, u64);
alloy_serde_deserialize!(U128, u128);
alloy_serde_deserialize!(U128, String);

alloy_serde_deserialize!(U256, usize);
alloy_serde_deserialize!(U256, u8);
alloy_serde_deserialize!(U256, u16);
alloy_serde_deserialize!(U256, u32);
alloy_serde_deserialize!(U256, u64);
alloy_serde_deserialize!(U256, u128);
alloy_serde_deserialize!(U256, String);
