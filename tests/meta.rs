use gars;

#[test]
fn test_get_version() {
    let version = gars::get_version();

    assert!(version != "unknown");
}
