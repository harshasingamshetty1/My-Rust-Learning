# Closures

Closures can capture variables by borrowing or moving. But they prefer to capture by borrowing and only go lower when required:

    By reference: &T
    By mutable reference: &mut T
    By value: T

If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the `move` keyword before the parameter list.

EX 2

```rs

/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}
```

SOl 2

```rs
/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;
    /*
        Here, we are using move, and as i32 implements copy trait, it moves a copy of count to the closure
     */
    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}
```

## Fn, FnMut, FnOnce

When taking a closure as an input parameter, the closure's complete type must be annotated using one of the following traits:

    Fn: the closure uses the captured value by reference (&T)
    FnMut: the closure uses the captured value by mutable reference (&mut T)
    FnOnce: the closure uses the captured value by value (T)

```rs
fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/* Fill in the blank */
fn exec<'a, F: __>(mut f: F)  {
    f("hello")
}
```

Sol 6

```rs

fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/*
    here, the closure is mutating the string, so it needs to be FnMut
    and its not returning anything
*/
fn exec<'a, F: FnMut(&'a str) >(mut f: F)  {
    f("hello")
}
```

EX 7

```rs
/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: __ {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

Sol 7

```rs
/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

    Move closures may still implement Fn or FnMut, even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them. The move keyword only specifies the latter.

    closures can only be coerced to `fn` types if they do not capture any variables

## Clarifying btween move keyword and fn trait bounds

Basically, move defines, how the value is being captured, i.e whether it is being borrowed or taking ownership.

Whereas, the trait bounds Fn, FnMut, FnOnce define, how the closure is being used within the closure, i.e whether it is being called by value or by reference.

So, there can be cases where even though we use move, but the Fn trait bound is sufficient, if the closure, does not modify the variables

EX 11

```rs

/* Fill in the blank and fix the error*/
fn factory(x:i32) -> __ {

    let num = 5;

    if x > 1{
        move |x| x + num
    } else {
        move |x| x + num
    }
}
```

Sol 11

```rs
/* Fill in the blank and fix the error*/
fn factory(x:i32) -> Box<dyn Fn(i32)->i32> {

    let num = 5;
    /*
        here, we must use the dynamic dispatch, bcoz any 2 closures have the same type, even though they are identical.
        thereore, as the compiler cannot know the size of the return type at compile time, we must use the dymanic dispatch
        By converting into the Box type
     */
    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}
fn main(){}
```
