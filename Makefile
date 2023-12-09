TARGET="thumbv7em-none-eabihf"
BUILD_OPTIONS=--target $(TARGET)

# follows can set in .cargo/config
export export CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER=hf2 elf
export RUSTFLAGS=-C link-arg=-Tlink.x -C link-arg=--nmagic

.phony: all clean

all: uart_hello

button_led:
uart_hello:
	$(MAKE) fmt
	cargo hf2 $(BUILD_OPTIONS) --example $@


fmt:
	cargo fmt

clean:
	cargo clean
