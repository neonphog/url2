#[macro_use]
extern crate url2;

#[test]
fn it_can_try_parse_macro() {
    assert_eq!(
        "test://bob/?a=42",
        &try_url2!("{}://{}?a={}", "test", "bob", 42)
            .unwrap()
            .to_string(),
    );
    assert_eq!(
        "test://bob/?a=42",
        &try_url2!("{}://{}?a={}", "test", "bob", 42,)
            .unwrap()
            .to_string(),
    );
}

#[test]
fn it_can_parse_macro() {
    assert_eq!(
        "test://bob/?a=42",
        &url2!("{}://{}?a={}", "test", "bob", 42).to_string(),
    );
    assert_eq!(
        "test://bob/?a=42",
        &url2!("{}://{}?a={}", "test", "bob", 42,).to_string(),
    );
}
