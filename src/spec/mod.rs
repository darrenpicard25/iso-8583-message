use self::iso_map::IsoSpecMap;

pub mod content_type;
pub mod data_element;
pub mod iso_map;
pub mod length;
pub mod redaction_level;

pub struct IsoSpec<'a> {
    pub name: &'a str,
    pub version: u16,
    pub map: IsoSpecMap<'a>,
}
