//! An attribute message for UxAS
//! Original code is in `AddressedAttributedMessage.h`
//! If using rust wihin UxAS build, the addressing and atrributing of serialized LMCP messages
//! is handled internally, but if we are using an external program to communicate with UxAS via
//! a TCP bridge, we have to provide the attributes outselves.
//!
//! From `AddressedAttributedMessage.h`:
//! The message components are the following:
//! 1. address (notional example values: "uxas.project.isolate.IntruderAlert", "eId12sId14", "uxas.roadmonitor")
//! 2. attributes:
//!		a. contentType (e.g., "lmcp", "json", "xml")
//!		b. descriptor (e.g., "afrl.cmasi.AirVehicleState" if contentType="lmcp" or a
//!					   json content descriptor; intent is some flexibility on values depending on contentType)
//!		d. senderGroup (notional example values: "fusion", "fusion.operator.sensor", "uxas", "agent", "uxas.roadmonitor")
//!		e. senderEntityId
//!		f. senderServiceId
//! 3. paylaod (LMCP message itself)
//!
//! Message components consist of 0-N ASCII characters, and are delimited with `$`.
//! Message attributes consist of 0-N ASCII characters, and are delimited with `$`
//! Message payload is a byte stream `[u8]` of arbitrary length.
//! And example of a message is:
//! ```notest
//! 	afrl.cmasi.AirVehicleState$lmcp|afrl.cmasi.AirVehicleState||0|0$LMCP...(payload continues)
//! ```
//! The design intend is to store values internally as `Vec<u8>` and expose them as `String`s only when necessary
//!
extern crate core;
use core::fmt;

#[derive(Debug)]
struct MessageAttributes {
    content_type: Vec<u8>,
    descriptor: Vec<u8>,
    sender_group: Vec<u8>,
    sender_entity_id: Vec<u8>,
    sender_service_id: Vec<u8>,
}

impl MessageAttributes {
    const DELIMITER: char = '|';
    const CHUNKS_LEN: usize = 5;

    /// An arbitrary default header size that should hold all the serializedd attributes
    const DEFAULT_HEADER_SIZE: usize = 50;

    pub fn default() -> MessageAttributes {
        MessageAttributes {
            content_type: vec![],
            descriptor: vec![],
            sender_group: vec![],
            sender_entity_id: vec![],
            sender_service_id: vec![],
        }
    }

    pub fn set_content_type(&mut self, val: &str) {
        self.content_type = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn set_descriptor(&mut self, val: &str) {
        self.descriptor = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn set_sender_group(&mut self, val: &str) {
        self.sender_group = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn set_sender_entity_id(&mut self, val: &str) {
        self.sender_entity_id = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn set_sender_service_id(&mut self, val: &str) {
        self.sender_service_id = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn deserialize(data: &[u8]) -> Option<MessageAttributes> {
        let chunks: Vec<_> = data.split(|b| *b == Self::DELIMITER as u8).collect();
        if chunks.len() != Self::CHUNKS_LEN {
            None
        } else {
            let mut msg = MessageAttributes::default();
            msg.content_type = chunks[0].to_vec();
            msg.descriptor = chunks[1].to_vec();
            msg.sender_group = chunks[2].to_vec();
            msg.sender_entity_id = chunks[3].to_vec();
            msg.sender_service_id = chunks[4].to_vec();
            Some(msg)
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let mut v = Vec::with_capacity(Self::DEFAULT_HEADER_SIZE);
        v.append(&mut self.content_type);
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.descriptor);
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.sender_group);
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.sender_entity_id);
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.sender_service_id);
        v
    }
}

impl fmt::Display for MessageAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.content_type))?;
        write!(f, "{}", Self::DELIMITER)?;
        write!(f, "{}", String::from_utf8_lossy(&self.descriptor))?;
        write!(f, "{}", Self::DELIMITER)?;
        write!(f, "{}", String::from_utf8_lossy(&self.sender_group))?;
        write!(f, "{}", Self::DELIMITER)?;
        write!(f, "{}", String::from_utf8_lossy(&self.sender_entity_id))?;
        write!(f, "{}", Self::DELIMITER)?;
        write!(f, "{}", String::from_utf8_lossy(&self.sender_service_id))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AddressedAttributedMessage {
    address: Vec<u8>,
    attributes: MessageAttributes,
    payload: Vec<u8>,
}

impl AddressedAttributedMessage {
    const DELIMITER: char = '$';
    //const CHUNKS_LEN: usize = 3;

    /// Default expected length of serialized address (somewhat arbitrary indeed)
    const DEFAULT_ADDR_SIZE: usize = 30;

    /// An arbitrary default header size that should hold all the serializedd attributes
    const DEFAULT_HEADER_SIZE: usize =
        MessageAttributes::DEFAULT_HEADER_SIZE + Self::DEFAULT_ADDR_SIZE;

    pub fn default() -> AddressedAttributedMessage {
        AddressedAttributedMessage {
            address: vec![],
            attributes: MessageAttributes::default(),
            payload: vec![],
        }
    }

    /// Return payload of the message
    pub fn get_payload(&self) -> &[u8] {
        self.payload.as_slice()
    }

    /// Get a byte stream representation of the attributed message
    /// The message is consumed.
    pub fn serialize(mut self) -> Vec<u8> {
        let mut v = Vec::with_capacity(Self::DEFAULT_HEADER_SIZE + self.payload.len());
        v.append(&mut self.address);
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.attributes.serialize());
        v.push(Self::DELIMITER as u8);
        v.append(&mut self.payload);
        v
    }

    /// Deserialize a message from a byte stream
    /// A typical vector looks like this:
    /// "afrl.cmasi.AirVehicleState$lmcp|afrl.cmasi.AirVehicleState||1|2$LMCPthisisthepayloadhere"
    pub fn deserialize(mut data: Vec<u8>) -> Option<AddressedAttributedMessage> {
        let mut msg = AddressedAttributedMessage::default();

        // Get address
        for idx in 0..data.len() {
            if data[idx] == Self::DELIMITER as u8 {
                msg.address = data.drain(..idx).collect();
                data.remove(0); // remove '$'
                break;
            }
        }

        // Get attributes
        for idx in 0..data.len() {
            if data[idx] == Self::DELIMITER as u8 {
                let attributes: Vec<_> = data.drain(..idx).collect();
                data.remove(0); // remove '$'
                match MessageAttributes::deserialize(&attributes) {
                    Some(attrs) => {
                        msg.attributes = attrs;
                        break;
                    }
                    None => {
                        return None;
                    }
                }
            }
        }

        msg.set_payload(data);
        Some(msg)
    }

    pub fn set_address(&mut self, val: &str) {
        self.address = {
            let mut v = Vec::with_capacity(val.len());
            v.extend_from_slice(val.as_bytes());
            v
        };
    }

    pub fn set_payload(&mut self, val: Vec<u8>) {
        self.payload = val;
    }

    pub fn set_content_type(&mut self, val: &str) {
        self.attributes.set_content_type(val);
    }

    pub fn set_descriptor(&mut self, val: &str) {
        self.attributes.set_descriptor(val);
    }

    pub fn set_sender_group(&mut self, val: &str) {
        self.attributes.set_sender_group(val);
    }

    pub fn set_sender_entity_id(&mut self, val: &str) {
        self.attributes.set_sender_entity_id(val);
    }

    pub fn set_sender_service_id(&mut self, val: &str) {
        self.attributes.set_sender_service_id(val);
    }
}

impl fmt::Display for AddressedAttributedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.address))?;
        write!(f, "{}", Self::DELIMITER)?;
        write!(f, "{}", self.attributes)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str =
        "afrl.cmasi.AirVehicleState$lmcp|afrl.cmasi.AirVehicleState||1|2$LMCPthisisthepayloadhereblabla$sads$";

    #[test]
    fn test_serialize() {
        let mut msg = AddressedAttributedMessage::default();
        msg.set_address("afrl.cmasi.AirVehicleState");
        msg.set_content_type("lmcp");
        msg.set_descriptor("afrl.cmasi.AirVehicleState");
        msg.set_sender_entity_id("1");
        msg.set_sender_service_id("2");
        msg.set_payload("LMCPthisisthepayloadhereblabla$sads$".as_bytes().to_vec());
        let s1 = msg.serialize();
        let s2 = TEST_DATA.to_string().as_bytes().to_vec();
        println!("s1={}", String::from_utf8(s1.clone()).unwrap());
        println!("s2={}", TEST_DATA);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_deserialize() {
        let data = TEST_DATA.to_string().as_bytes().to_vec();
        let msg = AddressedAttributedMessage::deserialize(data).unwrap();
        println!("msg = {}", msg);
        let s1 = msg.serialize();
        let s2 = TEST_DATA.to_string().as_bytes().to_vec();
        println!("s1={}", String::from_utf8(s1.clone()).unwrap());
        println!("s2={}", TEST_DATA);
        assert_eq!(s1, s2);
    }

}
