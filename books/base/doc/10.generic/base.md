# Generic Types, Traits, and Lifetimes

how to use generic
how to use traits to define behavior in a generic way.

lefttime: a variety of generics that give the compiler information about how references relate to each other.Lifettimes allow us to give the compiler enougth information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

# 10.1 generic data types

we use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

- function
- enum
- struct
- methods using generic

all these with generic

### function

the data types of the parameters and return values.

```rust
fn largest <T>(list:&[T]) -> &T{}
```

### struct

```rust

struct Point<T> {
  x:T,
  y:T,
}

struct PointA<T,U>{
  x:T,
  y:U,
}

```

### enum

```rust
enum Option<T>{
  Some(T),
  None
}

enum Result<T,E>{
  OK(T),
  Err(E),
}
```

### method


## Performance of Code using generics

Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.


# 10.2 traits: defining shared behavior

a trait defines functionality a particular type has and can share with other types. we can use traits to define shared behavior in an abstract way. we can use trait bounds to specify that a generic type can be any type that has certain behavior.

### defining a trait

a type's behavior consits of the methods we can call on type. Different types share the sam behavior if we can calll the same methods on all of those types. Trait definitions are a way to group method signatures together ot define a set of behaviors necessary to accomplish some purpose.


a trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

# 10.3 validating references with lifetimes

rather than ensuring that a type has the behavior we want, life time ensure that references are valid as long as we need them to be.

every reference in Rust has a lefetime,which is the scope for which that reference is valid.Most of the time, lifetimes are implicit and inferred. we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### the borrow checker

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
} 
```