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
//! <head>
//!   <title>Hello world</title>
//! </head>
//! <body>
//! <h1>Hello world!</h1>
//!   @for line in self.message.trim().lines() {
//!     <p>@line</p>
//!   }
//! </body>
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
//! # Differences from HTML
//!
//! HTML have weird combination of verboseness and ambiguity. While
//! Octanol adds placeholder to HTML, it also adds small flavor to HTML for
//! small improvement while keeping it familiar.
//!
//! ## C++ style comments
//!
//! As explained earlier, Octanol template can contain C++ style comments and
//! it will be treated normally like a comment. Additionally, you can use HTML
//! style comments.
//!
//! C++ style comments are more concise and generally easier to type.
//! Additionally, Rust also uses this style of comment, and hence the decision
//! to use this kind of style.
//!
//! ## Optional boilerplate
//!
//! When omitted, these boilerplate will be automatically added upon generation.
//!
//! - `<!DOCTYPE html>`
//! - `<html></html>`
//! - `<meta charset="utf-8">`
//!
//! These are already optional in HTML. However, such documents without those
//! boilerplate aren't guaranteed to be rendered properly.
//!
//! ## Always UTF-8
//!
//! This is more of an opinionated flavor. Octanol always assume it is
//! encoded in UTF-8. Explicitly setting `meta charset` to anything other than
//! UTF-8 will result in error.
//!
//! This decision is made to match Rust's UTF-8 encoding to strings.
//!
//! ## `<head>` and `<body>` is required
//!
//! Yeah thats right. TODO more proper explanation.
//!
pub mod engine;
pub mod parser;
