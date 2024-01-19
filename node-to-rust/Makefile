ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input.$(EXT) node_modules/ffi
	echo TODO!!
	#node src/main.js

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

node_modules/ffi:
	echo TODO!!
	#mkdir -p node_modules
	#npm install ffi
	#yarn add ffi || npm install ffi
	#yarn || npm install

clean:
	rm -rf ~/.nvm/versions/node/v16.*.*/lib/node_modules
	rm -rf ~/Library/Caches/node-gyp
	rm -rf target
	rm -rf node_modules
