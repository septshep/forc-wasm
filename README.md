# forc-wasm

WASM implementation of [Forc](https://docs.fuel.network/docs/forc/) (Fuel
Orchestrator) for creating, building, testing, and deploying
[Sway](https://docs.fuel.network/docs/sway/) projects directly in the browser.
Inspired by [WASM-Cairo](https://github.com/cryptonerdcn/wasm-cairo) and
[aptos-move-js](https://github.com/wangeguo/aptos-move-js).

## Installation

```bash
npm install --save forc-wasm
```

## Usage

### Building the contract

```javascript
import * as forc from 'forc-wasm';

// the contract source code
const code = `
    contract;

    abi MyContract {
        fn test_function() -> bool;
    }

    impl MyContract for Contract {
        fn test_function() -> bool {
            true
        }
    }`;

// build the contract
var params = new forc.BuildParams(code, 'testnet');
var result = forc.build(params);
```

The result of `forc.build` is an object with the following fields, if compile
failed, `result.error` will be set.

```json
{
    "abi": "{}",
    "bytecode": "0x1af0...0c74",
    "storageSlots": "[]",
    "forcVersion": "forc 0.60.0",
    "error": "",
}
```

### Deploying the contract

```javascript
var result = await forc.deploy({
    // the bytecode of the contract
    bytecode: '0x1af0...0c74',
    // the address to deploy the contract to
    address: 'fuel18caanqmumttfnm8qp0eq7u9yluydxtqmzuaqtzdjlsww5t2jmg9skutn8n',
    // the toolchain to use
    toolchain: 'testnet',
});
```

Before deploying the contract, ensure you have a Fuel wallet installed in your
browser and acquire test funds using the
[faucet](https://faucet-testnet.fuel.network/). After deployment, you will
receive details including the Contract ID and the block where the transaction
was signed. For further information, refer to the '[Setting up a local
wallet](https://docs.fuel.network/guides/contract-quickstart/#setting-up-a-local-wallet)'
section.

## Development

The WASM in this package is generated by Rust living in the `src` directory. You
need Rust tooling to work on it. We use
[wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) and
[wasm-pack](https://rustwasm.github.io/docs/wasm-pack/). To build `forc-wasm`
you will need to install the dependencies:

- Git https://git-scm.com/downloads
- Rust https://www.rust-lang.org/tools/install
- wasm-pack https://rustwasm.github.io/wasm-pack/installer/
- Just https://github.com/casey/just

See our setup for complete details, but after the typical `git clone` it's as
simple as running `just` in the package directory to lists all the available
commands.

## License

Licensed under the Apache License, Version 2.0, see the [LICENSE](LICENSE) file
for more information.
