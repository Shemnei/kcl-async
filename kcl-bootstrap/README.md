# kcl-bootstrap

Binary which sets up the environment to run any application with AWS Kinesis Client Library (KCL).

## Run

Requires two files:

- [`pom.xml`](https://github.com/awslabs/amazon-kinesis-client-net/blob/master/pom.xml): By default the current working directory is search for it
- [`XXX.properties`](https://github.com/awslabs/amazon-kinesis-client/blob/master/docs/kcl-configurations.md): Configuration for KCL, provided with `--properties`

```shell
# Sets up JARS and prints run command
./kcl-bootstrap --properties <PATH-TO-KCL-PROPERTIES>

# Sets up JARS and executes command
./kcl-bootstrap --properties <PATH-TO-KCL-PROPERTIES> --execute
```

## Mentions

- Logic from: <https://github.com/awslabs/amazon-kinesis-client-net/tree/master/Bootstrap>
