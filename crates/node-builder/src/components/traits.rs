//! Traits for the builder

use crate::{
    components::NodeComponents, node::FullNodeTypes, BuilderContext, ComponentsState,
    FullNodeTypesAdapter, Node, NodeBuilder, NodeHandle, RethFullAdapter, RethFullProviderType,
    TypesState,
};
use futures::Future;
use reth_db::{
    database::Database,
    database_metrics::{DatabaseMetadata, DatabaseMetrics},
};
use reth_network::NetworkHandle;
use reth_node_api::NodeTypes;
use reth_node_core::dirs::{ChainPath, DataDirPath};
use reth_payload_builder::PayloadBuilderHandle;
use reth_tasks::TaskExecutor;
use reth_transaction_pool::TransactionPool;

use super::PoolBuilder;

/// Encapsulates all types and components of the node.
pub trait FullNodeComponents: FullNodeTypes + 'static {
    /// The transaction pool of the node.
    type Pool: TransactionPool;

    /// Returns the transaction pool of the node.
    fn pool(&self) -> &Self::Pool;

    /// Returns the provider of the node.
    fn provider(&self) -> &Self::Provider;

    /// Returns the handle to the network
    fn network(&self) -> &NetworkHandle;

    /// Returns the handle to the payload builder service.
    fn payload_builder(&self) -> &PayloadBuilderHandle<Self::Engine>;

    /// Returns the task executor.
    fn task_executor(&self) -> &TaskExecutor;
}

/// A type that encapsulates all the components of the node.
#[derive(Debug)]
pub struct FullNodeComponentsAdapter<Node: FullNodeTypes, Pool> {
    pub(crate) evm_config: Node::Evm,
    pub(crate) pool: Pool,
    pub(crate) network: NetworkHandle,
    pub(crate) provider: Node::Provider,
    pub(crate) payload_builder: PayloadBuilderHandle<Node::Engine>,
    pub(crate) executor: TaskExecutor,
}

impl<Node, Pool> FullNodeTypes for FullNodeComponentsAdapter<Node, Pool>
where
    Node: FullNodeTypes,
    Pool: TransactionPool + 'static,
{
    type DB = Node::DB;
    type Provider = Node::Provider;
}

impl<Node, Pool> NodeTypes for FullNodeComponentsAdapter<Node, Pool>
where
    Node: FullNodeTypes,
    Pool: TransactionPool + 'static,
{
    type Primitives = Node::Primitives;
    type Engine = Node::Engine;
    type Evm = Node::Evm;

    fn evm_config(&self) -> Self::Evm {
        self.evm_config.clone()
    }
}

impl<Node, Pool> FullNodeComponents for FullNodeComponentsAdapter<Node, Pool>
where
    Node: FullNodeTypes,
    Pool: TransactionPool + 'static,
{
    type Pool = Pool;

    fn pool(&self) -> &Self::Pool {
        &self.pool
    }

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn network(&self) -> &NetworkHandle {
        &self.network
    }

    fn payload_builder(&self) -> &PayloadBuilderHandle<Self::Engine> {
        &self.payload_builder
    }

    fn task_executor(&self) -> &TaskExecutor {
        &self.executor
    }
}

impl<Node: FullNodeTypes, Pool> Clone for FullNodeComponentsAdapter<Node, Pool>
where
    Pool: Clone,
{
    fn clone(&self) -> Self {
        Self {
            evm_config: self.evm_config.clone(),
            pool: self.pool.clone(),
            network: self.network.clone(),
            provider: self.provider.clone(),
            payload_builder: self.payload_builder.clone(),
            executor: self.executor.clone(),
        }
    }
}

/// A type that configures all the customizable components of the node and knows how to build them.
///
/// Implementors of this trait are responsible for building all the components of the node: See
/// [NodeComponents].
///
/// The [ComponentsBuilder](crate::components::builder::ComponentsBuilder) is a generic
/// implementation of this trait that can be used to customize certain components of the node using
/// the builder pattern and defaults, e.g. Ethereum and Optimism.
pub trait NodeComponentsBuilder<Node: FullNodeTypes> {
    /// The transaction pool to use.
    type Pool: TransactionPool + Unpin + 'static;

    /// Builds the components of the node.
    fn build_components(
        self,
        context: &BuilderContext<Node>,
    ) -> impl std::future::Future<Output = eyre::Result<NodeComponents<Node, Self::Pool>>> + Send;
}

impl<Node, F, Fut, Pool> NodeComponentsBuilder<Node> for F
where
    Node: FullNodeTypes,
    F: FnOnce(&BuilderContext<Node>) -> Fut + Send,
    Fut: std::future::Future<Output = eyre::Result<NodeComponents<Node, Pool>>> + Send,
    Pool: TransactionPool + Unpin + 'static,
{
    type Pool = Pool;

    fn build_components(
        self,
        ctx: &BuilderContext<Node>,
    ) -> impl std::future::Future<Output = eyre::Result<NodeComponents<Node, Self::Pool>>> + Send
    {
        self(ctx)
    }
}

/// Trait for launching the node.
pub trait LaunchNode<DB, Types, Components>
where
    DB: Database + DatabaseMetrics + DatabaseMetadata + Clone + Unpin + 'static,
    Types:
        Node<FullNodeTypesAdapter<Types, DB, RethFullProviderType<DB, <Types as NodeTypes>::Evm>>>,
    Types::PoolBuilder: PoolBuilder<RethFullAdapter<DB, Types>>,
    Types::NetworkBuilder: crate::components::NetworkBuilder<
        RethFullAdapter<DB, Types>,
        <Types::PoolBuilder as PoolBuilder<RethFullAdapter<DB, Types>>>::Pool,
    >,
    Types::PayloadBuilder: crate::components::PayloadServiceBuilder<
        RethFullAdapter<DB, Types>,
        <Types::PoolBuilder as PoolBuilder<RethFullAdapter<DB, Types>>>::Pool,
    >,
    // Types: NodeTypes,
    Components: NodeComponentsBuilder<
        FullNodeTypesAdapter<Types, DB, RethFullProviderType<DB, Types::Evm>>,
    >,
{
    /// Launches the node and returns a handle to it.
    ///
    /// Returns a [NodeHandle] that can be used to interact with the node.
    fn launch(
        builder: NodeBuilder<
            DB,
            ComponentsState<
                Types,
                Components,
                FullNodeComponentsAdapter<
                    FullNodeTypesAdapter<Types, DB, RethFullProviderType<DB, Types::Evm>>,
                    Components::Pool,
                >,
            >,
        >,
        executor: TaskExecutor,
        data_dir: ChainPath<DataDirPath>,
    ) -> impl Future<
        Output = eyre::Result<
            NodeHandle<
                FullNodeComponentsAdapter<
                    RethFullAdapter<DB, Types>,
                    <Types::PoolBuilder as PoolBuilder<RethFullAdapter<DB, Types>>>::Pool,
                >,
            >,
        >,
    >;
}

// /// Trait for launching the node.
// pub trait LaunchNode<N, DB, Types, Components>
// where
//     DB: Database + DatabaseMetrics + DatabaseMetadata + Clone + Unpin + 'static,
//     N: Node<FullNodeTypesAdapter<N, DB, RethFullProviderType<DB, <N as NodeTypes>::Evm>>>,
//     N::PoolBuilder: PoolBuilder<RethFullAdapter<DB, N>>,
//     N::NetworkBuilder: crate::components::NetworkBuilder<
//         RethFullAdapter<DB, N>,
//         <N::PoolBuilder as PoolBuilder<RethFullAdapter<DB, N>>>::Pool,
//     >,
//     N::PayloadBuilder: crate::components::PayloadServiceBuilder<
//         RethFullAdapter<DB, N>,
//         <N::PoolBuilder as PoolBuilder<RethFullAdapter<DB, N>>>::Pool,
//     >,
//     Types: NodeTypes,
//     Components: NodeComponentsBuilder<
//         FullNodeTypesAdapter<Types, DB, RethFullProviderType<DB, Types::Evm>>,
//     >,
// {
//     /// Launches the node and returns a handle to it.
//     ///
//     /// Returns a [NodeHandle] that can be used to interact with the node.
//     fn launch(
//         builder: NodeBuilder<
//             DB,
//             ComponentsState<
//                 Types,
//                 Components,
//                 FullNodeComponentsAdapter<
//                     FullNodeTypesAdapter<Types, DB, RethFullProviderType<DB, Types::Evm>>,
//                     Components::Pool,
//                 >,
//             >,
//         >,
//         executor: TaskExecutor,
//         data_dir: ChainPath<DataDirPath>,
//     ) -> impl Future<
//         Output = eyre::Result<
//             NodeHandle<
//                 FullNodeComponentsAdapter<
//                     RethFullAdapter<DB, N>,
//                     <N::PoolBuilder as PoolBuilder<RethFullAdapter<DB, N>>>::Pool,
//                 >,
//             >,
//         >,
//     >;
// }
