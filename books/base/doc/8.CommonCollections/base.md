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

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
```
