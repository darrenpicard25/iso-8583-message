use std::sync::OnceLock;

use self::iso_map::IsoSpecMap;

pub mod content_type;
pub mod data_element;
mod deserialize;
pub mod iso_map;
pub mod length;
pub mod redaction_level;

const MASTERCARD_AUTHORIZATION_1987_SPEC_JSON: &'static str =
    include_str!("../../specs/mastercard-authorization-1987-spec.json");
static MASTERCARD_AUTHORIZATION_1987_SPEC: OnceLock<IsoSpec<'static>> = OnceLock::new();

#[derive(Debug)]
pub struct IsoSpec<'a> {
    pub name: &'a str,
    pub version: u16,
    pub map: IsoSpecMap<'a>,
}

impl<'a> IsoSpec<'a> {
    pub fn mastercard_auth_1987_spec() -> &'static IsoSpec<'a> {
        MASTERCARD_AUTHORIZATION_1987_SPEC.get_or_init(|| {
            serde_json::from_str(MASTERCARD_AUTHORIZATION_1987_SPEC_JSON)
                .expect("Error deserializing Auth spec")
        })
    }
}

impl<'a> TryFrom<&'a str> for IsoSpec<'a> {
    type Error = serde_json::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}
