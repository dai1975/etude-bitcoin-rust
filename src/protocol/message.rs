use std;
use ::serialize::Serializable;

pub trait Message: Serializable + std::fmt::Display
{
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE];
}
