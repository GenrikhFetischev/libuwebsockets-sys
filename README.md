# Rust sys crate for [uWebSockets CAPI](https://github.com/uNetworking/uWebSockets)

## Usage

In order to use uWebSockets in your Rust application you will have to install and link the following static libraries to you
binary - `libz`, `libuv`, `libssl`, `libcrypto` and `libstdc++`.

It may look something like that in your build.rs file:

```rust
println!("cargo:rustc-link-lib=z");
println!("cargo:rustc-link-lib=uv");
println!("cargo:rustc-link-lib=ssl");
println!("cargo:rustc-link-lib=crypto");
println!("cargo:rustc-link-lib=stdc++");
```

## Development
Make sure to fetch submodules - uWebSockets and uSockets. You probably don't want to fetch all submodules with --recursive option unless you want to. 
