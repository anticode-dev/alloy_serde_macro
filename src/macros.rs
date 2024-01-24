#![allow(non_snake_case)]

#[macro_export]
macro_rules! alloy_serde_serialize {
    ($src_type:ty, String) => {
        paste::item! {
            #[allow(non_snake_case)]
            pub fn [<$src_type _as_String>]<S>(value: &$src_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                $src_type: ToString,
            {
                let value_string = value.to_string();
                serializer.serialize_str(&value_string)
            }
        }
    };
    ($src_type:ty, $target_type:ident) => {
        paste::item! {
            #[allow(non_snake_case)]
            pub fn [<$src_type _as_ $target_type>]<S>(value: &$src_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                $src_type: TryInto<$target_type>,
                <$src_type as TryInto<$target_type>>::Error: std::fmt::Display,
            {
                let value_target: Result<$target_type, _> = (*value).try_into();
                match value_target {
                    Ok(v) => serializer.serialize_newtype_struct(stringify!($target_type), &v),
                    Err(e) => Err(S::Error::custom(format!("Cannot convert to {}: {}", stringify!($target_type), e))),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! alloy_serde_deserialize {
    ($target_type:ty, String) => {
        paste::item! {
            #[allow(non_snake_case)]
            pub fn [<$target_type _from_String>]<'de, D>(deserializer: D) -> Result<$target_type, D::Error>
            where
                D: Deserializer<'de>,
                $target_type: FromStr,
                <$target_type as FromStr>::Err: std::fmt::Display,
            {
                let value_string = String::deserialize(deserializer)?;
                $target_type::from_str(&value_string).map_err(serde::de::Error::custom)
            }
        }
    };
    ($target_type:ty, $src_type:ident) => {
        paste::item! {
            #[allow(non_snake_case)]
            pub fn [<$target_type _from_ $src_type>]<'de, D>(deserializer: D) -> Result<$target_type, D::Error>
            where
                D: Deserializer<'de>,
                $src_type: Deserialize<'de>,
            {
                let value_src = $src_type::deserialize(deserializer)?;
                Ok($target_type::from(value_src))
            }
        }
    };
}
