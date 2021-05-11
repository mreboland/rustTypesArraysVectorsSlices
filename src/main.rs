fn main() {
    println!("Hello, world!");
}


// Arrays, Vectors, and Slices

// Rust has three types for representing a sequence of values in memory:
//  1. The type [T; N] represents an array of N values, each of type T. An array's size is a constant determined at compile time, and is part of the type. You can't append new elements, or shrink an array.
// 2. The type Vec<T>, called a vector of Ts, is a dynamically allocated, growable sequence of values of type T. A vector's elements live on the heap, so you can resize vectors at will, push new elements onto them, append other vectors to them, delete elements, and so on.
// 3. The types &[T] and &mut[T], called a shared slice of Ts and mutable slice of Ts, are references to a series of elements that are part of some other value, like an array or vector. We can think of a slice as a pointer to its first element, together with a count of the number of elements we can access starting at that point. A mutable slice &mut [T] lets you read and modify elements, but can't be shared. A shared slice &[T] lets you share access among several readers, but doesn't let you modify elements.

// Given a value v of any of these three types, the expression v.len() gives the number of lements in v, and v[i] refers to the i'th element of v. The first element is v[0], the last is v[v.len() - 1]. Rust always checks that i falls within this range. If it doesn't, is panics. The length of v may be zero, in which case any attempt to index it will panic. 'i' must be a usize value, we can't use any other integer type as an index



// Arrays

// There are a few ways to write arrays, however the simplest is to use square brackets:
let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

assert_eq!(lazy_caterer[3], 7);
assert_eq!(taxonomy.len(), 3);

// For the common case of a long array filled with some value, we can write [V; N], where V is the value each element should have, and N is the length. For example, [true; 10000] is an array of 10,0000 bool elements all set to true:
let mut sieve = [true; 10000];
for i in 2..100 {
    if sieve[i] {
        let mut j = i * i;
        while j < 10000 {
            sieve[j] = false;
            j += i;
        }
    }
}

assert!(sieve[211]);
assert!(!sieve[9876]);

// We'll see this syntax used for fixed-size buffer: [0u8, 1024] can be a one-kilobyte buffer, filled with zero bytes. Rust has no notation for an uninitialized array. In general, Rust ensure that code can never access any sort of uninitialized value.

// An array's length is part of its type and fixed at compile time. If n is a variable, we can't write [true; n] to get an array of n elements. If we need an array whose length varies, use a vector instead.

// The useful methods you'd like to see on arrays -- iterating over elements, searching, sorting, filling, filtering, and so on -- all appear as methods of slices, not arrays. But Rust implicitly converts a reference to an array to a slice when searching for methods, so we can call any slice method on an array directly:
let mut chaos = [3, 5, 4, 1, 2];
chaos.sort();
assert_eq!(chaos, [1, 2, 3, 4, 5]);
// The sort method is actually defined on slices, but since sort takes its operand by reference, we can use it directly on chaos. The call implicitly produces a &mut [i32] slice referring to the entire array. In fact, the len method mentioned earlier is a slice method as well.


