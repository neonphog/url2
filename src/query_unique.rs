use std::collections::HashMap;

use crate::*;

/// Gives access to the query string restricting the view to unique keys.
///
/// Dereferences to `HashMap<String, String>` for easy key/value manipulation.
///
/// # Example
///
/// ```rust
/// use url2::prelude::*;
///
/// let mut url = Url2::parse("https://test.com/?a=1&b=2");
///
/// url.query_unique().remove("b");
///
/// assert_eq!("https://test.com/?a=1", url.as_str());
/// ```
pub struct Url2QueryUnique<'lt> {
    pub(crate) url_ref: &'lt mut Url2,
}

impl<'lt> Url2QueryUnique<'lt> {
    /// builder-style helper for chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// let mut url = Url2::default();
    /// url.query_unique()
    ///     .set_pair("a", "1")
    ///     .set_pair("b", "2");
    ///
    /// assert!(
    ///     "none:?a=1&b=2" == url.as_str() ||
    ///     "none:?b=2&a=1" == url.as_str()
    /// );
    /// ```
    pub fn set_pair(self, name: &str, value: &str) -> Self {
        (self.url_ref.0)
            .1
            .as_mut()
            .unwrap()
            .insert(name.to_string(), value.to_string());
        self
    }
}

impl<'lt> Drop for Url2QueryUnique<'lt> {
    fn drop(&mut self) {
        self.url_ref.priv_sync_query_unique_cache();
    }
}

impl<'lt> std::ops::Deref for Url2QueryUnique<'lt> {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        (self.url_ref.0).1.as_ref().unwrap()
    }
}

impl<'lt> std::ops::DerefMut for Url2QueryUnique<'lt> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.url_ref.0).1.as_mut().unwrap()
    }
}
