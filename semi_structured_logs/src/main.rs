#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Error => error(message),
        LogLevel::Warning => warn(message),
        LogLevel::Info => info(message)
    }
}

pub fn info(message: &str) -> String {
    let mut str : &str = "[INFO]: ";
    str.to_owned() + message
}

pub fn warn(message: &str) -> String {
    let mut str : &str = "[WARNING]: ";
    str.to_owned() + message
}

pub fn error(message: &str) -> String {
    let mut str : &str = "[ERROR]: ";
    str.to_owned() + message
}

fn main() {
    println!("{}", log(LogLevel::Error, "Stack"));
}
