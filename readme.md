# `libtor` example

Uses [libtor] crate to run a Tor daemon in process.

This example spawns the Tor daemon using Tokio's `spawn_blocking` and then spawn other work tasks to print dots, and to make a web request to `check.torproject.org` via the exposed socks proxy

## How to run

You'll need Rust and cargo

```bash
cargo run
```

Hit `ctrl+C` twice to quit

## Example Output

```
Running `target/debug/tor`
---poll
.Nov 26 12:29:19.077 [notice] Tor 0.4.6.7 running on Darwin with Libevent 2.1.12-stable, OpenSSL 1.1.1l, Zlib 1.2.11, Liblzma N/A, Libzstd N/A and Unknown N/A as libc.
Nov 26 12:29:19.077 [notice] Tor can't help you if you use it wrong! Learn how to be safe at https://www.torproject.org/download/download#warning
Nov 26 12:29:19.078 [notice] Configuration file "/Users/byron/.torrc" not present, using reasonable defaults.
Nov 26 12:29:19.080 [notice] Opening Socks listener on 127.0.0.1:19050
Nov 26 12:29:19.080 [notice] Opened Socks listener connection (ready) on 127.0.0.1:19050
Nov 26 12:29:19.000 [notice] Parsing GEOIP IPv4 file /Users/byron/code/tor/target/debug/build/libtor-sys-188d233741242f4c/out/share/tor/geoip.
.Nov 26 12:29:19.000 [notice] Parsing GEOIP IPv6 file /Users/byron/code/tor/target/debug/build/libtor-sys-188d233741242f4c/out/share/tor/geoip6.
.Nov 26 12:29:19.000 [notice] Bootstrapped 0% (starting): Starting
..---request
........Nov 26 12:29:20.000 [notice] Starting with guard context "default"
Nov 26 12:29:20.000 [notice] Bootstrapped 5% (conn): Connecting to a relay
..Nov 26 12:29:20.000 [notice] Bootstrapped 10% (conn_done): Connected to a relay
...Nov 26 12:29:20.000 [notice] Bootstrapped 14% (handshake): Handshaking with a relay
........Nov 26 12:29:21.000 [notice] Bootstrapped 15% (handshake_done): Handshake with a relay done
Nov 26 12:29:21.000 [notice] Bootstrapped 75% (enough_dirinfo): Loaded enough directory info to build circuits
Nov 26 12:29:21.000 [notice] Bootstrapped 90% (ap_handshake_done): Handshake finished with a relay to build circuits
Nov 26 12:29:21.000 [notice] Bootstrapped 95% (circuit_create): Establishing a Tor circuit
.........Nov 26 12:29:22.000 [notice] Bootstrapped 100% (done): Done
...................Tor is online!
..........................^CNov 26 12:29:27.000 [notice] Interrupt: exiting cleanly.
.tor exit result: 0
........^Creceived Ctrl+C!
received shutdown signal
```

[libtor]: https://github.com/MagicalBitcoin/libtor
