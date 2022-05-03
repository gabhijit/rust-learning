use serde::de::Deserializer;
use serde::de::Visitor;

struct Asn1EnumVisitor<T> {
    extensible: bool,
    lb: Option<u8>,
    ub: Option<u8>,
}

impl<'de> Visitor<'de> for Asn1EnumVisitor<T> {
    type Value = u8;

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let v = deserializer.parse_integer(self.extensible, self.lb, self.ub)?;
        Ok(v)
    }
}

struct AperDeserializer;

impl<'de> Deserializer<'de> for AperDeserializer {}

fn main() {
    println!("Hello, world!");
}
