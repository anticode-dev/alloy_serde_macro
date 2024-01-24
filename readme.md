![CI](https://github.com/anticode-dev/alloy_serde_macro/actions/workflows/rust.yml/badge.svg?branch=main)
![Crates.io](https://img.shields.io/crates/v/alloy_serde_macro)

# Alloy-Serde (De)Serialization Macros

## What is this crate about?
Alloy types are great, but sometimes the implementation requires to send the type as an integer or a string and because alloy by default serialises into hex, it is a good idea to override the default serialisation in those cases.

And this is why this crate exists.

It allows for working with native sol types while ensuring they are being deserialised into or serialised from non-hex types.

## Usage Example

In this case, the backend json rpc server accepts some values as integers, but by default, they would be serialised into hex. This crate contains a macro that generates and serialiser or deserialiser in the format "$target_type_as_$source_type" for serialization and "$target_type_from_$source_type" for deserialization.

Example below, taken from the awesome aori sdk.

```rust
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};
use alloy_serde_macro::*;

sol!(
    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriOrder {
        address offerer;
        // input
        address inputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 inputChainId;
        address inputZone;
        // output
        address outputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 outputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 outputChainId;
        address outputZone;
        // other
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 startTime;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 endTime;
        uint256 salt;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 counter;
        bool toWithdraw;
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use alloy_sol_types::SolValue;
    use alloy_primitives::U256;
    use alloy_primitives::Address;
    use alloy_primitives::keccak256;
    #[test]
    fn aori_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001"
                .parse::<Address>()
                .unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002"
                .parse::<Address>()
                .unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003"
                .parse::<Address>()
                .unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004"
                .parse::<Address>()
                .unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000"
                .parse::<Address>()
                .unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };
        let packed = order.abi_encode_packed();
        let hash = keccak256(packed);
        println!("rust order hash {}", hash);
    }

    #[test]
    fn serialize_aori_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000".parse::<Address>().unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };

        let serialized = serde_json::to_string(&order).unwrap();
        println!("Serialized AoriOrder: {}", serialized);

        // Deserialize the order
        let deserialized: AoriOrder = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized AoriOrder: {:?}", deserialized);
    }
}

```