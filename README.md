# kvm-rs

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

```bash
# Once: Build the guest binary image.
make -C guest

# Run the Real Mode example.
cargo run --example real_mode
```

## License
This project is licensed under the [MIT](LICENSE) license.
