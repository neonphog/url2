use std::collections::HashMap;
use url::Url;

use crate::*;

#[derive(Debug, Clone)]
/// An ergonomic wrapper around the popular Url crate
///
/// provides:
///  - a panic!ing `Url2::parse()` function that gives a direct instance
///  - a `Url2::try_parse()` function that returns a Result
///  - some additional conversion utility implementations
///  - a "unique key" view into the query string (see `Url2::query_unique()`)
pub struct Url2 {
    pub(crate) url: Url,
    pub(crate) unique_cache: Option<HashMap<String, String>>,
}

impl Url2 {
    /// would love to use std::convert::TryFrom, except for conflicting
    /// blanket implementation: https://github.com/rust-lang/rust/issues/50133
    pub fn try_parse<S: AsRef<str>>(s: S) -> Url2Result<Self> {
        Ok(Url2::priv_new(Url::parse(s.as_ref())?))
    }

    /// would love to use std::convert::From, except for conflicting
    /// blanket implementation: https://github.com/rust-lang/rust/issues/50133
    pub fn parse<S: AsRef<str>>(s: S) -> Self {
        Self::try_parse(s).unwrap()
    }

    /// convert this Url2 instance into a string
    pub fn into_string(self) -> String {
        self.into()
    }

    /// generates a hashed map of query keys on first call
    /// subsequent calls will use this existing cache
    /// returns an instance that dereferences to a HashMap<String, String>
    /// for manipulating the query string requiring unique keys
    /// (i.e. setting the same key again will overwrite the previous key)
    /// while query strings technically can have duplicate keys,
    /// unique keys are a common use-case, an this makes that case more ergonomic.
    /// When the returned instance is dropped, the underlying querystring
    /// will be updated to reflect any changes made to the HashMap.
    pub fn query_unique(&mut self) -> Url2QueryUnique {
        self.priv_ensure_query_unique_cache();
        Url2QueryUnique { url_ref: self }
    }

    /// when parsed as a unique map, does the query string contain given key?
    pub fn query_unique_contains_key(&mut self, key: &str) -> bool {
        self.priv_ensure_query_unique_cache();
        self.unique_cache.as_ref().unwrap().contains_key(key)
    }

    /// when parsed as a unique map, get the value for given key
    pub fn query_unique_get(&mut self, key: &str) -> Option<&str> {
        self.priv_ensure_query_unique_cache();
        match self.unique_cache.as_ref().unwrap().get(key) {
            None => None,
            // silly dance to convert &String to &str
            Some(s) => Some(s),
        }
    }

    // -- private -- //

    /// private constructor, you probably want `Url2::try_parse()`
    fn priv_new(url: Url) -> Self {
        Self {
            url,
            unique_cache: None,
        }
    }

    /// generate our unique query string entry cache if we haven't already
    fn priv_ensure_query_unique_cache(&mut self) {
        if self.unique_cache.is_none() {
            std::mem::replace(&mut self.unique_cache, Some(HashMap::new()));
            for (k, v) in self.url.query_pairs() {
                self.unique_cache
                    .as_mut()
                    .unwrap()
                    .insert(k.to_string(), v.to_string());
            }
        }
    }

    /// if changes have been made to our query unique cache, apply them
    pub(crate) fn priv_sync_query_unique_cache(&mut self) {
        let mut all = self
            .unique_cache
            .as_mut()
            .unwrap()
            .drain()
            .collect::<Vec<_>>();
        {
            let mut pairs = self.query_pairs_mut();
            pairs.clear();
            for (k, v) in all.iter() {
                pairs.append_pair(k, v);
            }
        }
        for (k, v) in all.drain(..) {
            self.unique_cache.as_mut().unwrap().insert(k, v);
        }
    }
}

impl std::cmp::PartialOrd for Url2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.url.partial_cmp(&other.url)
    }
}

impl std::cmp::PartialOrd<Url> for Url2 {
    fn partial_cmp(&self, other: &Url) -> Option<std::cmp::Ordering> {
        self.url.partial_cmp(other)
    }
}

impl std::cmp::Ord for Url2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.url.cmp(&other.url)
    }
}

impl std::cmp::PartialEq for Url2 {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(&other.url)
    }
}

impl std::cmp::Eq for Url2 {}

impl std::cmp::PartialEq<Url> for Url2 {
    fn eq(&self, other: &Url) -> bool {
        self.url.eq(&other)
    }
}

impl std::hash::Hash for Url2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl std::fmt::Display for Url2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl std::convert::From<Url2> for String {
    fn from(url: Url2) -> String {
        url.to_string()
    }
}

impl std::default::Default for Url2 {
    fn default() -> Self {
        Url2::priv_new(Url::parse("none:").unwrap())
    }
}

impl std::convert::AsRef<str> for Url2 {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl std::borrow::Borrow<str> for Url2 {
    fn borrow(&self) -> &str {
        self.url.as_ref()
    }
}

impl std::ops::Deref for Url2 {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl std::ops::DerefMut for Url2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
    }
}

impl std::borrow::Borrow<Url> for Url2 {
    fn borrow(&self) -> &Url {
        &self.url
    }
}

impl std::borrow::BorrowMut<Url> for Url2 {
    fn borrow_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}

impl std::convert::AsRef<Url> for Url2 {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::convert::AsMut<Url> for Url2 {
    fn as_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}

impl std::convert::From<Url> for Url2 {
    fn from(url: Url) -> Url2 {
        Url2::priv_new(url)
    }
}

impl std::convert::From<Url2> for Url {
    fn from(url: Url2) -> Url {
        url.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_display() {
        assert_eq!("test:foo", &format!("{}", Url2::parse("test:foo")));
        assert_eq!("test:foo", &Url2::parse("test:foo").into_string());
    }

    #[test]
    fn it_can_parse() {
        let url_a = Url2::try_parse("test:bob").unwrap();
        let url_b = Url2::parse("test:bob");
        assert_eq!(url_a, url_b);
    }

    #[test]
    fn it_can_convert_from() {
        let url = Url2::default();
        let url: Url = url.into();
        let url: Url2 = url.into();
        assert_eq!("none:", &url.to_string());
    }

    #[test]
    fn it_can_edit_query_unique() {
        let mut url = Url2::default();
        url.query_unique()
            .set_pair("a", "test1")
            .set_pair("b", "test2");
        assert!(
            "none:?a=test1&b=test2" == &url.to_string()
                || "none:?b=test2&a=test1" == &url.to_string()
        );
        assert_eq!(true, url.query_unique_contains_key("a"));
        assert_eq!(false, url.query_unique_contains_key("c"));
        assert_eq!(Some("test1"), url.query_unique_get("a"));
        assert_eq!(None, url.query_unique_get("c"));
    }
}
