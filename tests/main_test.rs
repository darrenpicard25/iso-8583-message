use std::fs;

use iso_8583_message::IsoMessage;

#[test]
fn end_to_end() {
    let buffer = fs::read("./sample_messages/i2c-authorization-request.bin").unwrap();

    let message = IsoMessage::from_buffer(buffer.clone()).unwrap();

    let response_buffer = message.get_message_buffer().unwrap();
    assert_eq!(buffer, response_buffer);
}
