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
