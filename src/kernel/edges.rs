use crate::{
    kernel::geometry::{Circle, Curve, Line},
    math::{Point, Vector},
};

/// The edges of a shape
pub struct Edges(pub Vec<Edge>);

impl Edges {
    /// Construct a new instance of `Edges`
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Compute line segments to approximate the edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    pub fn approx_segments(&self, tolerance: f64) -> Vec<Segment> {
        let mut vertices = Vec::new();
        for edge in &self.0 {
            vertices.extend(edge.approx_vertices(tolerance));
        }

        // If we have multiple connected edges, the previous step will produce
        // duplicate vertices.
        vertices.dedup();

        let mut segments = Vec::new();
        for segment in vertices.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            segments.push([v0, v1].into());
        }

        segments
    }
}

/// An edge of a shape
#[derive(Debug)]
pub struct Edge {
    /// The curve that defines the edge's geometry
    ///
    /// In principle, curves could be reused for multiple edges. However, this
    /// requires a facility, here in `Edge`, to define the boundary of the edge
    /// on the curve.
    ///
    /// While such a facility doesn't exist, edges are assumed to be bounded by
    /// the points with `0` and `1` parameters on the curve. For a line, those
    /// would be the two points that define the line, for example.
    pub curve: Curve,

    /// Indicates whether the curve is reversed
    pub reverse: bool,
}

impl Edge {
    /// Create an arc
    ///
    /// So far, the name of this method is a bit ambitious, as only full circles
    /// are supported.
    pub fn arc(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle { radius }),
            reverse: false,
        }
    }

    /// Create a line segment
    pub fn line_segment(start: Point, end: Point) -> Self {
        Self {
            curve: Curve::Line(Line { a: start, b: end }),
            reverse: false,
        }
    }

    /// Reverse the edge
    pub fn reverse(self) -> Self {
        Self {
            curve: self.curve,
            reverse: !self.reverse,
        }
    }

    /// Compute vertices to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edge.
    pub fn approx_vertices(&self, tolerance: f64) -> Vec<Point> {
        let mut vertices = self.curve.approx_vertices(tolerance);

        if self.reverse {
            vertices.reverse()
        }

        vertices
    }
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