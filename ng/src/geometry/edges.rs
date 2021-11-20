use std::f64::consts::PI;

use crate::math::{Point, Vector};

/// The edges of a shape
pub struct Edges(pub Vec<Edge>);

impl Edges {
    /// Construct a new instance of `Edges`
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

/// An edge of a shape
pub struct Edge {
    /// The path that defines the edge
    pub path: Path,

    /// Indicates whether the path is reversed
    pub reverse: bool,
}

impl Edge {
    /// Create an arc
    pub fn arc(radius: f64) -> Self {
        Self {
            path: Path::Arc { radius },
            reverse: false,
        }
    }

    /// Create a line segment
    pub fn line_segment(start: Point, end: Point) -> Self {
        Self {
            path: Path::LineSegment { start, end },
            reverse: false,
        }
    }

    /// Reverse the edge's path
    pub fn reverse(self) -> Self {
        Self {
            path: self.path,
            reverse: !self.reverse,
        }
    }

    /// Compute vertices to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edge.
    pub fn vertices(&self, tolerance: f64) -> Vec<Point> {
        let mut vertices = match &self.path {
            Path::Arc { radius } => {
                let angle_to_point = |angle: f64| {
                    let (sin, cos) = angle.sin_cos();

                    let x = cos * radius;
                    let y = sin * radius;

                    [x, y, 0.].into()
                };

                // To approximate the circle, we use a regular polygon for which
                // the circle is the circumscribed circle. The `tolerance`
                // parameter is the maximum allowed distance between the polygon
                // and the circle. This is the same as the difference between
                // the circumscribed circle and the incircle.
                //
                // Let's figure which regular polygon we need to use, by just
                // trying out some of them until we find one whose maximum error
                // is less than or equal to the tolerance.
                let mut n = 3;
                loop {
                    let incircle_radius = radius * (PI / n as f64).cos();
                    let maximum_error = radius - incircle_radius;

                    if maximum_error <= tolerance {
                        break;
                    }

                    n += 1;
                }

                let mut vertices = Vec::new();

                let first_vertex = angle_to_point(0.0);
                vertices.push(first_vertex);

                for i in 1..n {
                    let angle = 2. * PI / n as f64 * i as f64;
                    vertices.push(angle_to_point(angle));
                }

                // Connect the circle's to itself.
                vertices.push(first_vertex);

                vertices
            }
            Path::LineSegment { start, end } => vec![*start, *end],
        };

        if self.reverse {
            vertices.reverse()
        }

        vertices
    }
}

/// A path
pub enum Path {
    /// The edge is an arc
    ///
    /// The naming here is a bit ambitious, as actual arcs aren't supported yet,
    /// only full circles.
    Arc {
        /// The radius of the arc
        radius: f64,
    },

    /// The edge is a line segment
    LineSegment {
        /// The start of the line segment
        start: Point,

        /// The end of the line segment
        end: Point,
    },
}

/// A line segment
#[derive(Debug)]
pub struct Segment(pub [Point; 2]);

impl Segment {
    /// Translate the segment
    ///
    /// Translate all segment vertices by the given vector.
    pub fn translate(self, vector: Vector) -> Self {
        let vertices = self.0.map(|vertex| vertex + vector);
        Self(vertices)
    }
}

impl From<[Point; 2]> for Segment {
    fn from(vertices: [Point; 2]) -> Self {
        Self(vertices)
    }
}