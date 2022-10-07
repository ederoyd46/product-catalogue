# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}

AWS_CLI=aws --endpoint-url=http://localhost:4566
TERRAFORM=terraform -chdir=./infrastructure/aws

CROSS_TARGET=x86_64-unknown-linux-musl
CROSS_COMPILE=x86_64-linux-musl-

# Tasks

.PHONY: deploy

# Build Locally
build: 
	@cargo build 

build.local: 
	@cargo build --bin "local_*"

build.lambda: 
	@cargo build --bin "lambda_*"

run.local.graph.against.deployed:
	@STORE_PRODUCT_URL="$(shell $(TERRAFORM) output store_product_url)" \
	STORE_INVENTORY_URL="$(shell $(TERRAFORM) output store_inventory_url)" \
	cargo run --bin local_graphql

run.local:
	@LAMBDAS="$(shell cargo metadata --no-deps --format-version 1 | jq '.packages[].targets[]| select(.name | startswith("local")) | .name' | sed -e s/\"//g)" && \
	for lambda in $$LAMBDAS ; do \
		cargo run --bin $$lambda & \
	done; \

kill.local: 
	@LAMBDAS="$(shell cargo metadata --no-deps --format-version 1 | jq '.packages[].targets[]| select(.name | startswith("local")) | .name' | sed -e s/\"//g)" && \
	for lambda in $$LAMBDAS ; do \
		killall $$lambda; \
	done; 

test:
	@cargo test

#  Terraform
plan:
	@$(TERRAFORM) plan

terraform.init:
	@$(TERRAFORM) init

deploy:
	@$(TERRAFORM) apply -auto-approve

remove:
	@$(TERRAFORM) destroy -auto-approve

terraform.output:
	@$(TERRAFORM) output

release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*"
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*"
endif

package:
	@mkdir -p deploy
	@LAMBDAS="$(shell cargo metadata --no-deps --format-version 1 | jq '.packages[].targets[]| select(.name | startswith("lambda")) | .name' | sed -e s/\"//g)" && \
	TEMP_DIR="$(shell mktemp -d)" && \
	for lambda in $$LAMBDAS ; do \
		md5sum -c target/$(CROSS_TARGET)/release/$$lambda.md5 2> /dev/null; \
		if [ $$? != 0 ]; then \
			mkdir $$TEMP_DIR/$$lambda ; \
			cp target/$(CROSS_TARGET)/release/$$lambda $$TEMP_DIR/$$lambda/bootstrap ; \
			echo Packaging $$lambda ; \
			zip -j9 deploy/$$lambda.zip $$TEMP_DIR/$$lambda/bootstrap ; \
			md5sum target/$(CROSS_TARGET)/release/$$lambda > target/$(CROSS_TARGET)/release/$$lambda.md5; \
		fi \
	done ; \
	rm -rf $$TEMP_DIR

release.deploy: release package deploy

tail.graphql:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output graphql_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

tail.store.product:
	LOG_GROUP_NAME=$(shell $(TERRAFORM) output store_product_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

tail.store.invetory:
	LOG_GROUP_NAME=$(shell $(TERRAFORM) output store_inventory_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

dynamodb.table.list:
	@$(AWS_CLI) dynamodb list-tables

dynamodb.table.create:
	@$(AWS_CLI) dynamodb create-table \
		--table-name default-product-catalogue \
		--attribute-definitions=AttributeName=PK,AttributeType=S \
		--key-schema=AttributeName=PK,KeyType=HASH \
		--billing-mode PAY_PER_REQUEST

dynamodb.table.scan:
	@$(AWS_CLI) dynamodb scan --table-name default-product-catalogue

dynamodb.table.scan.pk:
	@make dynamodb.table.scan | jq .Items[].PK.S

docker.start.dynamodb:
	@docker run --name=dynamodb-local --rm=true -d -p 4566:8000 amazon/dynamodb-local
	@docker logs -f dynamodb-local

docker.start.scylladb:
	@docker run --name=scylladb-local --rm=true -d -p 4566:4566 scylladb/scylla --alternator-port 4566 --alternator-write-isolation always
	@docker logs -f scylladb-local

docker.start.localstack:
	@docker compose up -d
