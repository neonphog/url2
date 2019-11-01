//! Url2: Ergonomic wrapper around the popular url crate
//!
//! # Example
//!
//! ```rust
//! use url2::prelude::*;
//!
//! let mut url = Url2::parse("https://example.com/");
//! url.query_unique()
//!     .set_pair("hello", "world")
//!     .set_pair("foo", "bar");
//!
//! assert!(url.query_unique_contains_key("hello"));
//! assert_eq!("bar", url.query_unique_get("foo").unwrap());
//!
//! url.query_unique().remove("foo");
//!
//! assert_eq!(
//!     "https://example.com/?hello=world",
//!     url.as_str(),
//! )
//! ```

extern crate url;

mod error;
pub use crate::error::*;

mod query_unique;
pub use crate::query_unique::*;

mod url2;
pub use crate::url2::*;

pub mod prelude {
    // currently, just export everything
    // at some point, we may be more selective
    pub use super::*;
}
