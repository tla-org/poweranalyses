.DEFAULT_GOAL := all

JULIA ?= julia

SYSIMAGE_PATH := $(shell julia --startup-file=no -e 'print(unsafe_string(Base.JLOptions().image_file))')

# Based on the PackageCompiler devdocs sysimages_part_1.md
julia.o:
	$(JULIA) --startup-file=no --project -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=library/build -e 'using Pkg; Pkg.instantiate()'
	$(JULIA) --startup-file=no --project=build build/create_object_file.jl

JULIALIB_PATH := $(shell julia --startup-file=no -e 'print(abspath(Sys.BINDIR, Base.LIBDIR))')

INCLUDE_DIR := "_library/include"
LIB_DIR := "_library/lib"

# Linking in code from a library .o file into an executable is possible according to
# https://www.cs.swarthmore.edu/~newhall/unixhelp/howto_C_libraries.html.
# It states that resulting executables will contain machine code for the c code and the library.
#
# You can run the output with, for example, wasmer
poweranalyses.wasm:
	echo "WARN: Use something else for the INCLUDE_DIR"
	emcc \
		-Wall \
		julia.o \
		-L$(JULIALIB_PATH) \
		-I$(INCLUDE_DIR) \
		-o poweranalyses.wasm \
		main.c

all: julia.o poweranalyses.wasm

.PHONY: init-julia build-wasm all julia.o poweranalyses.wasm
