# patlite-rs
A Rust-implemented driver for PATLITE LR6-USB Signal Tower

This library depends on [hidapi](https://crates.io/crates/hidapi) crate, and it depends on C-library [hidapi](https://github.com/libusb/hidapi). Therefore, you need to install hidapi.

If you use Ubuntu, you can install easily with `apt`:

```bash
sudo apt install libhidapi-dev
```


You can try an example with the following command:

```bash
cargo run --example example
```
