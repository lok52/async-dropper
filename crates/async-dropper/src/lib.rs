//! `async_dropper` provides two ad-hoc implementations of asynchronous `drop` behavior (`AsyncDrop`).
//!
//! - `async_dropper::simple` uses a wrapper struct (suggested on [StackOverflow by paholg](https://stackoverflow.com/a/75584109))
//! - `async_dropper::derive` introduces a `derive` macro (`AsyncDrop`) which generates code that enables async drop functionality to run during a regular `drop`. That code requires `T: Default + PartialEq + Eq`.
//!
//! Here is a quick example of the shorter `async_dropper::simple`:
//!
//! ```
//! /// This object will be async-dropped (which must be wrapped in AsyncDropper)
//! #[derive(Default)]
//! struct AsyncThing(String);
//!
//! #[async_trait]
//! impl AsyncDrop for AsyncThing {
//!     async fn async_drop(&mut self) {
//!         tokio::time::sleep(Duration::from_secs(2)).await; // fake work
//!         Ok(())
//!     }
//! }
//!
//! #[your_async_runtime_of_choice::main] // i.e. tokio::main or async_std::main
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     drop(AsyncDropper::new(AsyncThing(String::from("test"))));
//!     Ok(())
//! }
//! ```
//!
//! Note that `AsyncThing` must be wrapped in `async_dropper::simple::AsyncDropper`.
//!
//! For `async_dropper::derive`, the simplest example looks like this:
//!
//! ```
//! use async_dropper::derive::AsyncDrop;
//!
//! // Your struct (named field structs and tuple structs both work)
//! #[derive(Debug, Default, PartialEq, Eq, AsyncDrop)]
//! struct AsyncThing(String);
//!
//! /// How it drops, asynchrounously
//! #[async_trait]
//! impl AsyncDrop for AsyncThing {
//!     async fn async_drop(&mut self) -> Result<(), AsyncDropError> {
//!         tokio::time::sleep(Duration::from_secs(2)).await; // fake work
//!         Ok(())
//!     }
//! }
//!
//! #[your_async_runtime_of_choice::main] // i.e. tokio::main or async_std::main
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     drop(AsyncThing(String::from("test")));
//!     Ok(())
//! }
//! ```
//!
//! The `async_dropper::derive` is interesting because it attempts to automatically (if somewhat painfully) determine
//! whether an object should have the defined asynchronous drop behavior performed by checking whether it is equal
//! to `Self::default()`.
//!
//! **Said differently, async drop behavior is skipped if an instance of `T` is exactly equal to a `T::default()`**.
//!
//! For convenience, a `reset(&mut self)` function that sets any T to T::default() is automatically derived, but it can be overriden via `AsyncDrop#reset()`.
//!
//! If `T` is *not* exactly equal to `T::default()`, the assumption is that `T` must still be holding things that require asynchronous dropping behavior, and as such that behavior will be performed.
//!
//! **If `reset(&mut self)` does not return `T` to a state where it is equal to `T::default()`, `drop` will panic**

pub use async_dropper_derive as derive;
pub use async_dropper_simple as simple;
