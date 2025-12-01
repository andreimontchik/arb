pub mod mmap_receiver;

pub use mmap_receiver::MmapReceiver;
use {
    anyhow::Result,
    common::{message::Message, serializer::Serializer},
    serde_json::Value,
};

pub trait Receiver<T: Serializer> {
    fn new(config: &Value) -> Self;
    fn try_new(config: &Value) -> Option<Self>
    where
        Self: Sized;
    fn receive(&mut self) -> Result<Option<Message>>;
}
