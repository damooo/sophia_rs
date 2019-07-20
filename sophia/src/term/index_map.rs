//! A trait for bidirectional mappings between terms and *indexes* of a smaller type.

use crate::term::factory::{FTerm, TermFactory};
use crate::term::RefTerm;

/// A bidirectionnal mapping between [`Term`]s and *indexes* of a smaller type.
///
/// The TermIndexMap maintains a reference count for each index,
/// to automatically free them whenever they are not used.
///
/// One special index (called the *null index*) is never mapped to any [`Term`].
///
/// [`Term`]: ../../term/enum.Term.html
///
pub trait TermIndexMap: Default {
    /// The type used to represent terms
    type Index: Copy + Eq;
    /// The factory used to instantiate terms.
    type Factory: TermFactory;

    // A reserved index representing no term
    const NULL_INDEX: Self::Index;

    /// Return the index associated to the given term, if it exists.
    fn get_index(&self, t: &RefTerm) -> Option<Self::Index>;
    /// Return the index associated to the given term, creating it if required, and increasing its ref count.
    fn make_index(&mut self, t: &RefTerm) -> Self::Index;
    /// Return the term associated to the given index, if it exists.
    fn get_term(&self, i: Self::Index) -> Option<&FTerm<Self::Factory>>;
    /// Increase the reference count of a given index (or do nothing if i is the null index).
    fn inc_ref(&mut self, i: Self::Index);
    /// Decrease the reference count of a given index (or do nothing if i is the null index).
    fn dec_ref(&mut self, i: Self::Index);
    /// Shrinks the capacity of the TermIndexMap as much as possible.
    fn shrink_to_fit(&mut self);
}

#[cfg(test)]
/// Takes an empty TermIndexMap, and checks that it behaves as expected.
pub fn assert_term_index_map_works<T: TermIndexMap>(ti: &mut T) {
    let t = RefTerm::new_iri("http://example.org/").unwrap();
    assert!(ti.get_index(&t).is_none());

    // insert term, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert term twice, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    let it2 = ti.make_index(&t);
    assert!(it == it2);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert term, incref it, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.inc_ref(it);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert two terms, then remove them

    let t1 = t;
    let t2 = RefTerm::new_iri("http://example.org/2").unwrap();
    let it1 = ti.make_index(&t1);
    let it2 = ti.make_index(&t2);
    assert!(it1 != it2);

    ti.dec_ref(it2);
    assert!(ti.get_index(&t1).is_some());
    assert!(ti.get_index(&t2).is_none());

    ti.dec_ref(it1);
    assert!(ti.get_index(&t1).is_none());
    assert!(ti.get_index(&t2).is_none());
}

#[cfg(test)]
mod test {
    // Nothing really worth testing here
}