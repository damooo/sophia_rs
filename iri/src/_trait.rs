//! IRI related traits.
use super::{Iri, IriRef};
use std::borrow::Borrow;

/// Marker trait guaranteeing that the underlying `str` is a valid IRI
/// (i.e. absolute or relative, with an optional fragment identifier)
pub trait IsIriRef: Borrow<str> {}

/// Marker trait guaranteeing that the underlying `str` is a valid IRI-reference
/// (i.e. absolute, with an optional fragment identifier)
pub trait IsIri: IsIriRef {}

/// Automatic trait for [`IsIriRef`], providing cheap conversion to [`IriRef`].
pub trait AsIriRef {
    /// Extract an [`IriRef`] wrapping the underlying `str`.
    fn as_iri_ref(&self) -> IriRef;
}

impl<T: IsIriRef> AsIriRef for T {
    fn as_iri_ref(&self) -> IriRef {
        IriRef::new_unchecked(self.borrow())
    }
}

/// Automatic trait for [`IsIri`], providing cheap conversion to [`Iri`].
pub trait AsIri {
    /// Extract an [`Iri`] wrapping the underlying `str`.
    fn as_iri(&self) -> Iri;
}

impl<T: IsIri> AsIri for T {
    fn as_iri(&self) -> Iri {
        Iri::new_unchecked(self.borrow())
    }
}
