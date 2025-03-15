use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
    Json(JsonValue),
}

impl Value {
    // Optionally, you can add utility methods to work with values.
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    // Get the float value, if it's a Float
    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    // Get the array value, if it's an Array
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    // Get the object value, if it's an Object
    pub fn as_object(&self) -> Option<&std::collections::HashMap<String, Value>> {
        if let Value::Object(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    // Get the JSON value, if it's a Json
    pub fn as_json(&self) -> Option<&JsonValue> {
        if let Value::Json(json) = self {
            Some(json)
        } else {
            None
        }
    }

    // Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    // Check if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    // Check if the value is an integer
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    // Check if the value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    // Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    // Check if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    // Check if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    // Check if the value is a JSON value
    pub fn is_json(&self) -> bool {
        matches!(self, Value::Json(_))
    }
}