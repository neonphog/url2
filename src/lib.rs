//! Url2: Ergonomic wrapper around the popular url crate
//!
//! # Example
//!
//! ```rust
//! #[macro_use]
//! extern crate url2;
//!
//! fn main() {
//!     let mut url = url2!("https://{}/", "example.com");
//!     url.query_unique()
//!         .set_pair("hello", "world")
//!         .set_pair("foo", "bar");
//!
//!     assert!(url.query_unique_contains_key("hello"));
//!     assert_eq!("bar", url.query_unique_get("foo").unwrap());
//!
//!     url.query_unique().remove("foo");
//!
//!     assert_eq!(
//!         "https://example.com/?hello=world",
//!         url.as_str(),
//!     )
//! }
//! ```

extern crate url;

mod error;
pub use crate::error::*;

mod query_unique;
pub use crate::query_unique::*;

mod url2;
pub use crate::url2::*;

/// works like the `format!()` macro, but passes the result through Url2::try_parse()
#[macro_export]
macro_rules! try_url2 {
    ($($e:expr),+) => {
        $crate::Url2::try_parse(&format!($($e),+))
    };
    ($($e:expr),+,) => {
        $crate::Url2::try_parse(&format!($($e),+))
    };
}

/// works like the `format!()` macro, but passes the result through Url2::parse()
#[macro_export]
macro_rules! url2 {
    ($($e:expr),+) => {
        $crate::Url2::parse(&format!($($e),+))
    };
    ($($e:expr),+,) => {
        $crate::Url2::parse(&format!($($e),+))
    };
}

pub mod prelude {
    // currently, just export everything
    // at some point, we may be more selective
    pub use super::*;
}
