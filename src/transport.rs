use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};

use super::message::input::Message as MessageIn;
use super::message::output::Message as MessageOut;

#[async_trait]
pub trait Transport {
    type Error;

    async fn write_error(&mut self, error: &str) -> Result<(), Self::Error>;

    async fn write_message(&mut self, message: &MessageOut) -> Result<(), Self::Error>;
    async fn read_message(&mut self) -> Result<MessageIn, Self::Error>;
}

#[derive(Debug)]
pub struct StdTransport {
    buf: String,
    stdin: io::BufReader<io::Stdin>,
    stderr: io::BufWriter<io::Stderr>,
    stdout: std::io::BufWriter<std::io::Stdout>,
}

impl Default for StdTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl StdTransport {
    pub fn new() -> Self {
        Self {
            buf: String::with_capacity(2048),
            stdin: io::BufReader::new(io::stdin()),
            stderr: io::BufWriter::new(io::stderr()),
            stdout: std::io::BufWriter::new(std::io::stdout()),
        }
    }
}

#[async_trait]
impl Transport for StdTransport {
    type Error = std::io::Error;

    async fn write_error(&mut self, error: &str) -> Result<(), Self::Error> {
        self.stderr.write_u8(b'\n').await?;
        self.stderr.write_all(error.as_bytes()).await?;
        self.stderr.write_u8(b'\n').await?;

        self.stderr.flush().await?;

        Ok(())
    }

    async fn write_message(&mut self, message: &MessageOut) -> Result<(), Self::Error> {
        self.stdout.write_all(b"\n")?;
        simd_json::to_writer(&mut self.stdout, message)?;
        self.stdout.write_all(b"\n")?;

        self.stdout.flush()?;

        Ok(())
    }

    async fn read_message(&mut self) -> Result<MessageIn, Self::Error> {
        self.buf.clear();
        self.stdin.read_line(&mut self.buf).await?;
        let msg = match simd_json::from_slice(unsafe { self.buf.as_bytes_mut() }) {
            Ok(msg) => msg,
            Err(err) => {
                // Get current timestamp as milliseconds since UNIX_EPOCH
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis()
                    .to_string();

                // Create directory "failures" if it doesn't exist
                let dir_path = Path::new("failures");
                if let Err(e) = std::fs::create_dir_all(dir_path) {
                    panic!("Failed to create failures directory: {e}");
                }

                // Create file with the timestamp as the filename
                let file_path = dir_path.join(&timestamp);
                let mut file = match File::create(&file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        panic!("Failed to create file {}: {}", file_path.display(), e);
                    }
                };

                // Write error, newline, then the message buffer
                if let Err(e) = writeln!(file, "{}\n\n{}", err, self.buf) {
                    eprintln!("Failed to write to file {}: {}", file_path.display(), e);
                }

                panic!("Failed to process message");
            }
        };

        Ok(msg)
    }
}
