/*
{"0":"0800","2":"40013","7":"1009185821","11":"006394","33":"031537","70":"270"}
*/

use iso_8583_message::IsoMessage;

#[test]
fn network_message() {
    let hex_string = "30383130c2200000820000020400000000000000303534303031333130303931393030323130303633393830363033313533373030303132564953303030303232313432323730";
    let buffer = hex::decode(hex_string).unwrap();

    let iso_message = IsoMessage::from_buffer(buffer.clone()).unwrap();

    let new_buffer = iso_message.get_message_buffer().unwrap();

    assert_eq!(buffer, new_buffer);
}
