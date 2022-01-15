mod circle;
mod line;

pub use self::{circle::Circle, line::Line};

use crate::math::Point;

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
///
/// This distinction is not observed here, but moving things into that direction
/// is the intention.
#[derive(Clone, Debug)]
pub enum Curve {
    /// A circle
    Circle(Circle),

    /// A line
    Line(Line),
}

impl Curve {
    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        match self {
            Curve::Circle(circle) => circle.approx_vertices(tolerance, out),
            Curve::Line(Line { origin: a, b }) => out.extend([*a, *b]),
        }
    }
}