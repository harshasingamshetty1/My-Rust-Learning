https://practice.course.rs/collections/string.html#representation
<br>

# Strings

Ex 4

```rs

// FILL in the blank and FIX errors
    fn main() {
    let s = String::from("hello, 世界");
    let slice1 = s[0]; tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[3..5]; Tips: `中` takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    Iterate through all chars in s
    for (i, c) in s.\_\_ {
    if i == 7 {
    assert_eq!(c, '世')
    }
    }

    println!("Success!");
    }
```

SOl 4

String type cannot be indexed.

    Strings are always valid UTF-8. This has a few implications:

    The first of which is that if you need a non-UTF-8 string, consider OsString. It is similar, but without the UTF-8 constraint.

    The second implication is that you cannot index into a String.

    Indexing is intended to be a constant-time operation, but UTF-8 encoding does not allow us to do this.
    Furthermore, it’s not clear what sort of thing the index should return: a byte, a codepoint, or a grapheme cluster. The bytes and chars methods return iterators over the first two, respectively.

```rs
fn main() {
let s = String::from("hello, 世界");
// /_
// here we cannot just use, s[0..1]. bcoz the size for values of type `str` cannot be known at compilation time
// and hence, whenever the problem is that we cannot know the size at compile time.
// The awesome solution is to use, the pointer so that we know the size at compile time itself and hence we always use string slices &s[0..1]
// _/

let slice1 = &s[0..1]; tips: `h` only takes 1 byte in UTF8 format
assert_eq!(slice1, "h");

let slice2 = &s[7..10]; Tips: `中` takes 3 bytes in UTF8 format
assert_eq!(slice2, "世");

Iterate through all chars in s
for (i, c) in s.chars().enumerate(){
if i == 7 {
assert_eq!(c, '世')
}
}

println!("Success!");
}
```

You can use utf8_slice to slice UTF8 string, it can index chars instead of bytes.

Example:

```rs
        use utf8_slice;
        fn main() {
        let s = "The 🚀 goes to the 🌑!";

        let rocket = utf8_slice::slice(s, 4, 5);
         Will equal "🚀"
        }
```

A String is made up of three components: a pointer to some bytes, a length, and a capacity.

1. The pointer points to an internal buffer String uses to store its data.
2. The length is the number of bytes currently stored in the buffer( always stored on the heap ), and the
3. capacity is the size of the buffer in bytes. As such, the length will always be less than or equal to the capacity.

🌟🌟 If a String has enough capacity, adding elements to it will not re-allocate.

<br>EX 5

```rs

// Modify the code below to print out:
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
fn main() {
    let mut s = String::new();

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}
```

Sol 5

```rs
fn main() {
    /* This allocates the initial string with the specified capacity.

     */
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}
```

# Vectors
