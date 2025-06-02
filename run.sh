#!/bin/bash

set -eu

# TOOL SETUP
mkdir -p tools
pushd tools

if [ ! -d "cctools" ]; then
  curl -sfSL --retry 5 --retry-delay 10 \
      https://firefox-ci-tc.services.mozilla.com/api/index/v1/task/gecko.cache.level-3.toolchains.v3.linux64-cctools-port.pushdate.2024.07.23.20240723071212/artifacts/public%2Fbuild%2Fcctools.tar.zst > cctools.tar.zst
  tar -I zstd -xf cctools.tar.zst
  rm cctools.tar.zst
fi

if [ ! -d "clang" ]; then
  curl -sfSL --retry 5 --retry-delay 10 \
      https://firefox-ci-tc.services.mozilla.com/api/index/v1/task/gecko.cache.level-3.toolchains.v3.clang-dist-toolchain.pushdate.2024.07.30.20240730145721/artifacts/public%2Fbuild%2Fclang-dist-toolchain.tar.xz > clang-dist-toolchain.tar.xz
  tar -xf clang-dist-toolchain.tar.xz
  mv builds/worker/toolchains/clang clang
  rm clang-dist-toolchain.tar.xz

  # Fixup symlink
  rm clang/bin/clang
  ln -s $(pwd)/clang/bin/clang-* $(pwd)/clang/bin/clang
fi

if [ ! -d "MacOSX11.0.sdk" ]; then
  curl -sfSL --retry 5 --retry-delay 10 \
    https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.0.sdk.tar.xz > MacOSX11.0.sdk.tar.gz

  tar -xf MacOSX11.0.sdk.tar.gz
  rm MacOSX11.0.sdk.tar.gz
fi

popd

clangdir=$(pwd)/tools/clang
cctoolsdir=$(pwd)/tools/cctools
sdkdir=$(pwd)/tools
export RUSTFLAGS="-C linker=${clangdir}/bin/clang -C link-arg=-fuse-ld=${cctoolsdir}/bin/aarch64-apple-darwin-ld -C link-arg=-B -C link-arg=${cctoolsdir}/cctools/bin -C link-arg=-target -C link-arg=aarch64-apple-darwin -C link-arg=-isysroot -C link-arg=${sdkdir}/MacOSX11.0.sdk -C link-arg=-Wl,-syslibroot,${sdkdir}/MacOSX11.0.sdk -C link-arg=-Wl,-dead_strip"

rustup target add aarch64-apple-darwin
cargo build --release --target=aarch64-apple-darwin --locked

nm=${cctoolsdir}/bin/aarch64-apple-darwin-nm
$nm target/aarch64-apple-darwin/release/librocketscience.dylib | grep contract
