#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::message::MessageId;

use crate::parser;
use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{multispace0, char, line_ending},
    combinator::{map, opt, value},
    sequence::preceded,
    character::complete::{self, space0},
    multi::separated_list0,
    IResult,
};

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

    fn parse(s: &str) -> nom::IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BU_:")(s)?;
        let (s, li) = opt(preceded(parser::ms1, separated_list0(parser::ms1, parser::c_ident)))(s)?;
        let (s, _) = space0(s)?;
        let (s, _) = line_ending(s)?;
        Ok((s, Node(li.unwrap_or_default())))   
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AccessNode {
    AccessNodeVectorXXX,
    AccessNodeName(String),
}

impl AccessNode {
    fn access_node_vector_xxx(s: &str) -> IResult<&str, AccessNode> {
        value(AccessNode::AccessNodeVectorXXX, tag("VECTOR_XXX"))(s)
    }

    fn access_node_name(s: &str) -> IResult<&str, AccessNode> {
        map(parser::c_ident, AccessNode::AccessNodeName)(s)
    }
}

impl DBCString for AccessNode {
    fn dbc_string(&self) -> String {
        return match self {
            Self::AccessNodeName(s) => s.to_string(),
            Self::AccessNodeVectorXXX => "Vector__XXX".to_string(),
        }
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
        where
            Self: Sized {
        alt((Self::access_node_vector_xxx, Self::access_node_name))(s)
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

impl AccessType {
    fn dummy_node_vector_0(s: &str) -> IResult<&str, AccessType> {
        value(AccessType::DummyNodeVector0, char('0'))(s)
    }

    fn dummy_node_vector_1(s: &str) -> IResult<&str, AccessType> {
        value(AccessType::DummyNodeVector1, char('1'))(s)
    }

    fn dummy_node_vector_2(s: &str) -> IResult<&str, AccessType> {
        value(AccessType::DummyNodeVector2, char('2'))(s)
    }
    fn dummy_node_vector_3(s: &str) -> IResult<&str, AccessType> {
        value(AccessType::DummyNodeVector3, char('3'))(s)
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = tag("DUMMY_NODE_VECTOR")(s)?;
        let (s, node) = alt((
            Self::dummy_node_vector_0,
            Self::dummy_node_vector_1,
            Self::dummy_node_vector_2,
            Self::dummy_node_vector_3,
        ))(s)?;
        Ok((s, node))
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

impl Transmitter {

    fn transmitter_vector_xxx(s: &str) -> IResult<&str, Transmitter> {
        value(Transmitter::VectorXXX, tag("Vector__XXX"))(s)
    }

    fn transmitter_node_name(s: &str) -> IResult<&str, Transmitter> {
        map(parser::c_ident, Transmitter::NodeName)(s)
    }
}

impl DBCString for Transmitter {
    fn dbc_string(&self) -> String {
        return match self {
            Self::NodeName(s) => s.to_string(),
            Self::VectorXXX => "Vector__XXX".to_string(),
        }
    }

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        alt((Self::transmitter_vector_xxx, Self::transmitter_node_name))(s)
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MessageTransmitter {
    pub (crate) message_id: MessageId,
    pub (crate) transmitter: Vec<Transmitter>,
}

impl MessageTransmitter {
    fn message_transmitters(s: &str) -> IResult<&str, Vec<Transmitter>> {
        separated_list0(parser::comma, Transmitter::parse)(s)
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BO_TX_BU_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, transmitter) = Self::message_transmitters(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            MessageTransmitter {
                message_id,
                transmitter,
            },
        ))
    }
}
