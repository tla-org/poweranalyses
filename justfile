default: build serve

alias s := serve
alias b := build

serve:
	@echo "Serving..."
	@cd app && npm run dev -- --open

build:
	@echo "Building..."
	@cargo build --target wasm32-unknown-emscripten --release

test:
	@echo "Testing..."
	@cargo test 

deploy:
	@echo "Deploying..."
	@cd app && npm run build