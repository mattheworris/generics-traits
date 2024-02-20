<style>
.reveal code.rust {
  font-size: 0.75em;
  line-height: 1em;
}
</style>
<style type="text/css">
    p { text-align: left; }
</style>

## Understanding Traits in Rust
### Harnessing Polymorphism and Code Reuse in Rust
#### Matthew Orris
#### February 20, 2024

notes:
:root {--r-code-font: "FiraCode Nerd Font";}

---
### What are Traits?

 >A **trait** defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.

  -- [The Rust Programming Language[^1]](https://rust-book.cs.brown.edu/ch10-02-traits.html)

**Key Point**: 
Traits allow different types to implement the same functionality.
- [^1]: I recommend the Rust Book Experiment version from Brown University linked here
notes:
- Explain what traits are in Rust and their role in enabling polymorphism and code reuse.
- start with the definition of trait
- similar to interfaces
---
### Defining Traits
Examples 

```rust
// From The Rust Programming Language
pub trait Summarizeable {
    fn summarize(&self) -> String;
}

/// Defines the behavior of a fungible token.
pub trait Fungible {
	fn set_balance(&mut self, owner: &Address, amount: Balance);

	fn transfer(&mut self, from: &Address, to: &Address, amount: Balance);

	fn balance_of(&self, owner: &Address) -> Balance;
}
```

notes:
- We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we’ll request that summary by calling a `summarize` method on an instance.
- private to module by default
---
### Implementing Traits

```rust
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summarizeable for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
}

impl Summarizeable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
notes:
- continuing example from The Rust Book
- Illustrates different implementations for different types

---
### Where can traits be implemented?
#### The Orphan Rule
**The Orphan Rule states:**

You can implement a trait for a type only if either the trait or the type is defined in your own crate. This means:

 - Implementing your own trait for any type:  **Yes**
 - Implementing any trait for your own type: **Yes**
 - Implementing an external trait for an external type:  **No**
--
**Why this rule?**

- **Prevents Conflicts**: It stops two crates from implementing the same trait for the same type, which would confuse Rust on which implementation to use.
- **Ensures Compatibility**: It helps maintain compatibility across different versions of crates.

notes:
- named so because the parent is missing
- the orphan rule see chapter 10
---
### Default Implementations
```rust
// Trait definition without a Default Implementation
pub trait Summarizeable {
    fn summarize(&self) -> String;
}

// Trait definition with a Default Implementation
pub trait Summarizeable {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
notes:
- Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.
- impl block omitted to get default implementation
- more in live-coding examples
--
### Default Trait
```rust
/// Returns the default value of `0`
impl Default for u32 {
	fn default() -> u32;
}

// In usage:
let zero = Default::default();

```
- The Default trait allows you to create a default value for a type.
notes:
---
### Traits as Parameters or Trait Bounds
```rust
// The notify function requires the Summarizeable trait
pub fn notify(item: &impl Summarizeable) {
    println!("Breaking news! {}", item.summarize());
}

// Equivalent syntax with generic trait bounds
pub fn notify<T: Summarizeable>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Example syntax with where clauses and + to specify multiple traits
pub fn notify<T>(item: &T)
where
	T: Summarizeable + Debug + Clone
{
    println!("Breaking news! {}", item.summarize());
}
```
`notify()` can accept any type `T` that implements the `Summarizeable` trait.
notes:
- item is a parameter whose type is specified by a generic trait bound
- Trait bounds are used to specify that a generic type must implement a particular trait. Trait bounds enable polymorphic functions and structs that can work with any type that implements a specified trait.
---
### Associated Types
```rust
// Using Type Aliases (Address, Balance) defined outside the trait
pub trait Fungible {
	fn set_balance(&mut self, owner: &Address, amount: Balance);

	fn transfer(&mut self, from: &Address, to: &Address, amount: Balance) \
	 -> Result<(), String>;

	fn balance_of(&self, owner: &Address) -> Balance;
}

// Using Associated Types defined inside the trait
pub trait Fungible {
	// Associated Types
	type Address;
	type Balance;

	fn set_balance(&mut self, owner: &Self::Address, amount: Self::Balance);

	fn transfer( &mut self, from: &Self::Address, to: &Self::Address, \
	 amount: Self::Balance) -> Result<(), String>;

	fn balance_of(&self, owner: &Self::Address) -> Self::Balance;
}
```
notes:
- type placeholder for trait definitions
- first exercise to show changes
- impl blocks must define a type for the Associated Types
---
### Using Generics with Traits
```rust
// Using Generics to replace Associated Types (Address, Balance)
pub trait Fungible<Address, Balance>
{
	fn set_balance(&mut self, owner: &Address, amount: Balance);

	fn transfer(&mut self, from: &Address, to: &Address, amount: Balance);

	fn balance_of(&self, owner: &Address) -> Balance;
}
```
notes:
- What if we want to use the Fungible trait on token and coin, what does that look like?
- How much can we implement in default functions?
- second exercise?
---
### Derivable Traits
Examples
```rust
// From The Rust Programming Language
#[derive(Debug, Clone, PartialEq)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// From the Capacity Pallet, using both std lib and parity traits
#[derive(
	Clone, Copy, Debug, Decode, Encode, TypeInfo, Eq, MaxEncodedLen,
	PartialEq, PartialOrd,
)]
/// The type of staking a given Staking Account is doing.
pub enum StakingType {
	MaximumCapacity,
	ProviderBoost,
}
```
[The Rust Programming Language: Appendix C: Derivable Traits](https://rust-book.cs.brown.edu/appendix-03-derivable-traits.html#appendix-c-derivable-traits)
notes:
- struct or enum
- Only traits from the standard library that are explicitly designed to be derivable
- libraries can implement derive for their own traits
- derive is a procedural macro
- https://doc.rust-lang.org/std/primitive.u32.html
- Display cannot be derived
---
### SuperTraits
```rust
use std::fmt::Display;

trait Printable: Display {
    fn print(&self) {
        println!("{}", self.to_string());
    }
}
```
In this case, for any type to implement `Printable`, it must also implement `Display`, ensuring that `to_string()` can be called, which `print` relies on.

notes:
- Supertraits are particularly useful in generic programming and when building abstractions that require certain behaviors from types. They allow trait authors to build upon existing traits rather than duplicating functionality or forcing trait users to implement multiple, potentially unrelated methods.
--
### Benefits

- **Code Reuse**: Supertraits allow for the reuse of existing traits, reducing code duplication.
- **Clarity**: They make trait dependencies explicit, improving code readability and understanding.
- **Flexibility**: Supertraits enable the composition of traits to define complex behaviors with clear requirements.

notes:
- Supertraits provide a powerful tool for designing modular and reusable code in Rust, allowing developers to build sophisticated type behaviors based on existing traits while ensuring consistency and type safety across implementations.---

---
### Live Coding Examples
[Traits-Playground](https://github.com/mattheworris/generics-traits)

---
### Best Practices

- **Cohesion**: Implement traits that logically group related functionality, making your code more modular and reusable.
    
- **Extension**: Use traits to extend existing types with new functionality, including types from the standard library or other crates.
--
- **Composition Over Inheritance**: Rust's trait system encourages composition over inheritance, enabling flexible and maintainable code designs.
    
- **Trait Naming**: Follow Rust's naming conventions, often using adjectives or verbs that describe the trait's behavior (e.g., `Readable`, `Writeable`).

---
### Questions?
### Feedback
#### Even More Advanced Traits Topics?
- Dynamic Dispatch With Trait Objects
- Using Trait Bounds To Conditionally Implement Methods
- Default Generic Type Parameters and Operator Overloading
- Using the Newtype Pattern to Implement External Traits on External Types
notes:
-[Using Trait Bounds to Conditionally Implement Methods](https://rust-book.cs.brown.edu/ch10-02-traits.html?highlight=orphan#using-trait-bounds-to-conditionally-implement-methods)

---
