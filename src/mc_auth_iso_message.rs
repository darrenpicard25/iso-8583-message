use std::collections::BTreeMap;

use crate::{iso_message::IsoMessage, IsoSpec};

pub struct McAuthIsoMessage;

impl McAuthIsoMessage {
    pub fn new(data: BTreeMap<String, String>) -> IsoMessage<'static> {
        IsoMessage::new(IsoSpec::mastercard_auth_1987_spec(), data)
    }
}
