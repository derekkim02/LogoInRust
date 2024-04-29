//! # RSLogo
//! 
//! This crate provides a Logo interpreter written in Rust. 
//! It includes a parser, tokenizer, and an abstract syntax tree (AST) for the Logo language.
//! 
//! 

/// The abstract syntax tree (AST) for the Logo language.
pub mod ast;

/// The parser for the Logo language.
pub mod parser;

/// The turtle graphics engine for the Logo language.
pub mod turtle;

/// The uncertain boolean type.
pub(crate) mod uncertain_bool;

/// The tokenizer for the Logo language.
pub mod tokenizer;
