//! In-memory implementations of RDF graphs.
//!
//! This module provides [building blocks](#structs)
//! for defining implementations of [`Graph`] and [`MutableGraph`],
//! with fine-tuned trade-offs between memory footprint and performance.
//!
//! It also provides two pre-defined trade-offs:
//! [`FastGraph`] and [`LightGraph`],
//! provided in different flavors
//! ([default](#types), [`small`], [`sync`].
//!
//! # Customized trade-off
//!
//! By combining a given core implementation with various wrappers,
//! you can easily make a Graph type with the exact trade-offs that you need.
//!
//! For example, if one needs a small graph (less than 2^16 terms),
//! that can be exchanged across threads,
//! and whose arcs will mostly be traversed backward (from object to subject),
//! an appropriate type definition would be:
//!
//! ```
//! use sophia_term::factory::ArcTermFactory;
//! use sophia_inmem::graph::*;
//!
//! type MyGraph = OpsWrapper<GenericGraph<u16, ArcTermFactory>>;
//! let g = MyGraph::new();
//! ```

use sophia_api::graph::{CollectibleGraph, Graph, MutableGraph, SetGraph};
use sophia_indexed::graph::*;
use sophia_term::factory::*;
use sophia_term::*;

// Symbols from other crates, re-exported for the sake of macros
pub use sophia_api::graph::{GResult, GTerm, GTripleSource};

#[macro_use]
mod _wrapper;
pub use self::_wrapper::*;
mod _hash_graph;
pub use self::_hash_graph::*;
mod _spo_wrapper;
pub use self::_spo_wrapper::*;
mod _ops_wrapper;
pub use self::_ops_wrapper::*;
mod _term_index_map_u;
pub use self::_term_index_map_u::*;

/// A generic in-memory graph.
///
/// `I` must be a type for which [`TermIndexMapU`]
/// implements [`TermIndexMap`](sophia_term::index_map::TermIndexMap),
/// typically `u16` or `u32`.
///
/// `F` must implement [`TermFactory`].
///
pub type GenericGraph<I, F> = HashGraph<TermIndexMapU<I, F>>;

type FastWrapper<T> = OpsWrapper<SpoWrapper<T>>;

/// A heavily indexed graph.
/// Fast to query but slow to load, with a relatively high memory footprint.
pub type FastGraph = FastWrapper<GenericGraph<u32, RcTermFactory>>;

/// A graph with no triple index.
/// Fast to load but slow to query, with a relatively low memory footprint.
pub type LightGraph = GenericGraph<u32, RcTermFactory>;

#[cfg(test)]
sophia_api::test_graph_impl!(test_fastg, FastGraph);
#[cfg(all(test, feature = "all_tests"))]
sophia_api::test_graph_impl!(test_lightg, LightGraph);

/// Flavors of Graph implementations with a smaller memory-footprint.
///
/// The trade-off is that these implementations can only contain a small number (2^16) of terms.
///
pub mod small {
    use super::*;

    /// A heavily indexed graph.
    /// Fast to query but slow to load, with a relatively high memory footprint.
    pub type FastGraph = FastWrapper<GenericGraph<u16, RcTermFactory>>;
    /// A graph with no triple index.
    /// Fast to load but slow to query, with a relatively low memory footprint.
    pub type LightGraph = GenericGraph<u16, RcTermFactory>;

    #[cfg(all(test, feature = "all_tests"))]
    sophia_api::test_graph_impl!(test_fastg, FastGraph);
    #[cfg(all(test, feature = "all_tests"))]
    sophia_api::test_graph_impl!(test_lightg, LightGraph);
}

/// Flavors of Graph implementations which are safe to share across threads.
pub mod sync {
    use super::*;

    /// A heavily indexed graph.
    /// Fast to query but slow to load, with a relatively high memory footprint.
    pub type FastGraph = FastWrapper<GenericGraph<u32, ArcTermFactory>>;
    /// A graph with no triple index.
    /// Fast to load but slow to query, with a relatively low memory footprint.
    pub type LightGraph = GenericGraph<u32, ArcTermFactory>;

    #[cfg(all(test, feature = "all_tests"))]
    sophia_api::test_graph_impl!(test_fastg, FastGraph);
    #[cfg(all(test, feature = "all_tests"))]
    sophia_api::test_graph_impl!(test_lightg, LightGraph);
}
