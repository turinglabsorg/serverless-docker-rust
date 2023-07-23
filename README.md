# Rust on Serverless Framework through Docker 🦀🐳

This is a working project to deploy Rust Serverless through Docker using Serverless Framework. 
We use [cargo-lambda](https://www.cargo-lambda.info/) for local development and Serverless framework to deploy on AWS.

A working example is available in the `hello` folder, which is a simple hello world function taken from [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime). We need to add `lambda_http` and `lambda_runtime` crates to get it work with Api Gateway. 

You can see that function live [here](https://gbb4vjgprd.execute-api.us-east-1.amazonaws.com/dev/).

Why use docker? Because it's the only way to get a working environment for Rust on Serverless Framework, all the other packages are outdated and not working, happy to be wrong on this.

AWS architecture is `arm64` and Docker image builds `linux/arm64` as well.

## Local deployment
To deploy locally, you need to install [Docker](https://www.docker.com/) and [cargo-lambda](https://www.cargo-lambda.info/), then run the following commands:

```bash
cd hello 
cargo lambda watch -a 127.0.0.1 -p 8000
```

## Deploy
We suggest to deploy with `--verbose` flag to see the output of the deployment.
```bash
serverless deploy --verbose
```