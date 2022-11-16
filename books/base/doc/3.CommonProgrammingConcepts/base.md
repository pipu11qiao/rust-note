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

Keep in mind that Rust is a statically typed language,which means that it must know the types of all variables at compile time;when many types are possible we must add a type annotation

```rust
let gutss:u32 = "42".parse().expet("Not a number");
```

对应的报错
error[E0282]: type annotations needed

### Scalar Types

integers, floating-point numbers,Booleans and characters

#### integer

integer types in rust

8-bit
16-bit
32-bit
64-bit
128-bit
arch

integer literals in Rust

use \_ as a visual separator to make number easier to read;

integer types default ot i32;

#### floating-point types

f32 f64

#### numeric operations

#### The Boolean Type

#### The Character Type

```rust
let f:bool = false
```

#### The Character Type

```rust
let c = 'z';
```

### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays;

#### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
let tup = (500,6.4,1);
let (x,y,z) = tup;
```


#### The Array Type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

# Functions

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

### Parameters

In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.

### Statements and Expressions

Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

``` rust
fn main() {
    let x = (let y = 6);
}
```

The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.



Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:


### Functions with Return values

