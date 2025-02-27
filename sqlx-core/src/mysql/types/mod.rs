use crate::mysql::protocol::Type;
use crate::mysql::MySql;
use crate::types::HasTypeMetadata;

mod bool;
mod float;
mod int;
mod str;
mod uint;

#[cfg(feature = "chrono")]
mod chrono;

#[derive(Default, Debug)]
pub struct MySqlTypeMetadata {
    pub(crate) r#type: Type,
    pub(crate) flag: u8, // 0 or 0x80 for unsigned
}

impl MySqlTypeMetadata {
    pub(crate) fn new(r#type: Type) -> Self {
        Self { r#type, flag: 0 }
    }

    pub(crate) fn unsigned(r#type: Type) -> Self {
        Self { r#type, flag: 0x80 }
    }
}

impl HasTypeMetadata for MySql {
    type TypeMetadata = MySqlTypeMetadata;

    type TableId = Box<str>;

    type TypeId = u8;
}

impl PartialEq<u8> for MySqlTypeMetadata {
    fn eq(&self, other: &u8) -> bool {
        &self.r#type.0 == other
    }
}
