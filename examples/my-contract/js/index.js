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

console.log(result);
