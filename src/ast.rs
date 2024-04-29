//! # AST
//! 
//! This module contains the AST (Abstract Syntax Tree) for the language. 
//! The AST is used to represent the parsed code in a way that is easier to execute. 
//! The AST is made up of nodes, which can be either a procedure or a control flow. 
//! The AST is executed by calling the `execute` method on the root node, which will recursively execute all the nodes in the tree. 
//! The AST is created by the parser, which converts the tokens into nodes in the tree.
//! 
//! The AST is made up of the following nodes:
//! 
//! - `ASTNode` - The root node of the tree, which can be either a `Procedure` or a `ControlFlow`.
//! - `Procedure` - Represents a single procedure in the language, such as `FORWARD 10` or `MAKE "x 10`.
//! - `ControlFlow` - Represents a control flow structure in the language, such as an `IF` statement or a `WHILE` loop.
//! - `Condition` - Represents a boolean condition in the language, such as `EQ 1 2` or `AND EQ 1 1 EQ 2 2`.
//! - `Expression` - Represents an expression in the language, such as a float, a variable, or a math operation.
//! - `Math` - Represents a math operation in the language, such as `ADD 1 2` or `MUL 3 4`.
//! - `Query` - Represents a query in the language, such as `XCOR` or `YCOR`.
//! 
//! The AST is used by the `Turtle` module to execute the parsed code and draw the resulting image.


use crate::{turtle::Turtle, uncertain_bool::is_option_eq};
use unsvg;

/// The root node of the AST, which can be either a `Procedure` or a `ControlFlow`.
/// 
/// The `execute` method is used to execute the AST.
/// It takes a mutable reference to a `Turtle` and recursively executes all the nodes in the tree.
/// 
/// # Methods
/// 
/// * `execute`: Executes the AST using the given `Turtle` state.
/// 
/// # Example
/// 
/// ```
/// use rslogo::ast::{ASTNode, Procedure, Expression};
/// use rslogo::turtle::Turtle;
/// use unsvg::Image;
/// 
/// let mut image = Image::new(100, 100);
/// let mut turtle = Turtle::new(&mut image);
/// 
/// let ast = ASTNode::Procedure(Procedure::Forward(Expression::Float(10.0)));
/// ast.execute(&mut turtle);
/// ast.execute(&mut turtle);
/// 
/// assert_eq!(turtle.get_y(), 30.0);
/// 
/// ```
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Represents a single procedure in the language, such as `FORWARD 10` or `MAKE "x 10`.
    Procedure(Procedure),

    /// Represents a control flow structure in the language, such as an `IF` statement or a `WHILE` loop.
    ControlFlow(ControlFlow),
}
impl ASTNode {
    pub fn execute(&self, turtle: &mut Turtle) -> Result<(), unsvg::Error>{
        if let ASTNode::Procedure(proceedure) = self {
            match proceedure {
                // Only the pen up and pen down procedures do not require an expression
                Procedure::PenUp => turtle.pen_up(),
                Procedure::PenDown => turtle.pen_down(),

                Procedure::Forward(s) => turtle.forward(s.to_float(turtle).expect("Invalid value"))?,
                Procedure::Back(s) => turtle.back(s.to_float(turtle).expect("Invalid value"))?,
                Procedure::Left(s) => turtle.left(s.to_float(turtle).expect("Invalid value"))?,
                Procedure::Right(s) => turtle.right(s.to_float(turtle).expect("Invalid value"))?,
                Procedure::Turn(s) => turtle.turn(s.to_float(turtle).expect("Invalid value")),
                Procedure::SetHeading(s) => turtle.set_heading(s.to_float(turtle).expect("Invalid value")),
                Procedure::SetPenColor(s) => turtle.set_pen_color(s.to_float(turtle).expect("Invalid value")),
                Procedure::SetX(s) => turtle.set_x(s.to_float(turtle).expect("Invalid value")),
                Procedure::SetY(s) => turtle.set_y(s.to_float(turtle).expect("Invalid value")),

                Procedure::Make(s, s2) => {
                    let name = match s {
                        Expression::Variable(var) => var,
                        _ => panic!("First argument of MAKE should be a variable"),
                    };
                    let val = match s2 {
                        Expression::Math(_) => s2.eval_math(turtle),
                        _ => s2.clone(),
                    };
                    turtle.add_variable(name, val);
                },
                Procedure::AddAssign(s, s2) => {
                    let name = match s {
                        Expression::Variable(var) => var,
                        _ => panic!("First argument of ADDASSIGN should be a variable"),
                    };
                    let cur = turtle.get_variable(name).to_float(turtle).expect("Variable not a float");
                    let add = s2.to_float(turtle).expect("Second argument can't be turned into a float");
                    turtle.add_variable(name, Expression::Float(cur + add));
                },
            }
        };
        if let ASTNode::ControlFlow(flow) = self {
            match flow {
                ControlFlow::If { condition, block } => {
                    let condition = condition.to_bool(turtle).expect("Control flow condition must be able to evaluate into a boolean");
                    if condition {
                        for instruction in block {
                            let _ = instruction.execute(turtle);
                        }
                    }
                },
                ControlFlow::While { condition, block } => {
                    let mut cond = condition.to_bool(turtle).expect("Control flow condition must be able to evaluate into a boolean");
                    while cond {
                        for instruction in block {
                            let _ = instruction.execute(turtle);
                        }
                        cond = condition.to_bool(turtle).expect("Control flow condition must be able to evaluate into a boolean");
                    }
                },
            }
        }
        Ok(())
    }
}


/// Represents a control flow structure in the language, such as an `IF` statement or a `WHILE` loop.
#[derive(Debug, Clone)]
pub enum ControlFlow {
    /// Represents an `IF` statement, which executes a block of code if a condition is true.
    If {
        condition: Expression,
        block: Vec<ASTNode>,
    },

    /// Represents a `WHILE` loop, which executes a block of code while a condition is true.
    While {
        condition: Expression,
        block: Vec<ASTNode>,
    },
}

/// `Condition` is an enum representing the conditional expressions in a programming language.
/// 
/// It currently supports six variants: `Equal`, `NotEqual`, `LessThan`, `GreaterThan`, `And`, and `Or`.
///
/// # Methods
///
/// * `eval`: Evaluates the `Condition` based on the given `Turtle` state and returns a boolean result.
///
/// # Example
///
/// ```
/// use unsvg::Image;
/// use rslogo::turtle::Turtle;
/// use rslogo::ast::{Condition, Expression};
/// 
/// let mut image = Image::new(100, 100);
/// let turtle = Turtle::new(&mut image);
/// 
/// let equal_condition = Condition::Equal(Expression::Float(1.0), Expression::Float(1.0));
/// let equal_expression = Expression::Bool(Box::new(equal_condition));
/// let equal_val = equal_expression.to_bool(&turtle).unwrap();
/// assert_eq!(equal_val, true);
/// ```
#[derive(Debug, Clone)]
pub enum Condition {
    /// Represents an equality comparison between two `Expression`s.
	Equal(Expression, Expression),

    /// Represents a non-equality comparison between two `Expression`s.
	NotEqual(Expression, Expression),

    /// Represents a less-than comparison between two `Expression`s.
	LessThan(Expression, Expression),

    /// Represents a greater-than comparison between two `Expression`s.
	GreaterThan(Expression, Expression),

    /// Represents a logical AND operation between two `Condition`s.
	And(Box<Condition>, Box<Condition>),

    /// Represents a logical OR operation between two `Condition`s.
	Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    fn eval(&self, turtle: &Turtle) -> bool {
        match self {
            Condition::Equal(expr1, expr2) => {
                let float = is_option_eq(expr1.to_float(turtle), expr2.to_float(turtle));
                let bool = is_option_eq(expr1.to_bool(turtle), expr2.to_bool(turtle));
                let string = is_option_eq(expr1.to_string(turtle), expr2.to_string(turtle));
                float.is_true() || bool.is_true() || string.is_true()
            }
            Condition::NotEqual(expr1, expr2) => {
                let float = is_option_eq(expr1.to_float(turtle), expr2.to_float(turtle));
                let bool = is_option_eq(expr1.to_bool(turtle), expr2.to_bool(turtle));
                let string = is_option_eq(expr1.to_string(turtle), expr2.to_string(turtle));
                float.is_false() && bool.is_false() && string.is_false()
            }
            Condition::LessThan(expr1, expr2) => {
                let val1 = expr1.to_float(turtle).expect("Can only compare floats");
                let val2 = expr2.to_float(turtle).expect("Can only compare floats");
                val1 < val2
            }
            Condition::GreaterThan(expr1, expr2) => {
                let val1 = expr1.to_float(turtle).expect("Can only compare floats");
                let val2 = expr2.to_float(turtle).expect("Can only compare floats");
                val1 > val2
            }
            Condition::And(cond1, cond2) => {
                let val1 = cond1.eval(turtle);
                let val2 = cond2.eval(turtle);
                val1 && val2
            }
            Condition::Or(cond1, cond2) => {
                let val1 = cond1.eval(turtle);
                let val2 = cond2.eval(turtle);
                val1 || val2
            }
        }
    }
}

/// Represents a single procedure in the language, such as `FORWARD 10` or `MAKE "x 10`.
#[derive(Debug, Clone)]
pub enum Procedure {
    /// Lifts the pen up, so the turtle does not draw.
    PenUp,

    /// Puts the pen down, so the turtle draws.
    PenDown,

    /// Moves the turtle forward by a given distance.
    Forward(Expression),

    /// Moves the turtle backward by a given distance.
    Back(Expression),

    /// Turns the turtle left by a given angle.
    Left(Expression),

    /// Turns the turtle right by a given angle.
    Right(Expression),

    /// Sets the pen color to a given value.
    SetPenColor(Expression),

    /// Turns the turtle to a given angle.
    Turn(Expression),

    /// Sets the heading of the turtle to a given angle.
    SetHeading(Expression),

    /// Sets the x-coordinate of the turtle to a given value.
    SetX(Expression),

    /// Sets the y-coordinate of the turtle to a given value.
    SetY(Expression),

    /// Creates a new variable with a given name and value.
    Make(Expression, Expression),

    /// Adds a value to an existing variable.
    AddAssign(Expression, Expression),
}


/// Represents an expression in the language, such as a float, a variable, or a math operation.
/// 
/// # Methods
/// 
/// - `to_float` - Converts the expression to a float, if possible.
/// - `to_string` - Converts the expression to a string, if possible.
/// - `to_bool` - Converts the expression to a boolean, if possible.
/// - `eval_math` - Evaluates the math operation in the expression and returns the result.
/// 
/// 
#[derive(Debug, Clone)]
pub enum Expression {
    /// Represents a floating point number.
    Float(f32),

    /// Represents a query to the turtle, such as `XCOR` or `YCOR`.
    Query(Query),

    /// Represents a variable name.
    Variable(String),

    /// Represents a string.
	String(String),

    /// Represents a math operation, such as `+ 1 2` or `* 3 4`.
    Math(Box<Math>),

    /// Represents a boolean condition.
	Bool(Box<Condition>),
}

impl Expression {
    pub fn to_float(&self, turtle: &Turtle) -> Option<f32> {
        match self {
            Expression::Float(val) => Some(*val),
            Expression::Variable(var) => Some(turtle.get_variable(var).to_float(turtle)?),
            Expression::Math(_) => Some(self.eval_math(turtle).to_float(turtle)?),
            Expression::Query(query) => {
                let float = match query {
                    Query::XCOR => turtle.get_x(),
                    Query::YCOR => turtle.get_y(),
                    Query::COLOR => turtle.get_pen_color(),
                    Query::HEADING => turtle.get_heading(),
                };
                Some(float)
            },
            _ => None,
        }
    }

    pub fn to_string(&self, turtle: &Turtle) -> Option<String> {
        match self {
            Expression::String(val) => Some(val.clone()),
            Expression::Variable(var) => Some(turtle.get_variable(var).to_string(turtle)?),
            _ => None,
        }
    }

    pub fn to_bool(&self, turtle: &Turtle) -> Option<bool> {
        match self {
            Expression::Bool(val) => Some(val.eval(turtle)),
            Expression::Variable(var) => turtle.get_variable(var).to_bool(turtle),
            _ => None,
        }
    }

    pub fn eval_math(&self, turtle: &Turtle) -> Expression {
        match self {
            Expression::Math(math) => {
                match math.as_ref() {
                    Math::Add(expr1, expr2) => {
                        let val1 = expr1.eval_math(turtle);
                        let val2 = expr2.eval_math(turtle);
                        val1 + val2
                    },
                    Math::Sub(expr1, expr2) => {
                        let val1 = expr1.eval_math(turtle);
                        let val2 = expr2.eval_math(turtle);
                        val1 - val2
                    },
                    Math::Mul(expr1, expr2) => {
                        let val1 = expr1.eval_math(turtle);
                        let val2 = expr2.eval_math(turtle);
                        val1 * val2
                    }
                    Math::Div(expr1, expr2) => {
                        let val1 = expr1.eval_math(turtle);
                        let val2 = expr2.eval_math(turtle);
                        val1 / val2
                    }
                }
            },
            _ => Expression::Float(self.to_float(turtle).expect("Cannot perform math on this type")),
        }
    }
}

/// Operator overloading for math operations on `Expression`.
/// Note: Only supports math operations on `Expression::Float`.
impl std::ops::Add for Expression {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Expression::Float(val1), Expression::Float(val2)) => {Expression::Float(val1 + val2)},
            _ => panic!("Can only add expressions that are floats"),
        }
    }
}

impl std::ops::Sub for Expression {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Expression::Float(val1), Expression::Float(val2)) => Expression::Float(val1 - val2),
            _ => panic!("Can only subtract expressions that are floats"),
        }
    }
}

impl std::ops::Mul for Expression {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Expression::Float(val1), Expression::Float(val2)) => Expression::Float(val1 * val2),
            _ => panic!("Can only multiply expressions that are floats"),
        }
    }
}

impl std::ops::Div for Expression {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Expression::Float(val1), Expression::Float(val2)) => {
                if val2 == 0.0 {
                    panic!("Division by zero")
                }
                Expression::Float(val1 / val2)
            },
            _ => panic!("Can only divide expressions that are floats"),
        }
    }
}

/// Represents a math operation in the language, such as `+ 1 2` or `* 3 4`.
#[derive(Debug, Clone)]
pub enum Math {
    /// Adds two expressions together.
	Add(Expression, Expression),

    /// Subtracts one expression from another.
	Sub(Expression, Expression),

    /// Multiplies two expressions together.
	Mul(Expression, Expression),

    /// Divides one expression by another.
	Div(Expression, Expression),
}

/// Represents a query in the language, such as `XCOR` or `YCOR`.
#[derive(Debug, Clone)]
pub enum Query {

    /// Returns the x-coordinate of the turtle.
	XCOR,
    /// Returns the y-coordinate of the turtle.
	YCOR,
    /// Returns the heading of the turtle.
	HEADING,
    /// Returns the pen color of the turtle.
	COLOR,
}
