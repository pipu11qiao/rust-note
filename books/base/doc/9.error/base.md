
# Error Handling


Rust groups errors into two major categories: recoverable and unrecoverable errors;For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, ans so we want to immediately stop the program.

Rust doesn't have exceptions. Instead , ite has the type Result<T,E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

# 9.1 Unrecoverable Errors with panic!

panic! macro. there are two ways to cause a panic in practice: by taking an action that causes our code to panic or by explicitly calling the panic! macro.

# 9.2 Recoverable errors with Result

Most errors aren't serious enough to require the program to stop entirely. Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.

The pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

#### A shortcut for propagating errors: the ? operator

The ? placed after a Result value is defined to work in almost the same way as match expressions.

##### difference between ? operator and match expression

# 9.3 to panic! or not to panic!

