use hex;
use serde::{de, de::Visitor, Deserializer, Serializer};
use std::fmt;

pub fn bytes_as_string<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = format!("0x{}", hex::encode(bytes));
    serializer.serialize_str(&hex_string)
}

pub fn bytes_from_string<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct BytesVisitor;

    impl<'de> Visitor<'de> for BytesVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a hex string with a '0x' prefix")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if let Some(stripped) = value.strip_prefix("0x") {
                hex::decode(stripped).map_err(de::Error::custom)
            } else {
                Err(de::Error::custom("missing '0x' prefix"))
            }
        }
    }

    deserializer.deserialize_str(BytesVisitor)
}
