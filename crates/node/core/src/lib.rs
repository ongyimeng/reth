//! The core of the Ethereum node. Collection of utilities and libraries that are used by the node.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod args;
pub mod cli;
pub mod dirs;
pub mod exit;
pub mod node_config;
pub mod utils;
pub mod version;

/// Re-exported primitive types
pub mod primitives {
    pub use reth_ethereum_forks::*;
    pub use reth_primitives_traits::*;
}

/// Re-export of `reth_rpc_*` crates.
pub mod rpc {
    /// Re-exported from `reth_rpc::rpc`.
    pub mod result {
        pub use reth_rpc_server_types::result::*;
    }

    /// Re-exported from `reth_rpc::eth`.
    pub mod compat {
        pub use reth_rpc_convert::*;
    }
}
