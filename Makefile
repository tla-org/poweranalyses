.DEFAULT_GOAL := all

JULIA ?= julia

lib:
	$(JULIA) --startup-file=no --project -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build library/build/build.jl

wasm:
	clang \
		--target=wasm32 \
		-Wall \
		-o _build/pa.wasm \
		main.c

all: lib wasm

.PHONY: init-julia build-library build-wasm all
