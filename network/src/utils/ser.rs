/// serde related operations
use ipnet::IpNet;
use serde::ser::{SerializeSeq, Serializer, Error as SerErr};
use std::string::ToString;
use regex::Regex;
use std::fmt::Display;

pub fn to_string<T: ToString, S: Serializer>(data: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&data.to_string())
}

pub fn ipver_human<S: Serializer>(family: &i32, serializer: S) -> Result<S::Ok, S::Error> {
    if *family == 2 {
        serializer.serialize_i32(4)
    } else if *family == 10 {
        serializer.serialize_i32(6)
    } else {
        Err(SerErr::custom("Only v4 or v6 family is valid"))
    }
}

/// Combine mulitiple addresses into lists of string
pub fn addrs_to_string<S: Serializer>(
    addreses: &Vec<IpNet>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let len = addreses.len();
    let mut seq = serializer.serialize_seq(Some(len))?;
    for ip_net in addreses {
        seq.serialize_element(&ip_net.to_string())?;
    }
    seq.end()
}


/// Print NetworkManager
pub fn nm_display<T: Display>(item: T) -> String {
    let res = format!("{}", item);
    Regex::new(r"\w+::").and_then(|x| {
        Ok(x.replace_all(&res, "").to_string())
    }).unwrap()
}
