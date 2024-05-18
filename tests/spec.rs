use std::{fs::File, io::BufReader};

use iso_8583_message::spec_builder::IsoSpecBuilder;
use serde_json;

#[test]
fn is_able_to_derive_1987_spec() {
    let file = include_str!("../specs/mastercard-authorization-1987-spec.json");
    let spec: IsoSpecBuilder = serde_json::from_str(file).unwrap();

    dbg!(spec);

    assert_eq!(true, true);
}
