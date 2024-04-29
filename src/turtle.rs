//! # Turtle
//! 
//! This module contains the `Turtle` struct, which represents the state of the turtle in the Logo language.
//! 
//! The `Turtle` struct includes fields for the image being drawn on, the current variables, the turtle's position (`x`, `y`),
//! heading, pen state, and pen color.
//! 
//! The `new` method is used to create a new `Turtle` with a given image. The turtle starts at the center of the image,
//! with a heading of 0.0, the pen up, and the pen color set to the 8th color in the `COLORS` array from the `unsvg` crate.
//! 
//! # Example
//! 
//! ```
//! use unsvg::Image;
//! use rslogo::turtle::Turtle;
//! 
//! let mut image = Image::new(100, 100);
//! let mut turtle = Turtle::new(&mut image);
//! 
//! assert_eq!(turtle.get_x(), 50.0);
//! assert_eq!(turtle.get_y(), 50.0);
//! ```
//! 
//! This example creates a new `Image` and a new `Turtle` that will draw on the image.

use std::collections::HashMap;
use unsvg::{get_end_coordinates, Color, Image, COLORS};
use crate::ast::Expression;


/// Represents the state of the turtle in the Logo language.
/// 
/// The `Turtle` struct includes fields for the image being drawn on, the current variables, the turtle's position (`x`, `y`),
/// heading, pen state, and pen color.
/// 
pub struct Turtle<'a> {
    image: &'a mut Image,
    variables: HashMap<String, Expression>,
    x: f32,
    y: f32,
    heading: f32,
    pen_down: bool,
    pen_color: Color,
}

impl<'a> Turtle<'a> {
    /// Creates a new `Turtle` with the given image.
    pub fn new(image: &'a mut Image) -> Self {
        let dimensions = image.get_dimensions();
        let (x, y) = (dimensions.0 as f32 / 2.0, dimensions.1 as f32 / 2.0);
        Self {
            image,
            variables: HashMap::new(),
            x,
            y,
            heading: 0.0,
            pen_down: false,
            pen_color: COLORS[7],
        }
    }

    /// Lifts the pen off the image. When the turtle moves, it will not draw anything.
    pub fn pen_up (&mut self) {
        self.pen_down = false;
    }

    /// Puts the pen down on the image. When the turtle moves, it will draw a line.
    pub fn pen_down (&mut self) {
        self.pen_down = true;
    }

    /// Moves the turtle forward by `expr` units. If the pen is down, it will draw a line.
    pub fn forward (&mut self, expr: f32) -> Result<(), unsvg::Error> {
        let end = get_end_coordinates(self.x, self.y, self.heading as i32, expr);
        if self.pen_down {
            self.image.draw_simple_line(self.x, self.y, self.heading as i32, expr, self.pen_color)?;
        }
        (self.x, self.y) = end;
        Ok(())
    }

    /// Moves the turtle backward by `expr` units. If the pen is down, it will draw a line.
    pub fn back (&mut self, expr: f32) -> Result<(), unsvg::Error> {
        self.forward(-expr)
    }

    /// Moves the turtle to the left by `expr` units. If the pen is down, it will draw a line.
    pub fn left (&mut self, expr: f32) -> Result<(), unsvg::Error> {
        let heading = (self.heading - 90.0) as i32;
        let end = get_end_coordinates(self.x, self.y, heading, expr);
        if self.pen_down {
            self.image.draw_simple_line(self.x, self.y, heading, expr, self.pen_color)?;
        } 
        (self.x, self.y) = end;
        Ok(())
    }


    /// Moves the turtle to the right by `expr` units. If the pen is down, it will draw a line.
    pub fn right (&mut self, expr: f32) -> Result<(), unsvg::Error> {
        self.left(-expr)
    }


    /// Turns the turtle by `expr` degrees.
    pub fn turn (&mut self, expr: f32) {
        self.heading += expr;
    }


    /// Sets the heading of the turtle to `expr` degrees.
    pub fn set_heading (&mut self, expr: f32) {
        self.heading = expr;
    }

    /// Sets the pen color to the color at index `expr` in the `COLORS` array.
    pub fn set_pen_color (&mut self, expr: f32) {
        self.pen_color = COLORS[expr as usize];
    }

    /// Sets the x-coordinate of the turtle to `expr`.
    pub fn set_x (&mut self, expr: f32) {
        self.x = expr;
    }

    /// Sets the y-coordinate of the turtle to `expr`.
    pub fn set_y (&mut self, expr: f32) {
        self.y = expr;
    }

    /// Sets the x and y coordinates of the turtle to `(x, y)`.
    pub fn add_variable (&mut self, name: &str, value: Expression) {
        self.variables.insert(name.to_string(), value);
    }

    /// Gets the value of the variable with the given name.
    pub fn get_variable (&self, name: &String) -> &Expression {
        self.variables.get(name).unwrap_or_else(|| panic!("{} Variable not found", name)) 
    }
    
    /// Gets the x-coordinate of the turtle.
	pub fn get_x(&self) -> f32 {
		self.x
	}

    /// Gets the y-coordinate of the turtle.
	pub fn get_y(&self) -> f32 {
		self.y
	}
	
    /// Gets the pen color of the turtle.
	pub fn get_pen_color(&self) -> f32 {
		COLORS.iter().position(|&x| x == self.pen_color).unwrap() as f32
	}

    /// Gets the heading of the turtle.
	pub fn get_heading(&self) -> f32 {
		self.heading
	}
}