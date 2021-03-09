use crate::types::*;
use serde::de::{Deserialize, Deserializer, Error};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MyChatMember {}

impl<'de> Deserialize<'de> for MyChatMember {
    fn deserialize<D>(deserializer: D) -> Result<MyChatMember, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MyChatMember {})
    }
}
