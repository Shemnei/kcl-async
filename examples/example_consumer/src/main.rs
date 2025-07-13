use async_trait::async_trait;
use kcl_async::{
    checkpoint::Checkpointer,
    message::input::{
        InitializeMessage, LeaseLostMessage, ProcessRecordsMessage, ShardEndedMessage,
        ShutdownMessage, ShutdownRequestedMessage,
    },
    processor::Processor,
    run,
    transport::{StdTransport, Transport},
};
use tracing::error;

pub struct ExampleProcessor;

#[async_trait]
impl<T: Transport + Send> Processor<T> for ExampleProcessor {
    type Error = ();

    async fn initialize(&mut self, _msg: InitializeMessage) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn process_records(
        &mut self,
        msg: ProcessRecordsMessage,
        checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error> {
        for record in msg.records {
            let _bytes = record.to_bytes();

            // Process ...
        }

        if checkpointer.checkpoint(None, None).await.is_err() {
            return Err(());
        }

        Ok(())
    }

    async fn shutdown(
        &mut self,
        _msg: ShutdownMessage,
        _checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error> {
        Err(())
    }

    async fn shutdown_requested(
        &mut self,
        _msg: ShutdownRequestedMessage,
        _checkpointer: &mut Checkpointer<'_, T>,
    ) -> Result<(), Self::Error> {
        Err(())
    }

    async fn lease_lost(&mut self, _msg: LeaseLostMessage) -> Result<(), Self::Error> {
        Err(())
    }

    async fn shard_ended(&mut self, _msg: ShardEndedMessage) -> Result<(), Self::Error> {
        Err(())
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Setup
    // e.g. tracing, database connections, ...

    // Start KCL process
    if let Err(err) = run(StdTransport::new(), ExampleProcessor).await {
        error!("Failed execution: {err:?}");
        return Err(());
    }

    Ok(())
}
