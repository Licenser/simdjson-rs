use crate::{Number, ErrorType, Deserializer, Error, Result, stry};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;
use std::fmt;

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            structural: 0,
            index: 0,
            character: '💩', //this is the poop emoji
            error: ErrorType::Serde(msg.to_string()),
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de>
{
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    #[cfg_attr(feature = "inline", inline(always))]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match stry!(self.next()) {
            b'n' => {
                stry!(self.parse_null_());
                visitor.visit_unit()
            }
            b't' => visitor.visit_bool(stry!(self.parse_true_())),
            b'f' => visitor.visit_bool(stry!(self.parse_false_())),
            b'-' => match stry!(self.parse_number_(true)) {
                Number::F64(n) => visitor.visit_f64(n),
                Number::I64(n) => visitor.visit_i64(n),
            },
            b'0'...b'9' => match stry!(self.parse_number_(false)) {
                Number::F64(n) => visitor.visit_f64(n),
                Number::I64(n) => visitor.visit_i64(n),
            },
            b'"' => {
                if let Some(next) = self.structural_indexes.get(self.idx + 1) {
                    if *next as usize - self.iidx < 32 {
                        return visitor.visit_borrowed_str(stry!(self.parse_short_str_()))
                    }
                }
                visitor.visit_borrowed_str(stry!(self.parse_str_()))
            },

            b'[' => visitor.visit_seq(CommaSeparated::new(&mut self)),
            b'{' => visitor.visit_map(CommaSeparated::new(&mut self)),
            _c => Err(self.error(ErrorType::UnexpectedCharacter)),
        }
    }
    /*

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.c() != b'"' {
            return Err(ErrorType::ExpectedString);
        }
        visitor.visit_borrowed_str(stry!(self.parse_str_()))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: i64 = stry!(self.parse_signed());
        visitor.visit_i8(v as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: i64 = stry!(self.parse_signed());
        visitor.visit_i16(v as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: i64 = stry!(self.parse_signed());
        visitor.visit_i32(v as i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(stry!(self.parse_signed()))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: u64 = stry!(self.parse_unsigned());
        visitor.visit_u8(v as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: u64 = stry!(self.parse_unsigned());
        visitor.visit_u16(v as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v: u64 = stry!(self.parse_unsigned());
        visitor.visit_u32(v as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(stry!(self.parse_unsigned()))
    }

    */
    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.

    #[cfg_attr(feature = "inline", inline)]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if stry!(self.peek()) == b'n' {
            self.skip();
            stry!(self.parse_null_());
            visitor.visit_unit()
        } else {
            visitor.visit_some(self)
        }
    }

    /*
    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        stry!(self.parse_null());
        visitor.visit_unit()
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!();
        // Parse the opening bracket of the sequence.
        if stry!(self.next()) == b'[' {
            // Give the visitor access to each element of the sequence.
            visitor.visit_seq(CommaSeparated::new(&mut self))
        } else {
            Err(ErrorType::ExpectedArray(self.idx(), self.c() as char))
        }
    }

     */

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.

    #[cfg_attr(feature = "inline", inline)]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let r = self.deserialize_seq(visitor);
        // tuples have a known length damn you serde ...
        self.skip();
        r
    }

    forward_to_deserialize_any! {
        seq  bool i8 i16 i32 i64 u8 u16 u32 u64 string str unit
            i128 u128 f32 f64 char
            bytes byte_buf  unit_struct newtype_struct
            tuple_struct map struct enum identifier ignored_any
    }
}


// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        CommaSeparated { first: true, de }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    #[cfg_attr(feature = "inline", inline)]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {

        let peek = match stry!(self.de.peek()) {
            b']' => {
                self.de.skip();
                return Ok(None);
            }
            b',' if !self.first => stry!(self.de.next()),
            b => {
                if self.first {
                    self.first = false;
                    b
                } else {
                    return Err(self.de.error(ErrorType::ExpectedArrayComma));
                }
            }
        };
        match peek {
            b']' => Err(self.de.error(ErrorType::ExpectedArrayComma)),
            _ => Ok(Some(stry!(seed.deserialize(&mut *self.de)))),
        }
    }
    #[cfg_attr(feature = "inline", inline)]
    fn size_hint(&self) -> Option<usize> {
        Some(self.de.count_elements())
    }
}


// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    #[cfg_attr(feature = "inline", inline)]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {

        let peek = match stry!(self.de.peek()) {
            b'}' => {
                self.de.skip();
                return Ok(None);
            }
            b',' if !self.first => {
                self.de.skip();
                stry!(self.de.peek())
            }
            b => {
                if self.first {
                    self.first = false;
                    b
                } else {
                    return Err(self.de.error(ErrorType::ExpectedArrayComma));
                }
            }
        };

        match peek {
            b'"' => seed.deserialize(&mut *self.de).map(Some),
            b'}' => Err(self.de.error(ErrorType::ExpectedArrayComma)), //Err(self.de.peek_error(ErrorCode::TrailingComma)),
            _ => Err(self.de.error(ErrorType::ExpectedString)), // TODO: Err(self.de.peek_error(ErrorCode::KeyMustBeAString)),
        }
    }

    #[cfg_attr(feature = "inline", inline)]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let c = stry!(self.de.next());
        if c != b':' {
            return Err(self.de.error(ErrorType::ExpectedMapColon));
        }
        seed.deserialize(&mut *self.de)
    }

    #[cfg_attr(feature = "inline", inline)]
    fn size_hint(&self) -> Option<usize> {
        Some(self.de.count_elements())
    }
}
