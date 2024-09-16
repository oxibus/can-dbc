#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::message::MessageId;

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Node(pub Vec<String>);

impl DBCString for Node {
    fn dbc_string(&self) -> String {
        return format!("BU_: {}",
            self.0.clone().join(" ")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AccessNode {
    AccessNodeVectorXXX,
    AccessNodeName(String),
}

impl DBCString for AccessNode {
    fn dbc_string(&self) -> String {
        return match self {
            Self::AccessNodeName(s) => s.to_string(),
            Self::AccessNodeVectorXXX => "Vector__XXX".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AccessType {
    DummyNodeVector0,
    DummyNodeVector1,
    DummyNodeVector2,
    DummyNodeVector3,
}

impl DBCString for AccessType {
    fn dbc_string(&self) -> String {
        return format!("DUMMY_NODE_VECTOR{}",
          match self {
            Self::DummyNodeVector0 => "0",
            Self::DummyNodeVector1 => "1",
            Self::DummyNodeVector2 => "2",
            Self::DummyNodeVector3 => "3",
          }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Transmitter {
    /// node transmitting the message
    NodeName(String),
    /// message has no sender
    VectorXXX,
}

impl DBCString for Transmitter {
    fn dbc_string(&self) -> String {
        return match self {
            Self::NodeName(s) => s.to_string(),
            Self::VectorXXX => "Vector__XXX".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MessageTransmitter {
    pub (crate) message_id: MessageId,
    pub (crate) transmitter: Vec<Transmitter>,
}

impl DBCString for MessageTransmitter {
    fn dbc_string(&self) -> String {
        return format!("BO_TX_BU_ {} : {}",
          self.message_id.dbc_string(),
          self.transmitter
            .clone()
            .into_iter()
            .map(|t| t.dbc_string())
            .collect::<Vec<String>>()
            .join(","),
            // TODO determine if it will be a problem to kick out Vector__XXX if no transmitter is defined
        )
    }
}
