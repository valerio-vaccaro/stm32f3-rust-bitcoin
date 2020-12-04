This demo works with either [the F3 board used by embedded-rust book](https://rust-embedded.github.io/book/intro/hardware.html) or [the F469 board used by Specter DIY](https://github.com/cryptoadvance/specter-diy/blob/master/docs/shopping.md#discovery-board).

In order to run the demo, first follow setup instructions for [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/install.html).

[The allocator we're using requires the nightly compiler](https://github.com/rust-embedded/alloc-cortex-m/blob/4673f9324233cce93473068e74dc97fa62775367/src/lib.rs#L3). Run the following within this directory:

```
rustup override set nightly
```

You'll need two terminals.

### Terminal 1:

If you're using the F3 board:

```
openocd -f f3.openocd.cfg
```

If you're using the F469 board:

```
openocd -f f469.openocd.cfg
```

### Terminal 2

If you're using F3 board:

```
cargo run --release
```

If you're using F469 board:

```
X=f469.x cargo run --release
```

This should drop "terminal 2" into a gbd debugging session. Type `continue` and hit enter, twice.

In "terminal 1" you should see the following:

```
Seed WIF: L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D
Address: bc1qpx9t9pzzl4qsydmhyt6ctrxxjd4ep549np9993
```

If you'd like to learn more, check out the [embedded-rust book](https://rust-embedded.github.io/book/intro/index.html) which this example is based on.
