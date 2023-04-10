use ipnet::IpNet;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;

pub fn ipnet_from_string<'de, D>(deserializer: D) -> Result<Vec<IpNet>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<IpNet>>);
    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<IpNet>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let addr = value.parse().unwrap();
            Ok(vec![addr])
        }
        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let values: Vec<String> =
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor)).unwrap();
            Ok(values.iter().map(|x| x.parse().unwrap()).collect())
        }
    }
    deserializer.deserialize_any(StringOrVec(PhantomData))
}
