.DEFAULT_GOAL := all

JULIA ?= julia

SYSIMAGE_PATH := $(shell julia --startup-file=no -e 'print(unsafe_string(Base.JLOptions().image_file))')

# Based on the PackageCompiler devdocs sysimages_part_1.md
lib:
	$(JULIA) --startup-file=no --project -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=build build/create_object_file.jl

INCLUDE_DIR := "_library/include"
LIB_DIR := "_library/lib"

# You can run the output with, for example, wasmer
wasm:
	emcc \
		-Wall \
		-I$(INCLUDE_DIR) \
		-L$(LIB_DIR) \
		-ljulia \
		-lmylib \
		-o poweranalyses.wasm \
		main.c

all: lib wasm

.PHONY: init-julia build-library build-wasm all
