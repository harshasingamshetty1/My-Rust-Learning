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

EX 1

```rs

fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    let v1 = vec!(arr);
    is_vec(&v1);

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}

```

sol 1

```rs

fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    // let v1 = vec!(arr);
    // is_vec(&v1);
    /*
        so whenever we have an arr and to convert it to Vec.
        directly use, Vec::from instead of vec!(arr).
        bcoz the latter's type would be Vec<[u8;3]>
     */
    let mut v1 = Vec::new();
    for i in &arr {
        v1.push(*i);
    }

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}
```

EX 3

```rs


fn main() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);


    // String -> Vec
    // impl From<String> for Vec
    let s: String = "hello".to_string();
    /*
        Basically, String is equivalent to Vec<u8>
        so, we can convert String into vector using, into_bytes or just s.into()
     */
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
 }
```

The elements in a vector must be the same type, for example , the code below will cause an error:

```rs
fn main() {
let v = vec![1, 2.0, 3];
}
```

But we can use enums or trait objects to store distinct types.

```rs
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    /*
        Here, we are able to store different types in vector with the help of enums
     */
    let v : Vec<IpAddr>= Vec::from([IpAddr::V4("127.0.0.1".to_string()),IpAddr::V6("::1".to_string())]);

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}
```

## HashMaps

Requirements of HashMap key
Any type that implements the Eq and Hash traits can be a key in HashMap. This includes:

    bool (though not very useful since there is only two possible keys)
    int, uint, and all variations thereof
    String and &str (tips: you can have a HashMap keyed by String and call .get() with an &str)

Note that f32 and f64 do not implement Hash, likely because floating-point precision errors would make using them as hashmap keys horribly error-prone.

All collection classes implement Eq and Hash if their contained type also respectively implements Eq and Hash. For example, Vec<T> will implement Hash if Timplements Hash.

## From / Into

The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types.

The From and Into traits are inherently linked, and this is actually part of its implementation. It means if we write something like this: impl From<T> for U, then we can use let u: U = U::from(T) or let u:U = T.into().

The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, then the Into trait will be automatically implemented for the same type.

Using the Into trait will typically require the type annotations as the compiler is unable to determine this most of the time.
