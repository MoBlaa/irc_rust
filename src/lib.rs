#![feature(test)]

//! This crate implements a simple irc message wrapper. Its goal is to ease the access
//! to fields of the message without requiring the user to handle offsets and other IRC
//! related things. Therefore this project expects the strings passed to tÖLhe struct
//! constructors to be valid parts of the IRC standard.
//!
//! As reference the [RFC2812](https://tools.ietf.org/html/rfc2812) and some extensions
//! from [IRCv3](https://ircv3.net/) are used.
//!
//! # Support
//!
//! Current support (as of version '0.1.1'):
//!
//! - **Message**: Create read-only Message from `&str` and with a builder `Message::builder()`.
//! - **Tags**: access through the indexing operator and iterating over all tags.
//! - **Prefix**: Read-only access + Builder.
//! - **Parameters List**: Read-only access, Iteration over elements, separate access to trailing parameter.
//!
//! # Examples - for starters
//!
//! ```
//! use irc_rust::message::Message;
//!
//! let message = Message::new("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
//!
//! assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
//! ```

pub mod message;
pub mod tags;
pub mod prefix;
pub mod params;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;