use checkpoint::Checkpointer;
use message::input::Message as MessageIn;
use message::output::Message as MessageOut;
use processor::Processor;
use transport::Transport;

pub mod checkpoint;
pub mod message;
pub mod processor;
pub mod transport;

#[derive(Debug, thiserror::Error)]
pub enum RunError<IoError, ProcessorError> {
    UnexpectedMessage(MessageIn),

    #[error(transparent)]
    IoError(IoError),

    #[error(transparent)]
    ProcessorError(ProcessorError),
}

pub async fn run<T: Transport + Send, P: Processor<T>>(
    mut transport: T,
    mut processor: P,
) -> Result<(), RunError<T::Error, P::Error>> {
    loop {
        let msg = transport.read_message().await.map_err(RunError::IoError)?;
        let msg_id = msg.id();

        {
            let mut checkpointer = Checkpointer::new(&mut transport);

            match msg {
                MessageIn::Initialize(m) => processor.initialize(m).await,
                MessageIn::ProcessRecords(m) => {
                    processor.process_records(m, &mut checkpointer).await
                }
                MessageIn::Shutdown(m) => processor.shutdown(m, &mut checkpointer).await,
                MessageIn::ShutdownRequested(m) => {
                    processor.shutdown_requested(m, &mut checkpointer).await
                }
                MessageIn::LeaseLost(m) => processor.lease_lost(m).await,
                MessageIn::ShardEnded(m) => processor.shard_ended(m).await,

                msg => {
                    return Err(RunError::UnexpectedMessage(msg));
                }
            }
            .map_err(RunError::ProcessorError)?;
        }

        {
            // Acknowledge Message

            let response = MessageOut::Status(message::output::StatusMessage {
                response_for: msg_id.into(),
            });

            transport
                .write_message(&response)
                .await
                .map_err(RunError::IoError)?;
        }
    }
}
