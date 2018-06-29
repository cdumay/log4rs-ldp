use log4rs_gelf::builder::Builder;
use log4rs_gelf::encode::gelf::GelfEncoder;
use log4rs_gelf::error::Error;
use serde_json::{self, Map, Value};
use std::error::Error as stdError;

pub struct OvhGelfEncoderBuilder {
    additionnal_fields: Map<String, Value>,
    null_character: bool,
    ovh_token: Option<String>,
}

impl OvhGelfEncoderBuilder {
    pub fn format_field(name: &str, value: Value) -> Result<(String, Value), Error> {
        match value {
            Value::Null => Err(Error::InvalidConfiguration("No value set".to_string())),
            Value::Bool(v) => Ok(
                (format!("_{}_bool", name), match v {
                    true => Value::from("true"),
                    false => Value::from("false")
                })
            ),
            Value::Number(ref v) => Ok(match v.is_f64() {
                true => (format!("_{}_float", name), value.clone()),
                false => (format!("_{}_int", name), value.clone()),
            }),
            Value::String(ref _v) => Ok((format!("_{}", name), value.clone())),
            Value::Array(ref v) => match serde_json::to_string(v) {
                Ok(data) => Ok((format!("_{}", name), Value::from(data))),
                Err(error) => Ok((format!("_{}", name), Value::from(error.description()))),
            },
            Value::Object(ref v) => match serde_json::to_string(v) {
                Ok(data) => Ok((format!("_{}", name), Value::from(data))),
                Err(error) => Ok((format!("_{}", name), Value::from(error.description()))),
            },
        }
    }

    pub fn add_field(mut self, name: &str, value: Value) -> OvhGelfEncoderBuilder {
        match Self::format_field(name, value) {
            Ok((k, v)) => { self.additionnal_fields.insert(k, v); }
            Err(error) => { println!("Error: {}", error); }
        }
        self
    }
    pub fn null_character(mut self, null_character: bool) -> OvhGelfEncoderBuilder {
        self.null_character = null_character;
        self
    }
    pub fn ovh_token(mut self, ovh_token: &str) -> OvhGelfEncoderBuilder {
        self.ovh_token = Some(String::from(ovh_token));
        self
    }
}

impl Builder for OvhGelfEncoderBuilder {
    type TargetItem = GelfEncoder;

    fn new() -> OvhGelfEncoderBuilder { OvhGelfEncoderBuilder { null_character: false, additionnal_fields: Map::new(), ovh_token: None } }

    fn build(self) -> Result<GelfEncoder, Error> {
        match self.ovh_token {
            Some(ovh_token) => Ok({
                let mut fields = self.additionnal_fields.clone();
                if let Ok((k, v)) = Self::format_field("X-OVH-TOKEN", Value::from(ovh_token)) {
                    fields.insert(k, v);
                }
                GelfEncoder::new(self.null_character, fields)
            }),
            None => Err(Error::InvalidConfiguration("No X-OVH-TOKEN set!".to_string()))
        }
    }
}