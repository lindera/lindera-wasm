.DEFAULT_GOAL := help

VERSION := $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="lindera-wasm") | .version')

default: ## Show Makefile list
	@echo $(MAKEFILE_LIST)

clean: ## Clean the project
	cargo clean
	rm -rf pkg

format: ## Format the project
	cargo fmt

lint: ## Lint the project
	cargo clippy --features=cjk

test: ## Test the project
	wasm-pack test --node

build: ## Build the project
	wasm-pack build --release --features=cjk --target=bundler

publish: ## Publish the project
	wasm-pack publish --access=public --target=bundler

build-example: ## Build the example application
	cd lindera-wasm && \
	jq '.version = "$(VERSION)"' ./package.json > ./temp.json && mv ./temp.json ./package.json && \
	npm install && \
	npm run build && \
	cp index.html dist/index.html

run-example: ## Run the example application
	cd lindera-wasm && npm run start

clean-example: ## Clean the example application
	cd lindera-wasm && \
	rm -rf dist && \
	rm -rf node_modules && \
	rm -rf package-lock.json && \
	rm -rf temp.json

tag: ## Make a tag
	git tag v$(VERSION)
	git push origin v$(VERSION)

help: ## Show help
	@echo "Available targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'
