#![cfg_attr(feature = "strict", deny(warnings))]


#[test]
fn test_hello_world() {
   // 'your_project_name' is set in the Carge.toml
    assert_eq!(true, your_project_name::start());
}
