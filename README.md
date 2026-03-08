# Secure Storage
Secure storage is a rust library that adds key-value storage with encryption

> ⚠️ Experimental
> 
> **This library is still in experimental phase!**

## How to use

- ### As Rust Crate
```sh
cargo add secure_storage --git https://github.com/reiyuchan/secure-storage-rs.git
```

```rs
use secure_storage::SecureStorage;
use anyhow::Result;

fn main() -> Result<()> {
    let db = SecureStorage::new("db")?;
        db.set("my_key","my_value".as_bytes())?;
    let value: Vec<u8> = db.get("my_key")?;
    let str = std::String::from_utf8(value)?;
        println!("{}",str);
}
```

- ### Binding generation

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
