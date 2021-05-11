fn main() {
    println!("Hello, world!");



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
    
    
    
    // Vectors
    
    // A vector Vec<T> is a resizable array of elements of type T, allocated on the heap.
    
    // The simples way to create a vector is to use the vec! macro, which gives us a syntax for vectors that looks very much like an array literal:
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    
    // A vector isn't an array, so we can add elements to it dynamically:
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);
    
    // We can also build a vector by repeating a given value a certain number of times using syntax that imitates array literals:
    fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
        vec![0; rows * cols]
    }
    
    // The vec! macro is equivalent to calling Vec::new to create a new, empty vector, and then pushing the elements onto it, which is another idiom:
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);
    
    // Another possibility is to build a vector from the values produced by an iterator:
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
    
    // We'll often need to supply the type when using collect (as above), because it can build many different sorts of collections, not just vectors. By making the type for v explicit, we've made it unambiguous which sort of collection we want.
    
    // Like arrays, we can use slice methods on vectors:
    // A palindrome!
    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    // Reasonable yet disappointing:
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    // Here the reverse method is defined on slices, but the call implicitly borrows a &mut [&str] slice from the vector, and invokes reverse on that.

    // Vec is an essential type to Rust. It's used almost anywhere one needs a list of dynamic size, so there are many other methods that construct new vectors or extend existing ones. More on them in chap 16.

    // A Vec<T> consists of three values: a pointer to the heap-allocated buffer, allocated to hold the elements; the number of elements that buffer has the capacity to store; and the number it actually contains now (its length). When the buffer has reached its capacity, adding another element to the vector entails allocating a larger buffer, copying the present contents into it, updating the vector's pointer and capacity to describe the new buffer, and finally freeing the old one.

    // If we know the number of elements a vector will need in advance, instead of Vec::new we can call Vec::with_capacity to create a vector with a buffer large enough to hold them all, right from the start; then we can add elements to the vector one at a time without causing any reallocation. The vec! marco uses a trick like this, since it knows how many elements the final vector will have. However, this only applies to it's initial size, if we exceed our estimate, the vector enlarges its storage as usual.

    // Many library functions look to use the Vec::with_capacity instead of Vec::new. An example of this is the collect example above using the iterator 0..5. It knows in advance it'll yield 5 values, so the collect function preallocates the vector with the correct capacity.

    // The vector's len method returns the number of elements it contains currently. Its capacity method returns the number of elements it could hold without reallocation:
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    // We learn a bit later that a vector, when it's capacity is exceeded, doubles. So here the original capacity is 2, we added a new variable which made the len 3 > cap 2, so now it grows to 4, doubling. Next would be 8, and so on.
    assert_eq!(v.capacity(), 4);
    // Vec and the system's heap allocator may round up requests, even in the with_capacity case.

    // We can insert and remove elements wherever we like in a vector. These operations shift all the elements after the insertion point, forward or backward, so they may be slow if the vector is long:
    let mut v = vec![10, 20, 30, 40, 50];

    // Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    // We can use the pop method to remove the last element and return it. Popping a value from a Vec<T> returns an Option<T>. None if the vector was already empty, or Some(v) if its last element had been v:
    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

    // We can use a for loop to iterate over a vector:
    // Get our command-line argument as a vector of Strings.
    // I believe the .skip(1) here is used to skip 'cargo run' when we use it to compile this file. It starts at the 3rd variable which is Lisp (using below)
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            });
    }

    // The above can be run by writing cargo run Lisp Scheme C C++ Fortran



    // Building Vectors Element by Element

    // Whenever a vector outgrows its buffer's capacity, it chooses a new buffer twice as large as the old one. I.e starting at 1, it'd grow to 2, then 4, 8, and so on (2^n for some n). Since the number of actual elements is at least half the buffer size, the vector has always performed less than two copies per element.
    // What this means is that using Vec::with_capacity instead of Vec::new is a way to gain a constant factor improvement in speed, rather than an algorithmic improvement. For small vectors, avoiding a few calls to the heap allocator can make an observable difference in performance.



    // Slices

    // A slice, written [T] without specifying the length, is a region of an array or vector. Since a slice can be any length, slices can't be stored directly in variables or passed as function arguments. Slices are always passed by reference.

    // A reference to a slice is a 'fat pointer': a two-word value comprising a pointer to the slice's first element, and the number of elements in the slice.
    // Example:
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707]; // vector
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707]; // array

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    // For the last two lines, Rust automatically converts the &Vec<f64> reference and the &[f64; 4] reference to slice references that point directly to the data. See page 105 for diagram.

    // An ordinary reference is a non-owning pointer to a single value, a reference to a slice is a non-owning pointer to several values. This makes slice references a good choice when you want to write a function that operates on any homogeneous data series, whether stored in an array, vector, stack, or heap. For example, a function that prints a slice of numbers, one per line:
    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }

    print(&v); // works on vectors
    print(&a); // works on arrays

    // Because this function takes a slice reference as an argument, we can apply it to either a vector or an array, as shown. In fact, many methods we'd might think of as belonging to vectors or arrays are actually methods defined on slices. For example, the sort and reverse methods, which sort or reverse a sequence of elements in place, are actually methods on the slice type [T].

    // We can get a reference to a slice of an array or vector, or a slice of an existing slice, by indexing it with a range:
    print(&v[0..2]); // print the first two elements of v
    print(&a[2..]); // print elements of a starting with a[2]
    print(&sv[1..3]); // print v[1] and v[2]

    // Great explanation of why you'd want to work with references vs directly:
    // https://www.reddit.com/r/rust/comments/au6khw/what_i_should_choose_reference_or_value/

    // As with ordinary array accesses, Rust checks that the indices are valid. Trying to borrow a slice that extends past the end of the data results in a panic.

    // We often use the terms slice for reference types like &[T] or &str, but that is a bit of shorthand. Those are properly called references to slices. Since slices almost always appear behind references, we use the shorter name for the more common concept.
}




