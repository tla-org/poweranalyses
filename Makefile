CC := clang

.DEFAULT_GOAL := all

JULIA ?= julia

init-julia:
	$(JULIA) --startup-file=no --project=library -e 'using Pkg; Pkg.instantiate()'

build-lib:
	$(JULIA) --startup-file=no --project -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build library/build/build.jl

build-wasm:
	$(CC) \
		--target=wasm32 \
		-Wall \
		-o _build/pa.wasm \
		main.c

.PHONY: init-julia build-library build-wasm all
