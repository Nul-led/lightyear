use crate::BitSerializable;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tracing::info;

use crate::channel::receivers::fragment_receiver::FragmentReceiver;
use crate::channel::receivers::ChannelReceive;
use crate::packet::message::{FragmentData, MessageContainer, SingleData};

const DISCARD_AFTER: Duration = Duration::from_secs(3);

pub struct UnorderedUnreliableReceiver {
    recv_message_buffer: VecDeque<SingleData>,
    fragment_receiver: FragmentReceiver,
    // TODO: maybe use WrappedTime
    current_time: Instant,
}

impl UnorderedUnreliableReceiver {
    pub fn new() -> Self {
        Self {
            recv_message_buffer: VecDeque::new(),
            fragment_receiver: FragmentReceiver::new(),
            current_time: Instant::now(),
        }
    }
}

impl ChannelReceive for UnorderedUnreliableReceiver {
    fn update(&mut self, delta: Duration) {
        self.current_time += delta;
        self.fragment_receiver
            .cleanup(self.current_time - DISCARD_AFTER);
    }

    fn buffer_recv(&mut self, message: MessageContainer) -> anyhow::Result<()> {
        match message {
            MessageContainer::Single(data) => self.recv_message_buffer.push_back(data),
            MessageContainer::Fragment(fragment) => {
                if let Some(data) = self
                    .fragment_receiver
                    .receive_fragment(fragment, Some(self.current_time))?
                {
                    self.recv_message_buffer.push_back(data);
                }
            }
        }
        Ok(())
    }

    fn read_message(&mut self) -> Option<SingleData> {
        self.recv_message_buffer.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::receivers::ChannelReceive;
    use crate::packet::message::{MessageId, SingleData};
    use crate::MessageContainer;
    use bytes::Bytes;

    #[test]
    fn test_unordered_unreliable_receiver_internals() -> anyhow::Result<()> {
        let mut receiver = UnorderedUnreliableReceiver::new();

        let mut single1 = SingleData::new(None, Bytes::from("hello"));
        let mut single2 = SingleData::new(None, Bytes::from("world"));

        // receive an old message
        single2.id = Some(MessageId(60000));
        receiver.buffer_recv(single2.clone().into())?;

        // it still gets read
        assert_eq!(receiver.recv_message_buffer.len(), 1);
        assert_eq!(receiver.read_message(), Some(single2.clone()));

        // receive message in the wrong order
        single2.id = Some(MessageId(1));
        receiver.buffer_recv(single2.clone().into())?;

        // we process the message
        assert_eq!(receiver.recv_message_buffer.len(), 1);
        assert_eq!(receiver.read_message(), Some(single2.clone()));

        // receive message 0
        single1.id = Some(MessageId(0));
        receiver.buffer_recv(single1.clone().into())?;

        // we process the message
        assert_eq!(receiver.recv_message_buffer.len(), 1);
        assert_eq!(receiver.read_message(), Some(single1.clone()));
        Ok(())
    }
}
