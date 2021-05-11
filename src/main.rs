fn main() {
    println!("Hello, world!");
}


// Arrays, Vectors, and Slices

// Rust has three types for representing a sequence of values in memory:
//  1. The type [T; N] represents an array of N values, each of type T. An array's size is a constant determined at compile time, and is part of the type. You can't append new elements, or shrink an array.
// 2. The type Vec<T>, called a vector of Ts, is a dynamically allocated, growable sequence of values of type T. A vector's elements live on the heap, so you can resize vectors at will, push new elements onto them, append other vectors to them, delete elements, and so on.
// 3. The types &[T] and &mut[T], called a shared slice of Ts and mutable slice of Ts, are references to a series of elements that are part of some other value, like an array or vector. We can think of a slice as a pointer to its first element, together with a count of the number of elements we can access starting at that point. A mutable slice &mut [T] lets you read and modify elements, but can't be shared. A shared slice &[T] lets you share access among several readers, but doesn't let you modify elements.

// Given a value v of any of these three types, the expression v.len() gives the number of lements in v, and v[i] refers to the i'th element of v. The first element is v[0], the last is v[v.len() - 1]. Rust always checks that i falls within this range. If it doesn't, is panics. The length of v may be zero, in which case any attempt to index it will panic. 'i' must be a usize value, we can't use any other integer type as an index
