


use super::service::sender::sender_interface::MessageSender;
use super::service::receiver::receiver_interface::MessageReceiver;
pub trait TelegramFacadeInterface: MessageSender + MessageReceiver {
}