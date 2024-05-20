use iso_8583_message::IsoSpec;

#[test]
fn is_able_to_derive_1987_spec() {
    let spec = IsoSpec::mastercard_auth_1987_spec();

    assert_eq!(spec.version, 1);
}
