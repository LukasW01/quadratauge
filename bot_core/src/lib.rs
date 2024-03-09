#[macro_use]
extern crate log;
extern crate serenity;

pub mod client;
pub use client::bot;
mod command;
pub use command::{botCommand, botCommandBaseline, botCommandOption};
mod error;
pub use error::Error;
pub mod handler;
pub mod http;
mod intents;
pub mod response;
pub mod utils;
