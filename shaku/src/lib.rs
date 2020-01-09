//! # What is Dependency Injection (aka. Dependency Inversion)?
//!
//! The idea behind inversion of control is that, rather than tie the classes in your application
//! together and let classes “new up” their dependencies, you switch it around so dependencies are
//! instead passed in during class construction. It's one of the 5 core principles of
//! [SOLID programming](https://en.wikipedia.org/wiki/SOLID_(object-oriented_design))
//!
//! If you want to read more on that:
//! - [Martin Fowler has an excellent article explaining dependency injection/inversion of control](http://martinfowler.com/articles/injection.html)
//! - [Wikipedia article on dependency inversion principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle)
//!
//! # Getting started
//! ## Structure your application
//! Start by writing a classical application with struct & types (in homage to
//! [AutoFac](https://autofac.org/) I ported their classical "getting started" example).
//! Code excerpts are used below to illustrate this little guide, the complete example is available
//! [here](https://github.com/bgbahoue/he-di/blob/master/examples/autofac/src/main.rs).
//!
//! ```rust
//! use std::sync::Arc;
//!
//! trait IOutput {
//!     fn write(&self, content: String);
//! }
//!
//! struct ConsoleOutput {
//!     prefix: String,
//!     other_param: usize,
//! }
//!
//! impl IOutput for ConsoleOutput {
//!     fn write(&self, content: String) {
//!         println!("{} #{} {}", self.prefix, self.other_param, content);
//!     }
//! }
//!
//! trait IDateWriter {
//!     fn write_date(&self);
//! }
//!
//! struct TodayWriter {
//!     output: Arc<dyn IOutput>,
//!     today: String,
//!     year: usize,
//! }
//!
//! impl IDateWriter for TodayWriter {
//!     fn write_date(&self) {
//!        let mut content = "Today is ".to_string();
//!        content.push_str(self.today.as_str());
//!        content.push_str(" ");
//!        content.push_str(self.year.to_string().as_str());
//!        self.output.write(content);
//!     }
//! }
//! ```
//!
//! ## Inherit "Interface" for the interface traits
//!
//! Interface traits require certain bounds, such as `'static` and optionally `Send + Sync` if using
//! the `thread_safe` feature. The `Interface` trait acts as a trait alias for these bounds, and is
//! automatically implemented on types which implement the bounds.
//!
//! In our example, the two interface traits would become:
//!
//! ```rust,ignore
//! trait IOutput: Interface {
//!     fn write(&self, content: String);
//! }
//!
//! trait IDateWriter: Interface {
//!     fn write_date(&self);
//! }
//! ```
//!
//! ## Mark structs as Component
//! A component is an expression or other bit of code that exposes one or more services and can take
//! in other dependencies.
//!
//! In our example, we have 2 components:
//!
//! - `TodayWriter` of type `IDateWriter`
//! - `ConsoleOutput` of type `IOutput`
//!
//! To be able to identify them as components [shaku](https://crates.io/crates/shaku) exposes a
//! `#[derive()]` macro (though the [shaku_derive](https://crates.io/crates/shaku_derive) crate).
//! It is simply done using the following attributes:
//!
//! ```rust,ignore
//! #[derive(Component)] // <--- mark as a Component
//! #[interface(IOutput)] // <--- specify the type of this Component
//! struct ConsoleOutput {
//!     prefix: String,
//!     other_param: usize,
//! }
//! ```
//!
//! ## Express dependencies
//! Some components can have dependencies to other components, which allows the DI logic to also
//! inject these components with another Component.
//!
//! In our example, `ConsoleOuput` is a Component with no dependency and `TodayWriter` a Component
//! with a dependency to a `IOutput` Component.
//!
//! To express this dependency, use the `#[inject]` attribute within your struct to flag the
//! property and declare the property as a
//! [trait object](https://doc.rust-lang.org/book/first-edition/trait-objects.html) wrapped in an
//! [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html).
//!
//! In our example:
//!
//! ```rust,ignore
//! use shaku_derive::Component;
//!
//! #[derive(Component)] // <--- mark a struct as a Component that can be registered & resolved
//! #[interface(IDateWriter)] // <--- specify which interface it implements
//! struct TodayWriter {
//!     #[inject] // <--- flag 'output' as a property which can be injected
//!     output: Arc<dyn IOutput>, // <--- trait object using the interface `IOutput`
//!     today: String,
//!     year: usize,
//! }
//! ```
//!
//! ## Application startup
//! At application startup, you need to create a [ContainerBuilder](struct.ContainerBuilder.html)
//! and register your components with it.
//!
//! In our example, we register `ConsoleOutput` and `TodayWriter` with a `ContainerBuilder` doing
//! something like this:
//!
//! ```rust,ignore
//! // Create your builder.
//! let mut builder = ContainerBuilder::new();
//!
//! builder.register_type::<ConsoleOutput>();
//! builder.register_type::<TodayWriter>();
//!
//! // Create a Container
//! let mut container = builder.build().unwrap();
//! ```
//!
//! The `Container` reference is what you will use to resolve types & components later. It can then
//! be stored as you see fit.
//!
//! ## Application execution
//! During application execution, you’ll need to make use of the components you registered. You do
//! this by resolving them from a `Container` with one of the 3 `resolve()` methods.
//!
//! ### Passing parameters
//! In most cases you need to pass parameters to a Component. This can be done when
//! registering a Component into a [ContainerBuilder](struct.ContainerBuilder.html).
//!
//! You can register parameters either using their property name or their property type. In the
//! later case, you need to ensure that it is unique.
//!
//! Passing parameters is done using the `with_named_parameter()` or
//! `with_typed_parameter()` chained methods like so:
//!
//! ```rust,ignore
//! builder
//!     .register_type::<ConsoleOutput>()
//!     .with_named_parameter("prefix", "PREFIX >".to_string())
//!     .with_typed_parameter::<usize>(117 as usize);
//! ```
//!
//! ## Dependency Injection in Action
//! For our sample app, we created a `write_date()` method to resolve the writer from a Container:
//!
//! ```rust,ignore
//! fn write_date(container: &Container) {
//!     let writer = container
//!         .resolve::<dyn IDateWriter>()
//!         .unwrap();
//!     writer.write_date();
//! }
//!
//! let mut builder = ContainerBuilder::new();
//! builder
//!     .register_type::<ConsoleOutput>()
//!     .with_named_parameter("prefix", "PREFIX >".to_string())
//!     .with_typed_parameter::<usize>(117 as usize);
//! builder
//!     .register_type::<TodayWriter>()
//!     .with_typed_parameter::<String>("June 20".to_string())
//!     .with_typed_parameter::<usize>(2017 as usize);
//!
//! let container = builder.build().unwrap();
//!
//! write_date(&container);
//! ```
//!
//! Now when you run your program...
//!
//! - The components and their parameters will be registered in the `ContainerBuilder`.
//! - `builder.build()` will create the registered components in order of dependency
//!   (first `ConsoleOutput`, then `TodayWriter`). These components will be returned in the
//!   `Container`.
//! - The `write_date()` method asks the `Container` for an `IDateWriter`.
//! - The `Container` sees that `IDateWriter` maps to `TodayWriter`, and it returns the component.
//!
//! Later, if we wanted our application to write a different date, we would just have to implement a
//! different `IDateWriter` and then change the registration at app startup. We won’t have to change
//! any other classes. Yay, inversion of control!
//!
//! ## Roadmap
//! The current implementation of this crate is still WIP. A few identified useful to know
//! limitations (being further explorer) are:
//!
//! - `#[derive(Component)]` should be tested against complex cases & more tests are to be written
//!   (e.g, struct with lifetime, generics, ...)
//! - we should support closures as a way to create parameters (at register or resolve time)

// Linting
#![deny(unused_must_use)]

// Reexport of [anymap](https://crates.io/crates/anymap)
#[doc(hidden)]
pub extern crate anymap;
#[macro_use]
extern crate log;

// Reexport Error type from shaku_internals
pub use shaku_internals::error::Error;

// Shortcut to main types / traits
pub use crate::component::Component;
pub use crate::component::Interface;
pub use crate::container::Container;
pub use crate::container::ContainerBuilder;
pub use crate::container::Dependency;
pub use crate::result::Result;

pub mod component;
pub mod container;
pub mod parameter;

// Main DI Result type mapping
#[doc(hidden)]
pub mod result {
    /// Alias for a `Result` with the error type [shaku::Error](enum.Error.html)
    pub type Result<T> = ::std::result::Result<T, shaku_internals::error::Error>;
}
