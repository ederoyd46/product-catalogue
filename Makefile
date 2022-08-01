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
# @cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*" -Z unstable-options --out-dir ./deploy
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*"
# @CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release --bin "lambda_*" -Z unstable-options --out-dir ./deploy
endif

package: 
	@mkdir -p deploy
	zip -uj9 deploy/graphql.zip target/$(CROSS_TARGET)/release/lambda_graphql && echo "@ lambda_graphql\n@=bootstrap" | zipnote -w deploy/graphql.zip
	
	# @upx -9 deploy/graphql/bootstrap

# package:
# 	zip -j9 deploy/graphql.zip deploy/lambda_graphql
# 	echo "@ lambda_graphql\n@=bootstrap" | zipnote -w deploy/graphql.zip

release.package.deploy: release package deploy

tail.graphql:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output graphql_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short
