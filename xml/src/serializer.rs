//! Serializer for the [RDF/XML] concrete syntax of RDF.
//! based on [`rio_xml`].
//!
//! **Important**:
//! the methods in this module accepting a [`Write`]
//! make no effort to minimize the number of write operations.
//! Hence, in most cased, they should be passed a [`BufWriter`].
//!
//! [RDF/XML]: https://www.w3.org/TR/rdf-syntax-grammar/
//! [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html

use rio_xml::RdfXmlFormatter;
use sophia_api::serializer::*;
use sophia_api::triple::stream::{SinkError, StreamResult, TripleSource};
use sophia_rio::serializer::rio_format_triples;
use std::io;

/// RDF/XML serializer configuration.
#[derive(Clone, Debug, Default)]
pub struct RdfXmlConfig {}

impl RdfXmlConfig {
    // TODO add ways to customize namespaces
}

/// RDF/XML serializer.
pub struct RdfXmlSerializer<W> {
    config: RdfXmlConfig,
    write: W,
}

impl<W> RdfXmlSerializer<W>
where
    W: io::Write,
{
    /// Build a new N-Triples serializer writing to `write`, with the default config.
    #[inline]
    pub fn new(write: W) -> RdfXmlSerializer<W> {
        Self::new_with_config(write, RdfXmlConfig::default())
    }

    /// Build a new N-Triples serializer writing to `write`, with the given config.
    pub fn new_with_config(write: W, config: RdfXmlConfig) -> RdfXmlSerializer<W> {
        RdfXmlSerializer { config, write }
    }

    /// Borrow this serializer's configuration.
    pub fn config(&self) -> &RdfXmlConfig {
        &self.config
    }
}

impl<W> TripleSerializer for RdfXmlSerializer<W>
where
    W: io::Write,
{
    type Error = io::Error;

    fn serialize_triples<TS>(
        &mut self,
        source: TS,
    ) -> StreamResult<&mut Self, TS::Error, Self::Error>
    where
        TS: TripleSource,
    {
        // temporarily move out self.write
        let mut tf = RdfXmlFormatter::new(&mut self.write).map_err(SinkError)?;
        rio_format_triples(&mut tf, source)?;
        tf.finish().map_err(SinkError)?;
        Ok(self)
    }
}

impl RdfXmlSerializer<Vec<u8>> {
    /// Create a new serializer which targets a `String`.
    #[inline]
    pub fn new_stringifier() -> Self {
        RdfXmlSerializer::new(Vec::new())
    }
    /// Create a new serializer which targets a `String` with a custom config.
    #[inline]
    pub fn new_stringifier_with_config(config: RdfXmlConfig) -> Self {
        RdfXmlSerializer::new_with_config(Vec::new(), config)
    }
}

impl Stringifier for RdfXmlSerializer<Vec<u8>> {
    fn as_utf8(&self) -> &[u8] {
        &self.write[..]
    }
}

// ---------------------------------------------------------------------------------
//                                      tests
// ---------------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use sophia_api::graph::isomorphic_graphs;
    use sophia_api::ns::*;
    use sophia_term::literal::convert::AsLiteral;
    use sophia_term::*;

    #[test]
    fn roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let me = StaticTerm::new_iri("http://champin.net/#pa")?;
        let g = vec![
            [
                me,
                rdf::type_.into(),
                StaticTerm::new_iri("http://schema.org/Person")?,
            ],
            [
                me,
                StaticTerm::new_iri("http://schema.org/name")?,
                "Pierre-Antoine".as_literal().into(),
            ],
        ];
        let s = RdfXmlSerializer::new_stringifier()
            .serialize_graph(&g)?
            .to_string();
        let g2: Vec<[BoxTerm; 3]> = crate::parser::parse_str(&s).collect_triples()?;
        assert!(isomorphic_graphs(&g, &g2)?);
        Ok(())
    }
}
