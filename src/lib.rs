#![feature(test)]

pub mod message;
pub mod tags;
pub mod prefix;
pub mod params;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;