# bitcoin-rust-etude
my bitcoin and rust study code

# Compile and Run
```
$ cargo build
$ cargo run
```

This software connect to 127.0.0.1:48333(hard corded in main.rs).
You need to setup bitcoin server listen on it.

bitcoin-core conf file seems like:

```
testnet=3
bind=127.0.0.1:48333
addnode=69.164.218.197
listen=1
```



