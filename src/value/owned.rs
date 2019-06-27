/// A lifetime less DOM implementation. It uses strings to make te
/// structure fully owned, avoiding lifetimes at the cost of performance.
mod cmp;
mod from;
mod serialize;

use crate::value::{ValueTrait, ValueType};
use crate::{stry, unlikely, Deserializer, ErrorType, Result};
use halfbrown::HashMap;
use std::fmt;
use std::ops::Index;

pub type Map = HashMap<String, Value>;

/// Parses a slice of bytes into a Value dom. This function will
/// rewrite the slice to de-escape strings.
/// We do not keep any references to the raw data but re-allocate
/// owned memory whereever required thus returning a value without
/// a lifetime.
pub fn to_value(s: &mut [u8]) -> Result<Value> {
    let de = stry!(Deserializer::from_slice(s));
    OwnedDeserializer::from_deserializer(de).parse()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Array(Vec<Value>),
    Object(Map),
}

impl ValueTrait for Value {
    type Map = Map;
    type Array = Vec<Value>;

    fn get(&self, k: &str) -> Option<&Value> {
        match self {
            Value::Object(m) => m.get(k),
            _ => None,
        }
    }

    fn get_mut(&mut self, k: &str) -> Option<&mut Value> {
        match self {
            Value::Object(m) => m.get_mut(k),
            _ => None,
        }
    }

    fn kind(&self) -> ValueType {
        match self {
            Value::Null => ValueType::Null,
            Value::Bool(_) => ValueType::Bool,
            Value::F64(_) => ValueType::F64,
            Value::I64(_) => ValueType::I64,
            Value::String(_) => ValueType::String,
            Value::Array(_) => ValueType::Array,
            Value::Object(_) => ValueType::Object,
        }
    }

    fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(i) => Some(*i),
            _ => None,
        }
    }

    fn as_u64(&self) -> Option<u64> {
        match self {
            Value::I64(i) if *i >= 0 => Some(*i as u64),
            _ => None,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(i) => Some(*i),
            _ => None,
        }
    }

    fn cast_f64(&self) -> Option<f64> {
        match self {
            Value::F64(i) => Some(*i),
            Value::I64(i) => Some(*i as f64),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.to_string()),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    fn as_object(&self) -> Option<&Map> {
        match self {
            Value::Object(m) => Some(m),
            _ => None,
        }
    }
    fn as_object_mut(&mut self) -> Option<&mut Self::Map> {
        match self {
            Value::Object(m) => Some(m),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::Bool(false) => f.write_str("false"),
            Value::Bool(true) => f.write_str("true"),
            Value::I64(n) => f.write_str(&n.to_string()),
            Value::F64(n) => f.write_str(&n.to_string()),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => write!(f, "{:?}", a),
            Value::Object(o) => write!(f, "{:?}", o),
        }
    }
}

impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, index: &str) -> &Value {
        static NULL: Value = Value::Null;
        self.get(index).unwrap_or(&NULL)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

struct OwnedDeserializer<'de> {
    de: Deserializer<'de>,
}

impl<'de> OwnedDeserializer<'de> {
    pub fn from_deserializer(de: Deserializer<'de>) -> Self {
        Self { de }
    }
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub fn parse(&mut self) -> Result<Value> {
        match self.de.next_() {
            b'"' => self.de.parse_str_().map(Value::from),
            b'n' => Ok(Value::Null),
            b't' => Ok(Value::Bool(true)),
            b'f' => Ok(Value::Bool(false)),
            b'-' => self.de.parse_number_root(true).map(Value::from),
            b'0'..=b'9' => self.de.parse_number_root(false).map(Value::from),
            b'[' => self.parse_array(),
            b'{' => self.parse_map(),
            _c => Err(self.de.error(ErrorType::UnexpectedCharacter)),
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn parse_value(&mut self) -> Result<Value> {
        match self.de.next_() {
            b'"' => self.de.parse_str_().map(Value::from),
            b'n' => Ok(Value::Null),
            b't' => Ok(Value::Bool(true)),
            b'f' => Ok(Value::Bool(false)),
            b'-' => self.de.parse_number(true).map(Value::from),
            b'0'..=b'9' => self.de.parse_number(false).map(Value::from),
            b'[' => self.parse_array(),
            b'{' => self.parse_map(),
            _c => Err(self.de.error(ErrorType::UnexpectedCharacter)),
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn parse_array(&mut self) -> Result<Value> {
        let es = self.de.count_elements();
        if unlikely!(es == 0) {
            self.de.skip();
            return Ok(Value::Array(Vec::new()));
        }
        let mut res = Vec::with_capacity(es);

        for _i in 0..es {
            res.push(stry!(self.parse_value()));
            self.de.skip();
        }
        Ok(Value::Array(res))
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn parse_map(&mut self) -> Result<Value> {
        // We short cut for empty arrays
        let es = self.de.count_elements();

        if unlikely!(es == 0) {
            self.de.skip();
            return Ok(Value::Object(Map::new()));
        }

        let mut res = Map::with_capacity(es);

        // Since we checked if it's empty we know that we at least have one
        // element so we eat this

        for _ in 0..es {
            self.de.skip();
            let key = stry!(self.de.parse_str_());
            // We have to call parse short str twice since parse_short_str
            // does not move the cursor forward
            self.de.skip();
            res.insert_nocheck(key.into(), stry!(self.parse_value()));
            self.de.skip();
        }
        Ok(Value::Object(res))
    }
}
