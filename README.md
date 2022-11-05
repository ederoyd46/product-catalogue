# Product Catalogue

A home project to help me learn more about Rust in a serverless environment.

This is very much a work in progress.

## Architecture

- GraphQL server
- REST services
- DynamoDB backend

## Development Environment

I've been trying to overcome the issues developing in serverless by writing and running the code locally. This also allows me to easily debug the code (see VSCODE debug config).

Currently these means building 2 versions of the executable - one for local development (local*\*) and one for AWS Lambda (lambda*\*).

## DynamoDB

I've created 3 configurations for this, all running in Docker containers (for now).

### Local DynamoDB official image

```shell
make docker.start.dynamodb
```

NOTE - if this is the first time running this, you'll need to run the following to create the table:

```shell
make dynamodb.table.create
```

### Sclylla DB running with DynamoDB compatibility mode (https://www.scylladb.com)

```shell
make docker.start.scylladb
```

NOTE - if this is the first time running this, you'll need to run the following to create the table:

```shell
make dynamodb.table.create
```

### Localstack (https://localstack.cloud)

```shell
make docker.start.localstack
```

Additionally you can build and deploy the whole project to localstack with the following command:

```shell
make release.deploy
```

The Terraform output will give the generated urls for the GraphQL and Rest services.

NOTE - if not already set up, you will need to run `make terraform.init` first to configure the terraform modules.

### Development Setup

This has been written on a Mac but should work on Linux.

To cross compile onto Linux there are a couple of dependencies to install:

```shell
brew install filosottile/musl-cross/musl-cross
```

```shell
rustup component add rust-std-x86_64-unknown-linux-musl
```

Update the Cargo config to use the musl toolchain (~/.cargo/config):

```shell
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

### Build

To build the code for local development:

```shell
make build
```

The build the code for AWS Lambda:

```shell
make release
```

## Examples

### GraphQL

```javascript
mutation product ($args: NewProduct!) {
  createProduct(newProduct: $args) {
    key
    name
    description
    price {
        amount
        currencyCode
    }
  }
}
```

```javascript
query {
  viewProduct(key: "e83bb923-293b-4795-be51-3b0770d9c6d2") {
    key
    name
    description
    price {
        amount
        currencyCode
    }
  }
}
```

### Rest (using POSTMAN)

```json
POST http://localhost:8081
{
    "key": "{{$randomUUID}}",
    "name": "{{$randomProductName}}",
    "price": {
        "currency_code": "GBP",
        "amount": {{$randomPrice}}
    },
    "description": "{{$randomLoremWords}}"
}
```


```json
POST http://localhost:8082
{
    "key": "{{$randomUUID}}",
    "inventory": {{$randomInt}}
}
```
