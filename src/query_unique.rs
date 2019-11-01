use std::collections::HashMap;

use crate::*;

/// Gives access to the query string restricting the view to unique keys
pub struct Url2QueryUnique<'lt> {
    pub(crate) url_ref: &'lt mut Url2,
}

impl<'lt> Url2QueryUnique<'lt> {
    /// builder-style helper for chaining .set_pair().set_pair().etc
    pub fn set_pair(self, name: &str, value: &str) -> Self {
        self.url_ref
            .unique_cache
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
        self.url_ref.unique_cache.as_ref().unwrap()
    }
}

impl<'lt> std::ops::DerefMut for Url2QueryUnique<'lt> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.url_ref.unique_cache.as_mut().unwrap()
    }
}
