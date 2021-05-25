use std::env;
use std::path::Path;
use std::io::Write;

fn main() {
    // Generate KVM constants from the system header <linux/kvm.h>.

    // Only re-run if the generation source file changed.
    println!("cargo:rerun-if-changed=sysdeps/kvm.c");

    // Input directory.
    let sysdeps = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("sysdeps");

    // Output file.
    let kvm_constants = Path::new(&env::var("OUT_DIR").unwrap()).join("kvm_constants.rs");

    // Run make to generate the output file.
    let o = std::process::Command::new("make")
        .arg("-C")
        .arg(sysdeps.to_str().unwrap())
        .arg(format!("OUT={}", kvm_constants.display()))
        .output()
        .unwrap();
    std::io::stdout().write_all(&o.stdout).unwrap();
}
