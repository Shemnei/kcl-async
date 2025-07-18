# kcl-async

> [!CAUTION]
>
> Under development and provides no guarantees for compatibility/future updates.
> Use at your own risk.

Rust async library for AWS Kinesis Client Library (KCL) consumers using the [MultiLangDaemon interface](https://github.com/awslabs/amazon-kinesis-client/blob/master/amazon-kinesis-client-multilang/src/main/java/software/amazon/kinesis/multilang/package-info.java).

## Usage

### Examples

See [./examples](./examples) for a full usage example of this crate.

```rust
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
```

### Bootstrap

This repo provides a tool which bootstraps the KCL setup, downloading the [required JAR files](./examples/example_consumer/pom.xml) and providing the command for running the application using KCL.
This tool is located at [./kcl-bootstrap](./kcl-bootstrap).
It must be run in the root directory and provided a [KCL configuration](https://github.com/awslabs/amazon-kinesis-client/blob/master/docs/kcl-configurations.md).

```shell
# Sets up JARS and prints run command
./kcl-bootstrap --properties <PATH-TO-KCL-PROPERTIES>

# Sets up JARS and executes command
./kcl-bootstrap --properties <PATH-TO-KCL-PROPERTIES> --execute
```

## Mentions

- Similar (sync) crate: <https://github.com/validus-risk-management/amazon-kinesis-client-rust>
- MultiLang Daemon API: <https://github.com/awslabs/amazon-kinesis-client-python>
- Messages: <https://github.com/awslabs/amazon-kinesis-client/tree/master/amazon-kinesis-client-multilang/src/main/java/software/amazon/kinesis/multilang/messages>
- Bootstrap: <https://github.com/awslabs/amazon-kinesis-client-net/blob/master/Bootstrap/Bootstrap.cs>

## References

- [MultiLangDaemon pom.xml](https://github.com/awslabs/amazon-kinesis-client-net/blob/master/pom.xml)
- [KCL Config Spec](https://github.com/awslabs/amazon-kinesis-client/blob/master/docs/kcl-configurations.md)
