# RUST-Tutorials

This folder is created to learn the Rust programming language from scratch and develop various projects. It includes examples and notes covering fundamental concepts to advanced topics.


## Current Progress

###  [**Basics**](./basics/src):
Beginner-level topics with examples and explanations.

- **Data Types**: Understanding primitive and compound types in Rust.
- **Functions**: Defining and calling functions, understanding parameter passing.
- **Strings**: Understanding string handling and manipulation in Rust.
- **Vectors**: Working with dynamic arrays and vector manipulation.
- **Debugging**: Techniques for debugging Rust code effectively.
- **Ownership**: Learning the ownership system, borrowing, and references.
- **Structs**: Creating and using structs to model data.
- **Lifetimes**: Understanding lifetimes and their role in Rust's memory safety.

---

###  [**Intermediate**](./intermediate/src):
Concepts to help you write idiomatic, efficient, and reusable Rust code.

- **built_in_traits.rs**: Examples of built-in traits like `Copy`, `Clone`, `Debug`, `Drop`, `Default`, `Display`, and `FromStr` with use cases.
- **closures.rs**: Functional closures, capturing environment variables, and closure types with examples.
- **enums.rs**: Enum declarations and their use in pattern matching, logic flow, and method implementations.
- **error_handling.rs**: Techniques for managing errors using `Result`, `Option`, `unwrap`, `expect`, and `?` operator.
- **generics.rs**: Writing functions, structs, and enums using generic type parameters.
- **pattern_matchings.rs**: Advanced pattern matching with `match`, `if let`, `while let`, and destructuring.
- **traits.rs**: Defining and implementing custom traits, trait bounds, and polymorphism.
- **where_clause.rs**: Cleaner generic code using `where` clause for trait bounds.
- **tests.rs**: Writing unit tests with `#[test]`, using assertions, and testing edge cases.
- **main.rs**: Entry point combining various intermediate modules for demonstration purposes.

---

###  [**Advanced**](./advanced/src):
In-depth topics to build robust, performant, and concurrent Rust applications.

- **asynchronous_programming/**: Working with async/await, and using runtimes like `tokio` for asynchronous tasks.
- **channels/**: Using Rust channels (`mpsc`, `sync`) for safe message passing between threads.
- **concurrency/**: Managing shared state across threads with locks and `Arc`; thread spawning and coordination.
- **deadlock_and_poisoning/**: Handling deadlocks and poisoned mutexes in multithreaded programs.
- **macros/**: Writing declarative macros (`macro_rules!`) for reusable, DRY code generation.
- **parallelism/**: Implementing parallel computations using crates like `rayon` for data-parallelism.
- **smart_pointers/**: Deep dive into `Box`, `Rc`, `RefCell`, `Arc`, and their interior mutability patterns.
- **streams/**: Using asynchronous streams and sinks with `futures` and `tokio-stream` for data pipelines.
- **unsafe_zone/**: Using `unsafe` blocks responsibly, working with raw pointers, FFI, and low-level operations.



