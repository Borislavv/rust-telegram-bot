use std::fmt::Debug;
use serde::Serialize;
use crate::telegram::domain::{contract, model};
use super::service::sender::sender_interface::MessageSender;
use super::service::receiver::receiver_interface::MessageReceiver;
pub trait TelegramFacadeInterface: MessageSender + MessageReceiver {
}