I am cross-compiling from a Linux `x86_64` host to `aarch64-apple-darwin`.
Since Rust 1.87.0 this results in a broken `dylib` file when it gets stripped.
This worked ok until 1.86.0

Unfortunately I'm not able to reproduce that on a really small example, but a reproduction case is available in https://github.com/badboy/rocketscience/tree/broken-dylib

Note that this requires additional tooling:

* A clang that can build for that target. The Firefox CI has one like that.
* cctools that support that target, available in [cctools-port](https://github.com/tpoechtrager/cctools-port/), and ready built by Firefox CI.
* The macOS SDK. Available at https://github.com/phracker/MacOSX-SDKs/releases/tag/11.3

The included [`run.sh`](https://github.com/badboy/rocketscience/blob/broken-dylib/run.sh) downloads all requirements and sets `RUSTFLAGS` accordingly.
All that is required is:

```
./run.sh
```

`RUSTFLAGS` is this:

```
export RUSTFLAGS="-C linker=${clangdir}/bin/clang -C link-arg=-fuse-ld=${cctoolsdir}/bin/aarch64-apple-darwin-ld -C link-arg=-B -C link-arg=${cctoolsdir}/cctools/bin -C link-arg=-target -C link-arg=aarch64-apple-darwin -C link-arg=-isysroot -C link-arg=${sdkdir}/MacOSX11.0.sdk -C link-arg=-Wl,-syslibroot,${sdkdir}/MacOSX11.0.sdk -C link-arg=-Wl,-dead_strip"
```

The final `dylib` at `target/aarch64-apple-darwin/release/librocketscience.dylib` _should_ be inspectable by `aarch64-apple-darwin-nm`.
When it's not it will error out.

On Rust 1.87.0 and 1.89.0-nightly (99e7c15e8 2025-06-01) it fails:

```
$PWD/tools/cctools/bin/aarch64-apple-darwin-nm: object: target/aarch64-apple-darwin/release/librocketscience.dylib malformed object (section contents at offset 367912 with a size of 936, overlaps section contents at offset 32768 with a size of 338140)
```

On Rust 1.86.0 or when setting `strip = false` in `Cargo.toml` it works:

```
0000000000007be8 T _ffi_rocketscience_uniffi_contract_version
```

This is all due to the bundled `rust-objcopy`.
Manually running the `rust-objcopy` command on the not-broken dylib that `cargo` also runs produces the broken `dylib`:

```
~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-objcopy --strip-debug target/aarch64-apple-darwin/release/librocketscience.dylib
```

This does NOT happen when building on my aarch64 macOS machine, only when cross-compiling.

This all sounds very similar to #138212.
Back then it was an upstream bug, so this might be another one.
Or is this due to mixing and matching the toolchains in an incompatible way?

### Version it worked on

```
rustc 1.86.0 (05f9846f8 2025-03-31)
binary: rustc
commit-hash: 05f9846f893b09a1be1fc8560e33fc3c815cfecb
commit-date: 2025-03-31
host: x86_64-unknown-linux-gnu
release: 1.86.0
LLVM version: 19.1.7
```

### Version with regression

```
rustc 1.87.0 (17067e9ac 2025-05-09)
binary: rustc
commit-hash: 17067e9ac6d7ecb70e50f92c1944e545188d2359
commit-date: 2025-05-09
host: x86_64-unknown-linux-gnu
release: 1.87.0
LLVM version: 20.1.1
```
