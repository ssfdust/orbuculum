//! The rules parser library
//!
//! The library is to run the target rule script with the given serde_json::Value
//! with the power of rhai script language.
use std::collections::HashMap;
use std::path::Path;

use eyre::{bail, Result};
use rhai::{Dynamic, Engine, Scope, AST};
use serde_json::{json, Value};

/// The parser for handling with the only one serde_json::Value argument, which
/// only returns one serde_json::Value.
struct SerdeRhaiParser<'a> {
    function: &'a str,
    engine: Engine,
    ast: Option<AST>,
}

impl<'a> SerdeRhaiParser<'a> {
    fn new(script: &'a str, function: &'a str) -> Self {
        let engine = Engine::new();
        let ast = engine.compile(script).map(|x| Some(x)).unwrap_or(None);
        Self {
            function,
            engine,
            ast,
        }
    }

    /// Run the script with the only one arg
    fn parser_one_arg(&self, args: &Value) -> Result<Value> {
        let mut scope = Scope::new();
        let args = args.clone();
        let dynamic_args: Dynamic = serde_json::from_value(args)?;
        match self.engine.call_fn::<Dynamic>(
            &mut scope,
            &self.ast.as_ref().expect("Failed to parse script to ast"),
            self.function,
            (dynamic_args,),
        ) {
            Ok(ret) => {
                let devices = serde_json::to_value(ret)?;
                Ok(devices)
            }
            Err(err) => {
                bail!("{}", err.to_string())
            }
        }
    }

    fn parser_two_args(&self, args: &Value, args_one: &str) -> Result<Value> {
        let mut scope = Scope::new();
        let args = args.clone();
        let dynamic_args: Dynamic = serde_json::from_value(args)?;
        match self.engine.call_fn::<Dynamic>(
            &mut scope,
            &self.ast.as_ref().expect("Failed to parse script to ast"),
            self.function,
            (dynamic_args, args_one.to_string()),
        ) {
            Ok(ret) => {
                let devices = serde_json::to_value(ret)?;
                Ok(devices)
            }
            Err(err) => {
                bail!("{}", err.to_string())
            }
        }
    }
}

/// Run the given path to the script.
/// The script must contains `insert_nic_type_ord` function, which takes only one
/// object mapping argument and returns the object mapping which contains the
/// `type_ord` key corresponding to the number type.
pub fn insert_nic_ord_types<'a>(script_path: &str, devices: &'a Value) -> Result<Vec<Value>> {
    // Read script from script path
    let script_path = Path::new(script_path);
    let script = std::fs::read_to_string(&script_path)?;

    let mut new_devices = Value::Array(vec![]);
    let new_devices_arr = new_devices.as_array_mut().unwrap();
    let devices_arr = devices.as_array().unwrap();
    let parser = SerdeRhaiParser::new(&script, "insert_nic_type_ord");
    for device in devices_arr {
        let trimed_device = trim_device_value(device);
        let ret = parser.parser_one_arg(&trimed_device)?;
        new_devices_arr.push(ret);
    }
    Ok(new_devices_arr.to_vec())
}

/// Insert desired connection name into the device information
/// The rules are defined from `modify_connection_names` function.
/// We will group the devices by their device type, and pass the results
/// of each group to the `modify_connection_names` in script. Finally,
/// returns a single array of modified device information.
pub fn insert_device_con_names(script_path: &str, devices: &Vec<Value>) -> Result<Vec<Value>> {
    // Read script from script path
    let script_path = Path::new(script_path);
    let script = std::fs::read_to_string(&script_path)?;

    let mut groups: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    let mut new_devices = Value::Array(vec![]);
    let new_devices_arr = new_devices.as_array_mut().unwrap();

    // Group devices by their device type
    for device in devices {
        if let Some(value) = device.get("device_type") {
            let entry = groups
                .entry(value.as_str().map(|x| x.to_string()).unwrap())
                .or_default();
            entry.push(device.clone());
        }
    }

    // Call modify_connection_names from script
    let parser = SerdeRhaiParser::new(&script, "modify_connections");
    for (key, items) in groups.iter() {
        let json_val = serde_json::to_value(&items)?;
        let ret = parser.parser_two_args(&json_val, key)?;
        let ret_arr = ret.as_array().unwrap();
        for item in ret_arr {
            new_devices_arr.push(item.clone())
        }
    }

    Ok(new_devices_arr.to_vec())
}

pub fn get_desired_devices(script_path: &str, devices: &Value) -> Result<Vec<Value>> {
    if let Some(sorted_devices) = sort_devices(script_path, devices) {
        let devices = insert_device_con_names(script_path, &sorted_devices)?;
        Ok(devices)
    } else {
        bail!("Failed to sort devices with script")
    }
}

/// Drop the useless ip information and convert the null value to empty string
fn trim_device_value(device: &Value) -> Value {
    let mut new_device_map: serde_json::Map<String, Value> = serde_json::Map::new();
    let device_obj = device.as_object().unwrap();
    for key in device_obj.keys() {
        let value = device.get(key).unwrap();
        // insert connection information
        if key == "connection" {
            new_device_map.insert(key.to_owned(), value.clone());
        }
        if !value.is_object() {
            if value.is_null() {
                new_device_map.insert(key.to_owned(), json!(""));
            } else {
                new_device_map.insert(key.to_owned(), value.clone());
            }
        }
    }
    Value::Object(new_device_map)
}

pub fn sort_devices<'a>(nicrule_file: &'a str, devices: &'a Value) -> Option<Vec<Value>> {
    let mut devices_arr = insert_nic_ord_types(&nicrule_file, devices).unwrap();
    devices_arr.sort_by(|device_a, device_b| {
        let device_ord_type_a = device_a["type_ord"].as_i64().unwrap();
        let device_ord_type_b = device_b["type_ord"].as_i64().unwrap();
        if device_ord_type_a != device_ord_type_b {
            device_ord_type_a.cmp(&device_ord_type_b)
        } else {
            let id_path_a = device_a["id_path"].as_str().unwrap_or("");
            let id_path_b = device_b["id_path"].as_str().unwrap_or("");
            id_path_a.cmp(id_path_b)
        }
    });
    Some(devices_arr)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    fn test_nic_rule_parser() {
        let script = r#"
            fn get_test_type(value) {
                print(`${value[0].name}`);
                return 3;
            }
        "#;
        let nic_rules = json!(
            [
                {
                    "name": "test",
                    "device_type": "Ethernet",
                },
                {
                    "name": "test1",
                    "device_type": "Ethernet",
                },
            ]
        );
        let rule_parser = SerdeRhaiParser::new(script, "get_test_type");
        rule_parser.parser_one_arg(&nic_rules).unwrap();
    }
}
