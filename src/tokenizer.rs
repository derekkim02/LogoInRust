//! # Tokenizer
//! 
//! This module contains the tokenizer for the Logo language.
//! It is responsible for breaking down the input Logo code into a series of tokens that can be parsed by the interpreter.
//! The main functionality is provided by the `tokenize` function (not shown in the excerpt),
//! which takes a string and returns an iterator of tuples containing a `Token` and a `Range<usize>`.
//! 
//! The `Token` enum is defined using the `logos` crate. Each variant of the `Token` enum represents a different kind of token that can appear in Logo code.
//! # Example
//! 
//! ```
//! use rslogo::tokenizer::{Token, tokenize};
//! 
//! let source_code = "PENUP FORWARD \"100";
//! let tokens = tokenize(source_code).map(|(token, _range)| token);
//! assert_eq!(tokens.collect::<Vec<_>>(), vec![Token::PenUp, Token::Forward, Token::Value("100".to_string())]);
//! ```
//! 
//! This example tokenizes a string of Logo code and prints each token along with its range in the original string.
use std::ops::Range;
use logos::Logos;


/// # Implementation
/// 
/// The `Token` enum is derived from the `Logos` trait, which is provided by the `logos` crate. 
/// This trait allows the enum to be used as a lexer, which can be created using the `Token::lexer` method. 
/// The `Token` enum also implements the `Debug`, `PartialEq`, `Clone`, `Hash`, `Eq`, `Ord`, and `PartialOrd` 
/// traits to enable comparison and debugging of the tokens.
/// 
/// The `#[token("...")]` attribute is used to define the keywords and symbols that the tokenizer should recognize.
/// The `#[regex("...")]` attribute is used to define regular expressions that the tokenizer should match.
/// The `#[regex(r"//.*\n", logos::skip)]` attribute is used to skip comments in the input string.
/// The `#[regex(r"[ \t\n\f]+", logos::skip)]` attribute is used to skip whitespace characters in the input string.
/// 
#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq, Ord, PartialOrd)]
pub enum Token {

	/// The `Error` variant is used to represent an error token when the tokenizer encounters an unknown token.
    Error,
	/// The `PenUp` variant is used to represent the `PENUP` keyword in Logo code.
	#[token("PENUP")]
	PenUp,

	/// The `PenDown` variant is used to represent the `PENDOWN` keyword in Logo code.
	#[token("PENDOWN")]
	PenDown,

	/// The `Forward` variant is used to represent the `FORWARD` keyword in Logo code.
	#[token("FORWARD")]
	Forward,

	/// The `Back` variant is used to represent the `BACK` keyword in Logo code.
	#[token("BACK")]
	Back,

	/// The `Left` variant is used to represent the `LEFT` keyword in Logo code.
	#[token("LEFT")]
	Left,

	/// The `Right` variant is used to represent the `RIGHT` keyword in Logo code.
	#[token("RIGHT")]
	Right,

	/// The `SetPenColor` variant is used to represent the `SETPENCOLOR` keyword in Logo code.
	#[token("SETPENCOLOR")]
	SetPenColor,

	/// The `Turn` variant is used to represent the `TURN` keyword in Logo code.
	#[token("TURN")]
	Turn,

	/// The `SetHeading` variant is used to represent the `SETHEADING` keyword in Logo code.
	#[token("SETHEADING")]
	SetHeading,

	/// The `SetX` variant is used to represent the `SETX` keyword in Logo code.
	#[token("SETX")]
	SetX,

	/// The `SetY` variant is used to represent the `SETY` keyword in Logo code.
	#[token("SETY")]
	SetY,

	/// The `Make` variant is used to represent the `MAKE` keyword in Logo code.
	#[token("MAKE")]
	Make,

	/// The `AddAssign` variant is used to represent the `ADDASSIGN` keyword in Logo code.
	#[token("ADDASSIGN")]
	AddAssign,

	/// The `Value` variant is used to represent a value in Logo code.
	#[regex(r#""[^\s"]*"#, |lex| lex.slice()[1..].to_string())]
    Value(String),

	/// The `Variable` variant is used to represent a variable in Logo code.
	#[regex(r#":[^\s"]*"#, |lex| lex.slice()[1..].to_string())]
	Variable(String),

	/// The `XCor` variant is used to represent the `XCOR` Query in Logo code.
	#[token("XCOR")]
	XCOR,

	/// The `YCor` variant is used to represent the `YCOR` Query in Logo code.
	#[token("YCOR")]
	YCOR,

	/// The `Heading` variant is used to represent the `HEADING` Query in Logo code.
	#[token("HEADING")]
	HEADING,

	/// The `Color` variant is used to represent the `COLOR` Query in Logo code.
	#[token("COLOR")]
	COLOR,
	
	/// The `If` variant is used to represent the `IF` keyword in Logo code.
	#[token("IF")]
	If,

	/// The `While` variant is used to represent the `WHILE` keyword in Logo code.
	#[token("WHILE")]
	While,

	/// The `Equal` variant is used to represent the `EQ` keyword in Logo code.
	#[token("EQ")]
	Equal,

	/// The `NotEqual` variant is used to represent the `NE` keyword in Logo code.
	#[token("NE")]
	NotEqual,

	/// The `LessThan` variant is used to represent the `LT` keyword in Logo code.
	#[token("LT")]
	LessThan,

	/// The `GreaterThan` variant is used to represent the `GT` keyword in Logo code.
	#[token("GT")]
	GreaterThan,

	/// The `And` variant is used to represent the `AND` keyword in Logo code.
	#[token("AND")]
	And,

	/// The `Or` variant is used to represent the `OR` keyword in Logo code.
	#[token("OR")]
	Or,

	/// The `LParen` variant is used to represent the `[` symbol in Logo code.
	#[token("[")]
	LParen,

	/// The `RParen` variant is used to represent the `]` symbol in Logo code.
	#[token("]")]
	RParen,

	/// The `Add` variant is used to represent the `+` symbol in Logo code.
	#[token("+")]
	Add,

	/// The `Sub` variant is used to represent the `-` symbol in Logo code.
	#[token("-")]
	Sub,

	/// The `Mul` variant is used to represent the `*` symbol in Logo code.
	#[token("*")]
	Mul,

	/// The `Div` variant is used to represent the `/` symbol in Logo code.
	#[token("/")]
	Div,

	/// The `Ignored` variant is used to represent whitespace, comments and newlines which are ignored in Logo code.
	#[regex(r"//.*\n", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Ignored,
}

/// The `tokenize` function takes a string slice as input and returns an iterator of tuples containing a `Token` and a `Range<usize>`.
pub fn tokenize(content: &str) -> impl Iterator<Item = (Token, Range<usize>)> + '_{
	let token_iter = Token::lexer(content)
		.spanned()
		.map(|(token, span)| match token {
			Ok(token) => (token, span),
			Err(()) => (Token::Error, span),
		});
	token_iter
}