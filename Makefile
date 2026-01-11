.DEFAULT_GOAL := help

VERSION := $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="lindera-wasm") | .version')

clean: ## Clean the project
	cargo clean
	rm -rf pkg
	rm -rf lindera-wasm/dist
	rm -rf lindera-wasm/node_modules

format: ## Format the project
	cargo fmt

lint: ## Lint the project
	cargo clippy --all-features

test: ## Test the project
	wasm-pack test --node --all-features

build: ## Build the project
	wasm-pack build --release --all-features --target=web

publish: ## Publish the project
	wasm-pack publish --access=public --target=web

build-example: ## Build the example application
	wasm-pack build --release --features=embed-ipadic --target=web && \
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
