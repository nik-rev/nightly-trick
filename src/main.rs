// see `lib.rs` for examples

fn main() {
    println!("try compiling with:");

    // uses nightly - allows us to use `#[feature]`
    println!("cargo test");

    // uses nightly rustfmt, allows using unstable options in `rust-toolchain.toml`
    println!("cargo fmt");

    // uses nightly clippy, allows using nightly lints
    println!("cargo clippy");

    // but compiles with our MSRV!
    println!("cargo +1.72.0 run");
}
