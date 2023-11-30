# Rust Hello World

rust hello world

## Docs

cargo add

```bash
cargo add serde serde_json -F serde/derive
```

### Generics

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

We read this definition as: the function `largest` is generic over some type `T`.
This function has one parameter named list, which is a slice of values of type `T`.
The largest function will return a reference to a value of the same type `T`.

### Traits

#### Trait Bounds

in the [code](src/generics/largest.rs)

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

or to use `where` clause

```rust
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

#### Trait Feynman Technique

**Feynman Technique to learning about trait bounds in Rust.**

1. **Choose the Concept:**
   Identify that you want to understand "trait bounds" in Rust.

2. **Teach the Concept Simply:**

   Pretend you are explaining trait bounds to someone who has no programming background:
   "In Rust, trait bounds are a way to specify constraints on the types that can be used with a particular function or method.
   Imagine you have a function that works with certain types, and you want to make sure it only works with types that have
   specific behavior or features. That's where trait bounds come in.
   You basically tell Rust, 'Hey, this function should only accept types that satisfy these conditions.'"

3. **Identify Gaps in Your Understanding:**

   While explaining, you might realize you're not entirely clear on how to define and use
   trait bounds or when to choose them over other approaches. These are the gaps you need to address.

4. **Review and Simplify:**

   Go back to your Rust documentation, tutorials, or any learning resources you have.
   Review how to define trait bounds, when to use them, and examples of their application.
   Simplify your explanation:
   "Trait bounds are like setting rules for the types a function
   can work with. You create these rules using traits, which define behavior. If a type
   follows those rules (implements the traits), it's good to go."

5. **Repeat the Process:**

   Practice explaining trait bounds to yourself or someone else, refining your explanation each time.
   Revisit specific scenarios where trait bounds are beneficial and understand how they
   contribute to Rust's type system.
   By going through these steps, you not only break down the concept into simpler terms
   but also ensure you understand the practical application of trait bounds in Rust. The
   key is to iterate through the process, addressing any gaps in your understanding until
   you can explain the concept clearly and confidently.

## Borrowing (Ref)

Rule

- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References must always be valid.

## Deref

use \*y to follow the reference to the value it’s pointing to (hence dereference) so the compiler can compare the actual value.

`*` - dereference operator
`&` - reference operator (**they allow you to refer to some value without taking ownership of it**), action of creating a reference is called _borrowing_

## a.clone vs Rc::clone(&a) in rust

In Rust, `a.clone()` and `Rc::clone(&a)` may seem similar at first, but they have different implications and use cases.

1. **`a.clone()`**

   The `clone` method is part of the `Clone` trait in Rust. When you call `a.clone()`, you are invoking the `clone` method on the value `a`. This method is used to create a deep copy of the value. It's important to note that not all types implement the `Clone` trait by default. If a type does not implement `Clone`, you won't be able to use `clone` directly unless you implement `Clone` for that type yourself.

   Example:

   ```rust
   #[derive(Clone)]
   struct MyStruct {
       // fields...
   }

   let a = MyStruct { /* initialize fields */ };
   let b = a.clone();  // Creates a deep copy of 'a'.
   ```

2. **`Rc::clone(&a)`**

   The `Rc` type in Rust is a reference-counted smart pointer, and it stands for "reference counting." It allows multiple ownership of data by keeping track of how many references exist to a particular piece of data. The `Rc::clone` method is used to increase the reference count of an `Rc` instance.

   Example:

   ```rust
   use std::rc::Rc;

   struct MyStruct {
       // fields...
   }

   let a = Rc::new(MyStruct { /* initialize fields */ });
   let b = Rc::clone(&a);  // Increases the reference count of 'a'.
   ```

   In this case, `Rc::clone` does not create a deep copy of the data but instead increases the reference count. Both `a` and `b` now point to the same piece of data, and the reference count has been incremented. When the last reference to the data is dropped, the data is deallocated.

In summary, `a.clone()` is used to create a deep copy of a value, while `Rc::clone(&a)` is used to increase the reference count of a reference-counted smart pointer (`Rc`). The choice between them depends on whether you need a completely independent copy of the data or if you want to share ownership through reference counting.

## Supertraits

https://doc.rust-lang.org/reference/items/traits.html#supertraits

```rust
#[async_trait::async_trait]
pub trait PoolFetching: Send + Sync {
    async fn fetch(&self, token_pairs: HashSet<TokenPair>, at_block: Block) -> Result<Vec<Pool>>;
}
```

send+sync is supertrait

- The `Send` trait indicates that a value of this type is safe to send from one thread to another.
- The `Sync` trait indicates that a value of this type is safe to share between multiple threads.

detailed explanation: [extensible-concurrency-with-the-sync-and-send-traits](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
```
