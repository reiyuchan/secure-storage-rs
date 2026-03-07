# Secure Storage
Secure storage is a rust library that adds key-value storage with encryption

## How to use
First clone the repo
```sh
git clone https://github.com/reiyuchan/secure-storage-rs.git
```
Build the library in release mode
```sh
cargo build --release
```
To generate bindings for swift
```sh
cargo run --bin uniffi-bindgen generate --library target/release/libsecure_storage.a --language swift --out-dir out 
```
To generate bindings for kotlin
```sh
cargo run --bin uniffi-bindgen generate --library target/release/libsecure_storage.so --language kotlin --out-dir out 
```
