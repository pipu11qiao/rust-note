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

Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or using the get method.

#### methods for iterating over strings

- "2A".charts()
- "2A".bytes()

### Strings Are Not So simple

strings are complicated

# 8.3 Storing Keys with Associated Values in Hash Maps

The type Hash<K,V> stores a mapping of keys of type K to values of type V using a hasing function, which determines how it places these keys and values into memory.

### Creating a New Hash Map
using new and adding elements with insert


```rust
use std::collection::HashMap
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);

```

### Accessing Values in a Hash Map

use get() method returns an Option<&V>

can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:

### Hash Maps and Ownership

For types that implements the Copy trait, the values are copied into the hash map. For owned values like a String ,the values will be moved and the hash map will be the owner of those values.


### Updating a Hash Map 

a same key 

#### Overwriting a Value

use insert() to overwrite the same key

#### adding a key and value only if a key isn't present

entry() check . the return value of the entry() is an enum called Entry

#### updating a value based on the old value



### hashing functions


