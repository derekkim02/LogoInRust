//! # Parser
//! 
//! This module contains the parser for the Logo language.
//! Contains the `parse_content` function that takes a string and returns a vector of `ASTNode`s.
//! The `ASTNode` enum is defined in the `ast` module.
//! 
//! The parser is implemented using the `chumsky` crate, which is a parser combinator library.

use chumsky::{prelude::*, Stream};
use regex::Regex;

use crate::tokenizer::{tokenize, Token};
use crate::ast::{ASTNode, ControlFlow, Condition, Expression, Procedure, Query, Math};

/// Parses the content of a Logo file and returns a vector of `ASTNode`s.
/// If the content is invalid, returns a vector of `Simple<Token>` containing the errors.
/// 
/// # Arguments
/// 
/// * `content` - A string slice containing the content of the Logo file.
/// 
/// # Example
/// 
/// ```
/// use rslogo::parser::parse_content;
/// use rslogo::ast::{ASTNode, Procedure, Expression};
/// 
/// let content = "MAKE \"x \"10\nFORWARD :x";
/// let asts = parse_content(content).unwrap();
/// 
/// assert_eq!(asts.len(), 2);
/// 
/// ```
/// 
/// 
pub fn parse_content(content:&str) -> Result<Vec<ASTNode>, Vec<Simple<Token>>>{
	let token_iter = tokenize(content);
	let token_stream = Stream::from_iter(content.len()..content.len(), token_iter);
	let asts = parser().parse(token_stream)?;
	Ok(asts)
}

fn parser() -> impl Parser<Token, Vec<ASTNode>, Error = Simple<Token>> {
	// Helper parsers
	let value = select! {
		Token::Value(s) if Regex::new(r"^-?[0-9]*\.?[0-9]+$").unwrap().is_match(&s) => Expression::Float(s.parse().unwrap()),
		Token::Value(s) if Regex::new(r"[A-Za-z]+").unwrap().is_match(&s) => Expression::String(s)
	};
	let variable = select!(Token::Variable(s) => Expression::Variable(s));
	let query = select! {
		Token::XCOR => Expression::Query(Query::XCOR),
		Token::YCOR => Expression::Query(Query::YCOR),
		Token::HEADING => Expression::Query(Query::HEADING),
		Token::COLOR => Expression::Query(Query::COLOR),
	};

	// Recursive parsers
	let arg = recursive(|math| {
		let add = just(Token::Add);
		let sub = just(Token::Sub);
		let mul = just(Token::Mul);
		let div = just(Token::Div);

		let op = add.or(sub).or(mul).or(div);
		let body = math.clone()
			.then(math.clone());

		op.then(body)
			.try_map(|(token, (lhs, rhs)), _span| {
				match token {
					Token::Add => Ok(Expression::Math(Box::new(Math::Add(lhs, rhs)))),
					Token::Sub => Ok(Expression::Math(Box::new(Math::Sub(lhs, rhs)))),
					Token::Mul => Ok(Expression::Math(Box::new(Math::Mul(lhs, rhs)))),
					Token::Div => Ok(Expression::Math(Box::new(Math::Div(lhs, rhs)))),
					_ => unreachable!(),
				}
			}).or(value)
			.or(variable)
			.or(query)
	});

	let condition = recursive(|cond| {
		let equal = just(Token::Equal);
		let not_eequal = just(Token::NotEqual);
		let less_than = just(Token::LessThan);
		let greater_than = just(Token::GreaterThan);

		let math_cond =  equal
			.or(not_eequal)
			.or(less_than)
			.or(greater_than)
			.then(arg.clone()
				.then(arg.clone()))
			.try_map(|(token, (lhs, rhs)), _span| {
				match token {
					Token::Equal => Ok(Condition::Equal(lhs, rhs)),
					Token::NotEqual => Ok(Condition::NotEqual(lhs, rhs)),
					Token::LessThan => Ok(Condition::LessThan(lhs, rhs)),
					Token::GreaterThan => Ok(Condition::GreaterThan(lhs, rhs)),
					_ => unreachable!(),
				}
			});

		let and = just(Token::And);
		let or = just(Token::Or);

		let bool_cond = and
			.or(or)
			.then(cond.clone()
				.then(cond.clone()))
			.try_map(|(token, (lhs, rhs)), _span| {
				match token {
					Token::And => Ok(Condition::And(Box::new(lhs), Box::new(rhs))),
					Token::Or => Ok(Condition::Or(Box::new(lhs), Box::new(rhs))),
					_ => unreachable!(),
				}
			});

		math_cond.or(bool_cond)
	});

	// Procedure parsers
	let no_arg = arg.clone().not().rewind()
		.ignored()
		.or(end());
	let procedure_no_args = just(Token::PenUp)
		.or(just(Token::PenDown))
		.try_map(|token, _span| {
			match token {
				Token::PenUp => Ok(ASTNode::Procedure(Procedure::PenUp)),
				Token::PenDown => Ok(ASTNode::Procedure(Procedure::PenDown)),
				_ => unreachable!(),
			}
		}).then_ignore(no_arg.clone());

	let procedure_one_arg = just(Token::Forward)
		.or(just(Token::Back))
		.or(just(Token::Left))
		.or(just(Token::Right))
		.or(just(Token::Turn))
		.or(just(Token::SetHeading))
		.or(just(Token::SetX))
		.or(just(Token::SetY))
		.or(just(Token::SetPenColor))
		.then(arg.clone())
		.try_map(|(token, value), _span| {
			match token {
				Token::Forward => Ok(ASTNode::Procedure(Procedure::Forward(value))),
				Token::Back => Ok(ASTNode::Procedure(Procedure::Back(value))),
				Token::Left => Ok(ASTNode::Procedure(Procedure::Left(value))),
				Token::Right => Ok(ASTNode::Procedure(Procedure::Right(value))),
				Token::Turn => Ok(ASTNode::Procedure(Procedure::Turn(value))),
				Token::SetHeading => Ok(ASTNode::Procedure(Procedure::SetHeading(value))),
				Token::SetX => Ok(ASTNode::Procedure(Procedure::SetX(value))),
				Token::SetY => Ok(ASTNode::Procedure(Procedure::SetY(value))),
				Token::SetPenColor => Ok(ASTNode::Procedure(Procedure::SetPenColor(value))),
				_ => unreachable!(),
			}
		}).then_ignore(no_arg.clone());

	let bool = condition.clone()
		.map(|c| Expression::Bool(Box::new(c)));

	let make = just(Token::Make)
		.ignore_then(arg.clone()
			.then(arg.clone().or(bool.clone())))
		.try_map(|(name, value), span| {
			let name = match name {
				Expression::String(s) => Expression::Variable(s),
				_ => return Err(Simple::custom(span, "First argument of MAKE should be a variable")),
			};
			Ok(ASTNode::Procedure(Procedure::Make(name, value)))	
		}).then_ignore(no_arg.clone());

	let add_assign = just(Token::AddAssign)
		.ignore_then(arg.clone()
			.then(arg.clone()))
		.try_map(| (name, value), span| {
			let name = match name {
				Expression::String(s) => Expression::Variable(s),
				_ => return Err(Simple::custom(span, "First argument of ADDASSIGN should be a variable")),
			};
			Ok(ASTNode::Procedure(Procedure::AddAssign(name, value)))
		}).then_ignore(no_arg.clone());
	
	let procedure_two_args = make.or(add_assign);
	let procedure = procedure_no_args.or(procedure_one_arg).or(procedure_two_args);
	
	// Control flow parsers
	let control_flow = recursive(|control_flow| {
		let cond = condition.clone()
			.map(|c| Expression::Bool(Box::new(c)))
			.or(variable);

		let if_condition = just(Token::If).then(cond.clone());
		let while_condition = just(Token::While).then(cond.clone());

		let body = procedure.clone()
			.or(control_flow)
			.repeated()
			.at_least(1)
			.delimited_by(just(Token::LParen), just(Token::RParen));

		if_condition.or(while_condition)
			.then(body)
			.try_map(|((token, condition), body), _span| {
				let control_flow = match token {
					Token::If => ControlFlow::If { condition, block: body },
					Token::While => ControlFlow::While { condition, block: body },
					_ => unreachable!(),
				};
				Ok(ASTNode::ControlFlow(control_flow))
			})
	});

	procedure
		.or(control_flow)
		.repeated()
		.at_least(1)
}