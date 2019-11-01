/// Url2 Result Type
pub type Url2Result<T> = Result<T, Url2Error>;

#[derive(Debug, Clone)]
/// Represents a Url2 Error
pub struct Url2Error(Box<Url2ErrorKind>);

impl Url2Error {
    /// access the Url2ErrorKind for this error
    pub fn kind(&self) -> &Url2ErrorKind {
        &self.0
    }

    /// convert this error into a raw kind
    pub fn into_kind(self) -> Url2ErrorKind {
        *self.0
    }
}

impl std::error::Error for Url2Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self.0 {
            Url2ErrorKind::UrlParseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Url2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self.0 {
            Url2ErrorKind::UrlParseError(ref err) => err.fmt(f),
            _ => write!(f, "Url2Error::Unknown"),
        }
    }
}

impl std::convert::From<Url2ErrorKind> for Url2Error {
    fn from(kind: Url2ErrorKind) -> Url2Error {
        Url2Error(Box::new(kind))
    }
}

impl std::convert::From<url::ParseError> for Url2Error {
    fn from(err: url::ParseError) -> Self {
        Url2ErrorKind::UrlParseError(err).into()
    }
}

#[derive(Debug, Clone)]
/// enum representing the type of Url2Error
pub enum Url2ErrorKind {
    /// Url Parsing Error
    UrlParseError(url::ParseError),

    // allow expansion
    #[doc(hidden)]
    __Nonexhaustive,
}
