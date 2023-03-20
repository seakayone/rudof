//! Prefix map implementation
//! 
//! Implements prefix maps, which are used in Turtle and ShEx
//! 
//! A prefix map is a list of alias declarations associated with IRIs
//! 
//! ```turtle
//! prefix schema: <http://schema.org/>
//! prefix :       <http://example.org/>
//! ```
//! 
//! Example 
//!
//! ```
//! # use std::str::FromStr;
//! # use iri_s::{IriS, IriError};
//! # use prefix_map::PrefixMap;
//! 
//! # fn main() -> Result<(), IriError> {
//! let schema_iri  = IriS::from_str("http://schema.org/")?;
//! let example_iri = IriS::from_str("http://example.org/")?;
//! let mut pm = PrefixMap::new();
//! pm.insert("schema", &schema_iri);
//! pm.insert("", &example_iri);
//! # Ok(())
//! # }
//! ```

// ```
// let mut pm = PrefixMap::new();
// let binding = ; 
// pm.insert("schema", &IriS::from_str("http://schema.org/"))
// pm.insert("", &IriS::from_str("http://example.org/")?);
// ```
pub mod prefix_map;
pub mod alias;
pub use crate::prefix_map::*;
pub use alias::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use iri_s::{IriS, IriError};

    use super::*;

    #[test]
    fn it_works() -> Result<(), IriError> {
        let mut pm = PrefixMap::new();
        let schema_iri = IriS::from_str("http://schema.org/")?;
        pm.insert("schema", &schema_iri);
        let resolved = pm.resolve("schema:knows")?;
        let schema_knows = IriS::from_str("http://schema.org/knows")?;
        assert_eq!(resolved, Some(schema_knows));
        Ok(())
    }
}
