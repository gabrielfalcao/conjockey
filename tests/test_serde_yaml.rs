#[test]
fn test_yaml_number_and_yaml_value_number_from_option_integer_and_option_value_integer_fail_to_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/*/caught.rs");
}
