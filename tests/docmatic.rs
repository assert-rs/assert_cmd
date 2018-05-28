extern crate docmatic;

#[test]
fn test_readme() {
    docmatic::assert_file("README.md");
}
