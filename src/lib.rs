//! Domain Name System (DNS) communication protocol.

#![deny(missing_docs)]

extern crate idna as external_idna;

pub use crate::address::address_name;
pub use crate::config::DnsConfig;
pub use crate::idna::{to_ascii, to_unicode};
pub use crate::message::{DecodeError, EncodeError, Message, Question, Resource, MESSAGE_LIMIT};
pub use crate::record::{Class, Record, RecordType};
pub use crate::resolver::{resolve_addr, resolve_host, DnsResolver};
pub use crate::socket::{DnsSocket, Error};

pub mod address;
pub mod config;
pub mod hostname;
pub mod hosts;
pub mod idna;
pub mod message;
pub mod record;
#[cfg(unix)]
pub mod resolv_conf;
pub mod resolver;
pub mod socket;
