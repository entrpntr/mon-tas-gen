#![feature(unsized_locals)]

extern crate bincode;
#[macro_use] extern crate bitflags;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate flate2;
extern crate num_cpus;

pub mod ftii;
pub mod gambatte;
pub mod gb;
pub mod gbexecutor;
pub mod rom;
pub mod segment;
pub mod statebuffer;
pub mod util;
