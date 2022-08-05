# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}

AWS_CLI=aws
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
	@cargo run --bin local_graphql

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

release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*"
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*"
endif

package:
	@mkdir -p deploy
	@LAMBDAS="$(shell cargo metadata --no-deps --format-version 1 | jq '.packages[].targets[]| select(.name | startswith("lambda")) | .name' | sed -e s/\"//g)" && \
	for lambda in $$LAMBDAS ; do \
		md5sum -c target/$(CROSS_TARGET)/release/$$lambda.md5; \
		if [ $$? != 0 ]; then \
			zip -j9 deploy/$$lambda.zip target/$(CROSS_TARGET)/release/$$lambda && echo "@ $$lambda\n@=bootstrap" | zipnote -w deploy/$$lambda.zip ; \
			md5sum target/$(CROSS_TARGET)/release/$$lambda > target/$(CROSS_TARGET)/release/$$lambda.md5; \
		fi \
	done
	
# upx -9 target/$(CROSS_TARGET)/release/$$lambda

release.package.deploy: release package deploy

tail.graphql:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output graphql_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short
