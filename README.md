# kvm-rs

[![Rust](https://github.com/johannst/mini-kvm-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/johannst/mini-kvm-rs/actions/workflows/rust.yml)

A playground for the [`Kernel Virtual Machine
(KVM)`](https://www.kernel.org/doc/html/latest/virt/kvm/index.html) in rust to
learn about `KVM`, [`rust
ffi`](https://doc.rust-lang.org/stable/std/ffi/index.html) and `x86_64`.

The sources are structured as follows:
- [`src/`](./src) provides a small library as abstraction over the raw [KVM
  API](https://www.kernel.org/doc/html/latest/virt/kvm/api.html#api-description).
- [`examples/`](./examples) contains example VMs using the library above.
- [`guest/`](./guest) contains the guest source code.
- [`sysdeps/`](./sysdeps) helper to generate some KVM constants from the system
  header (executed by [build.rs](./build.rs)).

## Documentation

Rustdoc for this crate is avalable at
[johannst.github.io/mini-kvm-rs](https://johannst.github.io/mini-kvm-rs/kvm_rs/index.html).

## Real Mode (16bit) example

Runs the [real mode VM](./examples/real_mode.rs) with the [guest program](./guest/guest16.S).

```bash
# Once: Build the guest binary image.
make -C guest

# Run the Real Mode example.
cargo run --example real_mode
```

## Long Mode (64bit) example

Runs the [long mode VM](./examples/long_mode.rs) with the [guest program](./guest/guest64.S).

```bash
# Once: Build the guest binary image.
make -C guest

# Run the Long Mode example.
cargo run --example long_mode
```

## License
This project is licensed under the [MIT](LICENSE) license.
