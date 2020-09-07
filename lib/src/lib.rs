#![deny(clippy::all)]
#![deny(clippy::cargo)]

//! This crate implements a simple irc message wrapper.
//!
//! This project has 2 goals:
//!
//! - Ease the access to fields of the message without requiring the user to handle offsets and other IRC related things.
//! - Minimize memory foodprint. For this goal the `Message` struct only owns the `String` of the actual message. Any
//!     parts of the message and other structs only work on references of this string.
//!
//! Therefore this project expects the strings passed to the struct
//! constructors to be valid parts of the IRC standard.
//!
//! As reference the [RFC2812](https://tools.ietf.org/html/rfc2812) and some extensions
//! from [IRCv3](https://ircv3.net/) are used.
//!
//! # Support
//!
//! Current support (as of version '0.2.*'):
//!
//! - **Message**: Create read-only Message from `String` or `&str` and with a builder `Message::builder()`.
//! - **Tags**: access through the indexing operator and iterating over all tags.
//! - **Prefix**: Read-only access + Builder.
//! - **Parameters List**: Read-only access, Iteration over elements, separate access to trailing parameter.
//! - **Serde**: Serialization in any format supported by serde.
//!
//! # Examples - for starters
//!
//! Simple example with static string:
//!
//! ```rust
//! use irc_rust::Message;
//!
//! let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
//!
//! assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
//! ```
//!
//! While reading from standard input the `Message::new` method has to be used.
//!
//! ```rust
//! use irc_rust::Message;
//! use std::io::{BufRead, stdin};
//!
//! for line in stdin().lock().lines() {
//!     match line {
//!         Ok(line) => {
//!             let message = Message::from(line);
//!             println!("> Received command: {}", message.command());
//!         }
//!         Err(e) => {
//!             println!("got error; aborting: {}", e);
//!             break;
//!         }
//!     }
//! }
//!
//! ```

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod builder;
mod errors;
mod message;
mod params;
mod prefix;
mod tags;

#[cfg(test)]
mod test;

pub use builder::Message as MessageBuilder;
pub use errors::{InvalidIrcFormatError, MessageBuildError};
pub use message::Message;
pub use params::Params;
pub use prefix::Prefix;
pub use tags::Tags;
