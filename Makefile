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

build_local: 
	@cargo build --bin "local_*"

build_lambda: 
	@cargo build --bin "lambda_*"

run_local_graph:
	STORE_PRODUCT_URL="$(shell $(TERRAFORM) output store_product_url)"
	STORE_INVENTORY_URL="$(shell $(TERRAFORM) output store_inventory_url)"
	$$STORE_PRODUCT_URL $$STORE_INVENTORY_URL cargo run --bin local_graphql

test:
	@cargo test

#  Terraform
plan:
	@$(TERRAFORM) plan -var-file=terraform.tfvars

terraform.init:
	@$(TERRAFORM) init

deploy:
	@$(TERRAFORM) apply -auto-approve

remove:
	@$(TERRAFORM) destroy -auto-approve

output:
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
	
# upx -9 target/$(CROSS_TARGET)/release/$$lambda

release.deploy: release package deploy

terraform.output:
	$(TERRAFORM) output

tail.graphql:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output graphql_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

tail.store.product:
	LOG_GROUP_NAME=$(shell $(TERRAFORM) output store_product_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

