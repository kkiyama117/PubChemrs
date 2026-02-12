//! Higher-level structural types for working with PubChem data.
//!
//! These types provide a more ergonomic interface than the raw API response
//! types in [`crate::response`]. Use [`crate::response::Compound::setup_atoms`]
//! and [`crate::response::Compound::setup_bonds`] to convert raw records.

mod atom;
mod bond;
mod classification;
mod compound;
mod coordinates;

pub use atom::{Atom, Element};
pub use bond::{Bond, BondType};
pub use classification::{CompoundIdType, ProjectCategory, ResponseCoordinateType};
pub use compound::CompoundID;
pub use coordinates::{Coordinate, CoordinateType};
