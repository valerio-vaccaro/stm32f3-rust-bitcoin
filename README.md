You need [this board](https://rust-embedded.github.io/book/intro/hardware.html).

Follow setup instructions for [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/install.html).

In one terminal run `openocd`.

In another terminal run `cargo run --release`.

This should drop your second terminal into a gbd debugging session. Type `continue` and hit enter, twice.

In the first terminal you should see the following:

```
Seed WIF: L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D
Address: bc1qpx9t9pzzl4qsydmhyt6ctrxxjd4ep549np9993
```

[rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) works on microcontrollers!

If you'd like to learn more, check out the [embedded-rust book](https://rust-embedded.github.io/book/intro/index.html). This demo is based on examples from the book and uses the same board.