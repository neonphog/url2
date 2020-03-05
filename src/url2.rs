use std::collections::HashMap;
use url::Url;

use crate::*;

#[derive(Clone)]
/// Ergonomic wrapper around the popular Url crate
pub struct Url2 {
    pub(crate) url: Url,
    pub(crate) unique_cache: Option<HashMap<String, String>>,
}

impl Url2 {
    // would love to use std::convert::TryFrom, except for conflicting
    // blanket implementation: https://github.com/rust-lang/rust/issues/50133
    /// Try to parse a utf8 slice into a Url2 instance.
    /// May result in a UrlParseError
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// assert_eq!(
    ///     "Err(Url2Error(UrlParseError(RelativeUrlWithoutBase)))",
    ///     &format!("{:?}", Url2::try_parse("")),
    /// );
    /// assert_eq!(
    ///     "Ok(Url2 { url: \"none:\" })",
    ///     &format!("{:?}", Url2::try_parse("none:")),
    /// );
    /// ```
    pub fn try_parse<S: AsRef<str>>(s: S) -> Url2Result<Self> {
        Ok(Url2::priv_new(Url::parse(s.as_ref())?))
    }

    // would love to use std::convert::From, except for conflicting
    // blanket implementation: https://github.com/rust-lang/rust/issues/50133
    /// Try to parse a utf8 slice into a Url2 instance.
    /// If this results in a UrlParseError, this method will panic!
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// assert_eq!("none:", Url2::parse("none:").as_str());
    /// ```
    pub fn parse<S: AsRef<str>>(s: S) -> Self {
        Self::try_parse(s).unwrap()
    }

    /// convert this Url2 instance into a string
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// assert_eq!("none:", Url2::default().as_str());
    /// ```
    pub fn into_string(self) -> String {
        self.into()
    }

    /// Access query string entries as a unique key map.
    ///
    /// Url query strings support multiple instances of the same key
    /// However, many common use-cases treat the query string
    /// keys as unique entries in a map. An optional API viewing the
    /// query string in this manner can be more ergonomic.
    ///
    /// The HashMap that backs this view is only created the first time this
    /// function (or the following query_unique_* functions) are invoked.
    /// If you do not use them, there is no additional overhead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// let mut url = Url2::default();
    /// url.query_unique().set_pair("a", "1").set_pair("a", "2");
    ///
    /// assert_eq!("none:?a=2", url.as_str());
    /// ```
    pub fn query_unique(&mut self) -> Url2QueryUnique {
        self.priv_ensure_query_unique_cache();
        Url2QueryUnique { url_ref: self }
    }

    /// When parsed as a unique map, does the query string contain given key?
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// let mut url = Url2::parse("none:?a=1");
    ///
    /// assert!(url.query_unique_contains_key("a"));
    /// assert!(!url.query_unique_contains_key("b"));
    /// ```
    pub fn query_unique_contains_key(&mut self, key: &str) -> bool {
        self.priv_ensure_query_unique_cache();
        self.unique_cache.as_ref().unwrap().contains_key(key)
    }

    /// When parsed as a unique map, get the value for given key
    ///
    /// # Example
    ///
    /// ```rust
    /// use url2::prelude::*;
    ///
    /// let mut url = Url2::parse("none:?a=1");
    ///
    /// assert_eq!(
    ///     "Some(\"1\")",
    ///     &format!("{:?}", url.query_unique_get("a")),
    /// );
    /// assert_eq!(
    ///     "None",
    ///     &format!("{:?}", url.query_unique_get("b")),
    /// );
    /// ```
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

impl std::fmt::Debug for Url2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Url2")
            .field("url", &self.url.as_str())
            .finish()
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

impl std::convert::From<&Url> for Url2 {
    fn from(url: &Url) -> Url2 {
        Url2::priv_new(url.clone())
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
        let url_c = try_url2!("{}:{}", "test", "bob").unwrap();
        let url_d = url2!("{}:{}", "test", "bob");
        assert_eq!(url_a, url_b);
        assert_eq!(url_a, url_c);
        assert_eq!(url_a, url_d);
    }

    #[test]
    fn it_can_convert_from() {
        let url = Url2::default();
        let url: Url = url.into();
        let url: Url2 = url.into();
        let url: Url = url.into();
        let url: Url2 = (&url).into();
        assert_eq!("none:", url.as_str());
    }

    #[test]
    fn it_can_edit_query_unique() {
        let mut url = Url2::default();
        url.query_unique()
            .set_pair("a", "test1")
            .set_pair("b", "test2");
        assert!("none:?a=test1&b=test2" == url.as_str() || "none:?b=test2&a=test1" == url.as_str());
        assert_eq!(true, url.query_unique_contains_key("a"));
        assert_eq!(false, url.query_unique_contains_key("c"));
        assert_eq!(Some("test1"), url.query_unique_get("a"));
        assert_eq!(None, url.query_unique_get("c"));
    }
}
