# CW1155 Basic

This is a basic implementation of a CW1155 multiple tokens contract. It 
implements the [CW1155 spec](../../packages/cw1155/README.md) and is designed 
to be deployed as is, or imported into other contracts to easily build 
CW1155-compatible NFTs with custom logic.

Implements:

- [x] CW1155 Base
- [x] Metadata extension

## Running this contract

You will need Rust 1.65+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via:

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1155_base.wasm .
ls -l cw1155_base.wasm
sha256sum cw1155_base.wasm
```

Or for a production-ready (optimized) build, run a build command in the
repository root: https://github.com/CosmWasm/cw-ntfs#compiling.

*HINT: Take a look at the Makefile.toml.*

## Importing this contract

You can also import much of the logic of this contract to build another
CW1155-compliant contract.

Basically, you just need to write your handle function and import
`cw1155_base::contract::handle_transfer`, etc and dispatch to them.
This allows you to use custom `ExecuteMsg` and `QueryMsg` with your additional
calls, but then use the underlying implementation for the standard CW1155
messages you want to support. The same with `QueryMsg`. You will most likely 
want to write a custom, domain-specific `instantiate`.
