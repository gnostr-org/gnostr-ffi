ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/double
	@target/double
	@cargo run -q

target:
	@mkdir -p $@

target/double: target/main.o target/debug/libgnostr.a
	@$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libgnostr.a: src/lib.rs Cargo.toml
	@cargo build

target/main.o: src/main.c | target
#echo $@
#echo $<
	@$(CC) -o $@ -c $<

clean:
	rm -rf target
