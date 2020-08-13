//! A template engine like razor engine in rust.
//!
//! ```ignore
//! use octanol::Model;
//!
//! #[derive(Model)]
//! #[template = include_str!("hello_world.rshtml")]
//! struct HelloWorld {
//!     message: String,
//! }
//! ```
//!
//! ```html
//! <h1>Hello world!</h1>
//! <p>@self.message</p>
//! ```
//!
//! # Syntax
//!
//! An Octanol template file is a mix of HTML and Rust code.
//!
//! > Warning: Octanol is still work in progress, these syntaxes are subject to
//! > change without prior notice.
//!
//! ## Comment
//!
//! You can use both single and multi-line C++ style comments, as well
//! as HTML comments.
//!
//! ```html
//! // single line comment
//! /*
//!   multi-line comment
//! */
//! <!--
//!   another multi-line comment
//! -->
//! ```
//!
//! ## Placeholder
//!
//! Placeholders starts with `@` and contains expression similar to Rust's.
//!
//! ### Basic placeholder
//!
//! ```html
//! @self.message
//! @self.message.to_upper()
//! ```
//!
//! On surface, these can contain limited form of Rust expression:
//!
//! - literal
//! - variable
//! - property access
//! - index access
//! - function call
//! - macro invocation
//! - grouping
//! - block expression
//! - reference and dereference
//!
//! Parentheses, square brackets, or curly brackets can contain
//! more complex expressions. Although, if it gets wildly complex, consider
//! moving the logic to rust files.
//!
//! Moreover, the expression must evaluate to a value that implements `Display`.
//!
//! ### If placeholder
//!
//! ```html
//! @if let Some(message) = self.message {
//!   <p>@message</p>
//! }
//! ```
//!
//! These are similar to Rust's if expression, the difference is that the body
//! must be Octanol template.
//!
//! You can use `if let` for pattern matching and it can be chained with
//! `else`s.
//!
//! ### For placeholder
//!
//! ```html
//! @for people in self.peoples {
//!   <p>@people.name's favorite is @people.favorite</p>
//! }
//! ```
//!
mod lexer;
