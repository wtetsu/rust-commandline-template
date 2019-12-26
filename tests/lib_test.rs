extern crate rust_commandline_template;

#[test]
fn pub_operation_name() {
  assert_eq!(rust_commandline_template::plus(1, 1), 2);
  assert_eq!(rust_commandline_template::plus(1, 2), 3);
  assert_eq!(rust_commandline_template::plus(4, 5), 9);
}
