// this module is transparently re-exported by its parent `graph`
// It defines implementation of Graph and MutableGraph for existing types.

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

use super::*;
use ::error::Never;
use ::streams::WrapAsOks;
use ::term::*;
use ::triple::*;


impl<'a, T> GraphBase for [T] where
    T: Triple<'a>+'a,
{
    type Error = Never;
}

impl<'a, T> Graph<'a> for [T] where
    T: Triple<'a>+'a,
{
    type Triple = &'a T;

    #[inline]
    fn iter(&'a self) -> GFallibleTripleIterator<Self> {
        Box::new(
            <[T]>::iter(self).wrap_as_oks()
        )
    }
}



impl<'a, T> GraphBase for Vec<T> where
    T: Triple<'a>+'a,
{
    type Error = Never;
}

impl<'a, T> Graph<'a> for Vec<T> where
    T: Triple<'a>+'a,
{
    type Triple = &'a T;

    #[inline]
    fn iter(&'a self) -> GFallibleTripleIterator<Self> {
        Box::from(
            <[T]>::iter(self).wrap_as_oks()
        )
    }
}



impl<'a, T> GraphBase for HashSet<T> where
    T: Eq + Hash + Triple<'a> + 'a,
{
    type Error = Never;
}

impl<'a, T> Graph<'a> for HashSet<T> where
    T: Eq + Hash + Triple<'a> + 'a,
{
    type Triple = &'a T;

    #[inline]
    fn iter(&'a self) -> GFallibleTripleIterator<Self> {
        Box::from(self.iter().wrap_as_oks())
    }
}

impl MutableGraph for HashSet<[BoxTerm;3]> where
{
    fn insert<T, U, V> (&mut self, s: &Term<T>, p: &Term<U>, o: &Term<V>) -> Result<bool, Never> where
        T: Borrow<str>,
        U: Borrow<str>,
        V: Borrow<str>,
    {
        let s = BoxTerm::from(s);
        let p = BoxTerm::from(p);
        let o = BoxTerm::from(o);
        Ok(HashSet::insert(self, [s, p, o]))
    }
    fn remove<T, U, V> (&mut self, s: &Term<T>, p: &Term<U>, o: &Term<V>) -> Result<bool, Never> where
        T: Borrow<str>,
        U: Borrow<str>,
        V: Borrow<str>,
    {
        let s = BoxTerm::from(s);
        let p = BoxTerm::from(p);
        let o = BoxTerm::from(o);
        Ok(HashSet::remove(self, &[s, p, o]))
    }
}

impl<'a, T> SetGraph for HashSet<T> where
    T: Eq + Hash + Triple<'a> + 'a,
{}



#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use resiter::oks::*;

    use ::graph::*;
    use ::ns::*;
    use ::term::BoxTerm;

    #[test]
    fn test_slice() {
        let g = [
            [rdf::type_, rdf::type_, rdf::Property],
            [rdf::Property, rdf::type_, rdfs::Class],
            [rdfs::Class, rdf::type_, rdfs::Class],
        ];
        let v: Vec<_> = <[_] as Graph>::iter(&g).oks().collect();
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_vec() {
        let g = vec![
            [rdf::type_, rdf::type_, rdf::Property],
            [rdf::Property, rdf::type_, rdfs::Class],
            [rdfs::Class, rdf::type_, rdfs::Class],
        ];
        let v: Vec<_> = <Vec<_> as Graph>::iter(&g).oks().collect();
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_hashset() {
        let mut g1: HashSet<[BoxTerm;3]> = HashSet::new();

        let g2 = [
            [rdf::type_, rdf::type_, rdf::Property],
            [rdf::Property, rdf::type_, rdfs::Class],
            [rdfs::Class, rdf::type_, rdfs::Class],
        ];
        let inserted = g1.insert_all(<[_] as Graph>::iter(&g2)).unwrap();
        assert_eq!(inserted, g2.len());
        let v: Vec<_> = <HashSet<_> as Graph>::iter(&g1).oks().collect();
        assert_eq!(v.len(), 3);
    }
}