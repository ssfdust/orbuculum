//! The rules parser library
//!
//! The library is to run the target rule script with the given serde_json::Value
//! with the power of rhai script language.
use std::path::Path;

use eyre::{bail, Result};
use rhai::{Dynamic, Engine, Scope};
use serde_json::Value;

/// The parser for handling with the only one serde_json::Value argument, which
/// only returns one serde_json::Value.
struct SerdeRhaiParser<'a> {
    script: &'a str,
    function: &'a str,
    engine: Engine,
    args: Value,
}

impl<'a> SerdeRhaiParser<'a> {
    fn new(script: &'a str, function: &'a str, args: &Value) -> Self {
        Self {
            script,
            function,
            engine: Engine::new(),
            args: args.clone(),
        }
    }

    /// Run the script with the args
    fn parser(&self) -> Result<Value> {
        let ast = self.engine.compile(self.script)?;
        let mut scope = Scope::new();
        let devices = self.args.clone();
        let dynamic_args: Dynamic = serde_json::from_value(devices)?;
        match self
            .engine
            .call_fn::<Dynamic>(&mut scope, &ast, self.function, (dynamic_args,))
        {
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
/// The script must contains `get_nic_type_ord` function, which takes only one
/// argument and returns a number for device type sorting.
pub fn get_nic_ord_types<'a>(script_path: &'a str, devices: &'a Value) -> Result<i32> {
    let script_path = Path::new(script_path);
    let script = std::fs::read_to_string(&script_path)?;
    let parser = SerdeRhaiParser::new(&script, "get_nic_type_ord", devices);
    let ret = parser.parser()?;
    if let Some(ord_type) = ret.as_i64() {
        Ok(ord_type as i32)
    } else {
        bail!("Failed to get network device type ord.")
    }
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
        let rule_parser = SerdeRhaiParser::new(script, "get_test_type", &nic_rules);
        rule_parser.parser().unwrap();
    }
}
