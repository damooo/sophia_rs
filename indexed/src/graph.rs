//! A utility trait for building graphs using indexed terms.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use sophia_api::term::TTerm;
use sophia_term::*;

/// Symbols from other crates, re-exported for the sake of macros
pub mod reexport {
    pub use sophia_api::graph::MgResult;
    pub use sophia_api::triple::stream::{StreamResult, TripleSource};
    pub use sophia_api::triple::Triple;
}

/// A utility trait for implementing [`Graph`](sophia_api::graph::Graph)
/// and [`MutableGraph`](sophia_api::dataset::Dataset)
/// based on an internal [`TermIndexMap`](sophia_term::index_map::TermIndexMap)
/// for efficient storage.
///
/// The [`impl_mutable_graph_for_indexed_graph!`] macro
/// can be used to derive the `MutableGraph` implementation
/// for any implementation of `IndexedGraph`.
pub trait IndexedGraph {
    /// The type used to represent terms internally.
    type Index: Copy + Eq + Hash;

    /// The type used to hold the term data.
    type TermData: TermData + 'static;

    /// Construct a new empty graph, provisioning for storing `capacity` triples.
    fn with_capacity(capacity: usize) -> Self;

    /// Shrink the memory consumption of the graph as much as possible.
    fn shrink_to_fit(&mut self);

    /// Return the index for the given term, if it exists.
    fn get_index<T>(&self, t: &T) -> Option<Self::Index>
    where
        T: TTerm + ?Sized;

    /// Return the term for the given index, if it exists.
    fn get_term(&self, i: Self::Index) -> Option<&Term<Self::TermData>>;

    /// Insert a triple in this Graph,
    /// and return the corresponding tuple of indices.
    fn insert_indexed<TS, TP, TO>(&mut self, s: &TS, p: &TP, o: &TO) -> Option<[Self::Index; 3]>
    where
        TS: TTerm + ?Sized,
        TP: TTerm + ?Sized,
        TO: TTerm + ?Sized;
    /// Remove a triple from this Graph,
    /// and return the corresponding tuple of indices.
    fn remove_indexed<TS, TP, TO>(&mut self, s: &TS, p: &TP, o: &TO) -> Option<[Self::Index; 3]>
    where
        TS: TTerm + ?Sized,
        TP: TTerm + ?Sized,
        TO: TTerm + ?Sized;
}

/// Defines the implementation of [`CollectibleGraph`](sophia_api::graph::CollectibleGraph)
/// for [`IndexedGraph`].
#[macro_export]
macro_rules! impl_collectible_graph_for_indexed_graph {
    ($indexed_mutable_graph: ty) => {
        impl CollectibleGraph for $indexed_mutable_graph {
            impl_collectible_graph_for_indexed_graph!();
        }
    };
    () => {
        fn from_triple_source<TS: $crate::graph::reexport::TripleSource>(
            mut triples: TS,
        ) -> $crate::graph::reexport::StreamResult<Self, TS::Error, Self::Error> {
            use $crate::graph::reexport::Triple;
            let (tmin, tmax) = triples.size_hint_triples();
            let cap = tmax.unwrap_or(tmin);
            let mut g = Self::with_capacity(cap);
            triples
                .try_for_each_triple(|t| -> Result<(), Self::Error> {
                    g.insert_indexed(t.s(), t.p(), t.o());
                    Ok(())
                })
                .map(|_| g)
        }
    };
}

/// Defines the implementation of [`MutableGraph`](sophia_api::graph::MutableGraph)
/// for [`IndexedGraph`].
#[macro_export]
macro_rules! impl_mutable_graph_for_indexed_graph {
    ($indexed_mutable_graph: ty) => {
        impl MutableGraph for $indexed_mutable_graph {
            impl_mutable_graph_for_indexed_graph!();
        }
    };
    () => {
        type MutationError = std::convert::Infallible;

        fn insert<TS_, TP_, TO_>(
            &mut self,
            s: &TS_,
            p: &TP_,
            o: &TO_,
        ) -> $crate::graph::reexport::MgResult<Self, bool>
        where
            TS_: sophia_api::term::TTerm + ?Sized,
            TP_: sophia_api::term::TTerm + ?Sized,
            TO_: sophia_api::term::TTerm + ?Sized,
        {
            Ok(self.insert_indexed(s, p, o).is_some())
        }
        fn remove<TS_, TP_, TO_>(
            &mut self,
            s: &TS_,
            p: &TP_,
            o: &TO_,
        ) -> $crate::graph::reexport::MgResult<Self, bool>
        where
            TS_: sophia_api::term::TTerm + ?Sized,
            TP_: sophia_api::term::TTerm + ?Sized,
            TO_: sophia_api::term::TTerm + ?Sized,
        {
            Ok(self.remove_indexed(s, p, o).is_some())
        }
    };
}

/// Insert an absent value in the Vec value of a HashMap,
/// creating the Vec if it does not exist.
///
/// # Returns
///
/// `true` if the Vec was created,
///  meaning that "parent" indexes need to be updated.
///
pub fn insert_in_index<K, W>(hm: &mut HashMap<K, Vec<W>>, k: K, w: W) -> bool
where
    K: Eq + Hash,
    W: Copy + Eq,
{
    let mut ret = false;
    hm.entry(k)
        .or_insert_with(|| {
            ret = true;
            Vec::new()
        })
        .push(w);
    ret
}

/// Remove an existing value in the Vec value of a HashMap,
/// removing the entry completely if the Vec ends up empty.
///
/// # Returns
///
/// `true` if the entry was removed,
///  meaning that "parent" indexes need to be updated.
///
/// # Panics
///
/// This function will panic if either
/// * `k` is not a key of `hm`, or
/// * `w` is not contained in the value associated to `k`.
pub fn remove_from_index<K, W>(hm: &mut HashMap<K, Vec<W>>, k: K, w: W) -> bool
where
    K: Eq + Hash,
    W: Copy + Eq,
{
    match hm.entry(k) {
        Entry::Occupied(mut e) => {
            {
                let ws = e.get_mut();
                if ws.len() > 1 {
                    let wi = ws
                        .iter()
                        .enumerate()
                        .filter_map(|(i, w2)| if *w2 == w { Some(i) } else { None })
                        .next()
                        .unwrap();
                    ws.swap_remove(wi);
                    return false;
                }
            }
            e.remove_entry();
            true
        }
        Entry::Vacant(_) => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    // Nothing really worth testing here
}
