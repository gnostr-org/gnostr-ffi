# Rust FFI Examples

This is an example repository that shows how to interface between Rust and a
various number of other languages.

At this time, though, I unfortunately don't have time to continue maintaining
this so I'm going to archive the repository. Thanks to everyone for
contributing!

```
# RLIB would be the default for rust
# this gets invoked as `rustc --crate-type=lib --crate-name=foo foosrc/lib1.rs`
add_library(foo, foosrc/lib.rs RLIB)

# `rustc --crate-type=staticlib --crate-name=bar barsrc/lib.rs`
add_library(bar, barsrc/entry.rs STATIC)

# `rustc --crate-type=cdylib --crate-name=baz baz
src/entry.rs`
add_library(baz, bazsrc/entry.rs SHARED)

# built as normal
add_library(syslib, sys.c STATIC)

add_executable(main bin/main.rs)
add_dependencies(main foo)
add_dependencies(main syslib)
add_dependencies(main bar)

```

```
rustc --crate-type=bin \
    -lstatic:outdir/syslib.a \
    -lstatic:outdir/bar.a \
    --extern=foo=libfoo.rlib \
    bin/main.rs \
    -omain
```

##### REF: [https://discourse.cmake.org/t/enable-language-option-for-rust-cargo/8514/7](https://discourse.cmake.org/t/enable-language-option-for-rust-cargo/8514/7)
