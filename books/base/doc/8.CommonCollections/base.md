# Common Collections

Rust's standard library includes a number of very useful data structures called collections.Unlike the builtin array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

three collections

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We've mentioned the String type previoursly, but in this chapter we'll talk about it in depth
- A hash map allows you to associate a value with a particular key. I'ts a particular implementation of the more general data structure called a map.

# 8.1 Storing Lists of Values with Vectors

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.Vectors can only store values of the same type.

### Creating a New Vector

Vectors are implemented using generics;
More often, you'll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation. Rust conveniently provides the vec! macro which will create a new vector that holds the values you give it.

```rust
let v =vec![1,2,3];
```

### Updating a Vector

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
```

Reading Elements of Vectors

```rust
   let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

```

supply for index and get two ways .
when you try to use an index value outside the range of existing elements.

#### mutable and immutable

because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elemetns to the new space.

### Iterating over the Values in a Vector

```rust
let v = vec![100,32,57];
for i in &v{
  pringtln!("{}",i)
}
```

# 8.2 Storing UTF-8 Encode Text with Strings

we discuss strings in the context of colletions because strings are implement4ed as a collection of bytes;
in this section, we'll talk about the operations on String that every collection type has, such as creating, updating, and reading. We'll also discuss the ways in which String is different from the other collections, namely how indexing into a String is complicated by the fiffereneces between how people and computers interpret String data.

### What is a String

- the String
- the string slice &str

The String type, which is provided by rust's library rather than code into the corelanguage, is a growable, mutable,owned.UTF-8 encoded string type.

### Creating a new String

mamy of the same operations available with Vec<T> are available with String as well as, because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees,restrictions,and capabilities.

```rust
let mut s = String::new();
```

```rust
let data = "initial contents";

let s = data.to_string();

let s = "initial contents".to_string();
```

```rust
let s = String::from("initial contents");
```

- push_str takes a string slice
- push takes a single character

#### Concatenation with + Operator or the format! Macro

```rust
let s3 = s1 + &s2;
```

why + left is String and right is reference

- operator uses the add method

```rust
fn add(self,s:&str)-> String{}
```

will lose s1;

&s2 is &String not &str

why it can compile

The reason we're able to use &s2 in the call to ad is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].

```rust
let s = format!("{}-{}-{}",s1,s2,s3);
```

### indexing string

rust don't support indexing the way
```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

#### Internal Representation

