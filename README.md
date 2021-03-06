![Crates.io](https://img.shields.io/crates/l/url2)
![Crates.io](https://img.shields.io/crates/v/url2)

# url2

Url2: Ergonomic wrapper around the popular url crate

## Example

```rust
#[macro_use]
extern crate url2;

fn main() {
    let mut url = url2!("https://{}/", "example.com");
    url.query_unique()
        .set_pair("hello", "world")
        .set_pair("foo", "bar");

    assert!(url.query_unique_contains_key("hello"));
    assert_eq!("bar", url.query_unique_get("foo").unwrap());

    url.query_unique().remove("foo");

    assert_eq!(
        "https://example.com/?hello=world",
        url.as_str(),
    )
}
```
