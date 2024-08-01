CC = "$(WASI_SDK_PATH)/bin/clang" --sysroot="$(WASI_SDK_PATH)/share/wasi-sysroot"

CFLAGS = -W -Wall -Wextra -Werror -Wno-unused -Wconversion -Wsign-conversion -MMD -MP -fno-exceptions
	
LDFLAGS += -Wl,--export-all,--no-gc-sections
OBJECTS = $(patsubst src/%.c, build/%.o, $(wildcard src/*.c))
DEPS = $(OBJECTS:.o=.d)


all: build/game.wasm

build/game.wasm: $(OBJECTS)
	$(CC) -o $@ $(OBJECTS)


build/%.o: src/%.c
	$(CC) -c $< -o $@ $(CFLAGS)

-include DEPS
