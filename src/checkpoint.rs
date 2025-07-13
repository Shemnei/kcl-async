use super::{
    message::input::Message as MessageIn, message::output::Message as MessageOut,
    transport::Transport,
};

#[derive(Debug, thiserror::Error)]
pub enum CheckpointError<TransportError> {
    #[error(transparent)]
    TransportError(TransportError),

    #[error("failed to checkpoint: {reason}")]
    Failed { reason: String },

    #[error("invalid state: {}", message.id())]
    InvalidState { message: MessageIn },
}

pub struct Checkpointer<'a, T>(&'a mut T);

impl<'a, T> Checkpointer<'a, T>
where
    T: Transport,
{
    pub(crate) fn new(transport: &'a mut T) -> Self {
        Self(transport)
    }

    pub async fn checkpoint(
        &mut self,
        sequence_number: Option<String>,
        sub_sequence_number: Option<u64>,
    ) -> Result<(), CheckpointError<T::Error>> {
        let request = MessageOut::Checkpoint(super::message::output::CheckpointMessage {
            sequence_number,
            sub_sequence_number,
        });

        self.0
            .write_message(&request)
            .await
            .map_err(CheckpointError::TransportError)?;

        let response = self
            .0
            .read_message()
            .await
            .map_err(CheckpointError::TransportError)?;

        if let MessageIn::Checkpoint(msg) = response {
            if let Some(error) = msg.error {
                Err(CheckpointError::Failed { reason: error })
            } else {
                Ok(())
            }
        } else {
            Err(CheckpointError::InvalidState { message: response })
        }
    }
}
