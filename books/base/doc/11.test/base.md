Writing Automated Tests


a test in Rust ia a function that's annotated with the test attribute. Attributes are metadata about pieces of Rust code; to change a function into a test function, add #[test] on the line before fn. when you run your tests with the cargo test command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.
