# Managing Growing Projects with Packages, Crates, and Moduels

## module system

- Package: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or execuatble
- Modules and Use: let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

# 7.1 Packages and Crates

a crate is the smallest amount of code that the Rust compiler consider at a time.Crates can contain modules.

a binary crate or a library crate.

A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

A package can contains as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that's a library or binary crate.

# 7.2 Defining Moduels to Control Scope and Privacy

moduel

- namely paths that allow you to name items
- the use keyword that bring a path into scope
- the pub keyword to make items public
- the as keyword, external packages, and the glob operator

### how module work in compiler

##### 1. start from crate root

src/lib.rs for a library crate or src/main.rs for a binary crate

##### 2. Declaring moduels

In the crate root file you can declare new modules;say, you declare a "garden" module with mode garden; find path:

- Inline, within curly brackets that replace the semicolon following mode garden
- in the file src/garden.rs
- in the file src/garden/mod.rs

##### 3.Paths to code in moduels:

Once a moduel is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.

##### 4.Private vs public

Code within a module is private from its parent moduels by default.To make a module public, declare it with pub mode instead of mod. To make items within a public module public as well, use pub before thier declarations.

##### 5.The use keyword

Within a scope, the use keyword creates shortcuts ot items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus,youcan craate a shortcaut with use crate::garden::vegeables::Asparagus; and from then on you only need to wrete Asparaus

### Grouping Related Code in Modules

Moduels let us organize code within a crate for readability and easy reuse.Modules also allow us to control the privacy of items, because code within a module is private by default.

# 7.3 Paths for Referring to an Item in the Module Tree

how Rust where to find an item in a moduel tree, we use a path in the same way we use a path when navigating a filesystem. To call a function, we need to know its path.

A path can take two forms:

- An absolute path is the full path starting from a crate root;for code from an external crate,the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.

- A relative path starts from the current module and uses self, super or an indentifier in the current module.

seperated by double colons(::).

In Rust, all items(functions,methods, structs,enums,modules, and constants) are private to parent modules by default.

### Exposing Paths with the pub Keyword

Adding the pub keyword in front of mode hosting makes the module public. With this change, if we can access front_of_house, we can access hosting. But the contents of hosting are still private; making the module public doesn't make its contents public. The pub keyword on a module only lets code in its ancestor module refer to it, not access its inner code.

### Starting Relative Paths with super

```rust
        fn seat_at_table() {
            super::super::eat_at_restaurant();
        }
```

### Making Structs and Enums Public

if we use pub before a struct definition, we make the struct public , but the stuct's fields will still be private.

# 7.4 Bringing Paths into Scope with the use Keyword

Having to write out the paths to call functions can feel inconvenient and repetitive.a way to simplify this process: we can create a shortcut to a path with the use keyword once and then use the shorter name everywhere else in the scope.

Adding use and a path in ascope is similar to creating a symbolic link in the filesystem.

### Creating Idiomatic use Paths

use parent moduel

### Providing New Names with the as Keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1 ()->Result{}

fn function2()->IoResult<()>{}

```

### Re-exporting Names with pub use

```rust
mod front_of_house{
  pub mod hosting {
    pub fn add_to_waitlist(){}
  }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
}

```

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would thingk about the domain.

### Using External Packages

1. add rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand avaailable to our project.

2. add a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope.

```rust
use rand:Rng;
fn main(){
  let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

standard std library is also a crate that's external to our package.

```rust
use std::collection::HashMap;
```

### Using Nested Paths to Clean Up Large use Lists

If we're using multiple items defined in the same crate or same module, listing each item on its onw line can take up a lot of vertical space in our files.

```rust
use std::cmp::Ordering;
use std::io;
```

Instead, we can use nested paths to bring the same items into scope in one line.

```rust
use std::{cmp::Ordering,io};
```

We can use a nested path at any level in a path, which is useful when combining two use statement that share a subpath.

```rust
use std::io;
use std::io::Write;
```

```rust
use std::io::{self,Write};
```

### The Glob Operator

if we want to bring all public items defined in a path into scope, we can specify that path followed by the \* glob operator:

```rust
use std::collection::*;
```


# 7.5 Separating Modules into Different Files

