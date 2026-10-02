//! `finl_unicode` is a crate to provide Unicode support for the finl project. This is not necessarily
//! meant to be a comprehensive Unicode support, although I will consider adding additional use cases
//! as necessary. Unicode 18.0.0 is implemented in the current version.
//!
//! Two features are currently supported:
//! - **Unicode segmentation**. (Specify `grapheme_clusters` as a feature when importing the crate.) For a peekable
//!   iterator of `Chars` or `CharIndices`, we extend that iterator to include a `next_cluster` method which returns
//!   `Option<String>` which will contain the next grapheme cluster if there is one or `None` if there isn't. For
//!   iterating over all the clusters of a `&str`, `Graphemes` returns each cluster as a `&str` without allocating.
//! - **Character category**. (Specify `categories` as a feature when importing the crate.) Extends the `char` class with methods for testing the
//!   category of the character.
//!
//! The default is to compile all features. Note that the Rust compiler/linker will not link unused
//! code, so most of the time, there will be no need to remove features.
//!
//! The Unicode data tables are pre-generated and checked into the crate (by the `generate-sources`
//! program in the repository), so building the crate does not require network access.

#[cfg(feature = "categories")]
pub mod categories;

#[cfg(feature = "grapheme_clusters")]
pub mod grapheme_clusters;

mod data;
