#[macro_use]
extern crate log;
extern crate serenity;

pub mod client;
pub use client::Bot;
mod command;
pub use command::{BotCommand, BotCommandBaseline, BotCommandOption};
mod error;
pub use error::Error;
pub mod handler;
pub mod http;
mod intents;
pub mod response;
pub mod utils;
