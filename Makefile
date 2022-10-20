CC := clang

build:
	$(CC) \
		--target=wasm32 \
		-Wall \
		-o _build/pa.wasm \
		main.c

