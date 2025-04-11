TARGET="thumbv7em-none-eabihf"
BUILD_OPTIONS=--target $(TARGET)

# follows can set in .cargo/config
export export CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER=hf2 elf
export RUSTFLAGS=-C link-arg=-Tlink.x -C link-arg=--nmagic

.phony: all clean

all: pwm_buzzer

pwm_buzzer simple_buzzer button_led uart_hello echo_and_ledtoggle spi_read_display_power_mode lcd:
	$(MAKE) fmt
	cargo hf2 $(BUILD_OPTIONS) --example $@


fmt:
	cargo fmt

env:
	rustup target add thumbv7em-none-eabihf
	cargo install cargo-generate
	cargo install hf2-cli
	cargo install cargo-hf2

clean:
	cargo clean
