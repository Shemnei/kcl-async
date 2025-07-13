use async_trait::async_trait;

use crate::{
    message::input::{
        InitializeMessage, LeaseLostMessage, ProcessRecordsMessage, ShardEndedMessage,
        ShutdownMessage, ShutdownRequestedMessage,
    },
    transport::Transport,
};

use super::checkpoint::Checkpointer;

#[async_trait]
pub trait Processor<T>
where
    T: Transport + Send,
{
    type Error;

    async fn initialize(&mut self, msg: InitializeMessage) -> Result<(), Self::Error>;

    async fn process_records(
        &mut self,
        msg: ProcessRecordsMessage,
        checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error>;

    async fn shutdown(
        &mut self,
        msg: ShutdownMessage,
        checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error>;

    async fn shutdown_requested(
        &mut self,
        msg: ShutdownRequestedMessage,
        checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error>;

    async fn lease_lost(&mut self, msg: LeaseLostMessage) -> Result<(), Self::Error>;

    async fn shard_ended(&mut self, msg: ShardEndedMessage) -> Result<(), Self::Error>;
}
