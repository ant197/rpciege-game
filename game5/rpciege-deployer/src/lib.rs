#![no_std]

mod contract;
mod storage;
mod types;

pub mod liqpool {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/rpciege_game5_liquiditypool.wasm"
    );
}