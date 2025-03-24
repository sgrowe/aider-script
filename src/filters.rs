use convert_case::{Case, Casing};
use std::collections::HashMap;
use tera::{Filter, Result, Value};

/// Filter that converts a string to kebab-case
pub struct KebabFilter;

impl Filter for KebabFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_case(Case::Kebab))),
            None => Ok(value.clone()),
        }
    }
}

/// Filter that converts a string to PascalCase
pub struct PascalFilter;

impl Filter for PascalFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_case(Case::Pascal))),
            None => Ok(value.clone()),
        }
    }
}

/// Filter that converts a string to camelCase
pub struct CamelFilter;

impl Filter for CamelFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_case(Case::Camel))),
            None => Ok(value.clone()),
        }
    }
}

/// Filter that converts a string to snake_case
pub struct SnakeFilter;

impl Filter for SnakeFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_case(Case::Snake))),
            None => Ok(value.clone()),
        }
    }
}
