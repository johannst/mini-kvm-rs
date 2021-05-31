# kvm-rs 

[![Rust](https://github.com/johannst/mini-kvm-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/johannst/mini-kvm-rs/actions/workflows/rust.yml)

A playground for the [`Kernel Virtual Machine
(KVM)`](https://www.kernel.org/doc/html/latest/virt/kvm/index.html) in rust to
learn about KVM and [`rust
ffi`](https://doc.rust-lang.org/stable/std/ffi/index.html).

The sources are structured as follows:
- [`src/`](./src) provides a small library as abstraction over the raw [KVM
  API](https://www.kernel.org/doc/html/latest/virt/kvm/api.html#api-description).
- [`examples/`](./examples) contains example VMs using the library above.
- [`guest/`](./guest) contains the guest source code.
- [`sysdeps/`](./sysdeps) helper to generate some KVM constants from the system
  header (executed by [build.rs](./build.rs)).

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
