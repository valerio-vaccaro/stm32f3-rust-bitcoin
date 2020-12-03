You need [this board](https://rust-embedded.github.io/book/intro/hardware.html).

Follow setup instructions for [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/install.html).

In one terminal run `openocd`.

In another terminal run `cargo run`.

This should drop your second terminal into a gbd debugging session. Type `continue` and hit enter, twice.

In the first terminal you should see `Seed WIF: L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D`. This 