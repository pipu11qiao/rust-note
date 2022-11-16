# Common Programming Concepts

Thie chapter covers concepts that appear in almost every programming language and how they work in Rust.

Specifically, you'll learn about variables,baic types,functions comments, and control flow.

# 3.1 variables and mutability

### variables is immutable

### use mut to set variable to mutable

### constants

Rust's naming convention for constants is to use all uppercase with underscores between words;

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

## Shadowing

you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variables is shadowed by the second.
we can shadow a variable by using the same variable's name and repeating the use of the let keyword.

```rust
fn main(){
  let x =5;
  let x = x+1;
  {
    let x = x *2;
    println!("The value of x in the inner scope is: {x}");
  }
      println!("The value of x is: {x}");
}
```

can change the type of the value but reuse the save name.

# 3.2 Data types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

two data type type subsets: scalar and compund;