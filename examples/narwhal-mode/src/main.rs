//! This example shows how to run a narwhal dev node programmatically.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy_genesis::Genesis;
use alloy_primitives::{address, Address, Bytes, U256};
use reth::{
    args::DevArgs,
    builder::{
        components::{ExecutorBuilder, PayloadServiceBuilder},
        BuilderContext, NodeBuilder,
    },
    tasks::TaskManager,
};
use reth_chainspec::{Chain, ChainSpec};

use reth_node_core::{args::RpcServerArgs, node_config::NodeConfig};
use reth_node_ethereum::{
    node::{EthereumAddOns},
    EthereumNode,
};
use reth_tracing::{RethTracer, Tracer};
use std::{sync::Arc};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _guard = RethTracer::new().init()?;
    let tasks = TaskManager::current();

    let spec = reth_chainspec::DEV.clone();
    let node_config =
        NodeConfig::test()
        .dev()
        .narwhal()
        .with_rpc(RpcServerArgs::default().with_http())
        .with_chain(spec);

    let handle = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .with_types::<EthereumNode>()
        .with_components(EthereumNode::components())
        .with_add_ons(EthereumAddOns::default())
        .launch()
        .await
        .unwrap();

        // let handle = NodeBuilder::new(node_config)
        // .testing_node(tasks.executor())
        // // configure the node with regular ethereum types
        // .with_types::<EthereumNode>()
        // // use default ethereum components but with our executor
        // .with_components(
        //     EthereumNode::components()
        //         .executor(MyExecutorBuilder::default())
        //         .payload(MyPayloadBuilder::default()),
        // )
        // .with_add_ons(EthereumAddOns::default())
        // .launch()
        // .await
        // .unwrap();
    println!("Node started");

    handle.node_exit_future.await
}

// TODO: remove this once we have a way to specify the chain spec in the node config
fn custom_chain() -> Arc<ChainSpec> {
    let custom_genesis = r#"
{
    "nonce": "0x42",
    "timestamp": "0x0",
    "extraData": "0x5343",
    "gasLimit": "0x1388",
    "difficulty": "0x400000000",
    "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "coinbase": "0x0000000000000000000000000000000000000000",
    "alloc": {
        "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b": {
            "balance": "0x4a47e3c12448f4ad000000"
        }
    },
    "number": "0x0",
    "gasUsed": "0x0",
    "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "config": {
        "ethash": {},
        "chainId": 2600,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0,
        "byzantiumBlock": 0,
        "constantinopleBlock": 0,
        "petersburgBlock": 0,
        "istanbulBlock": 0,
        "berlinBlock": 0,
        "londonBlock": 0,
        "terminalTotalDifficulty": 0,
        "terminalTotalDifficultyPassed": true,
        "shanghaiTime": 0
    }
}
"#;
    let genesis: Genesis = serde_json::from_str(custom_genesis).unwrap();
    Arc::new(genesis.into())
}
