use std::vec;

use fj_interop::ext::SliceExt;
use fj_math::Point;

use crate::{geometry::SurfacePath, objects::Face};

use super::CurveEdgeIntersection;

/// The intersections between a curve and a [`Face`], in curve coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveFaceIntersection {
    /// The intervals where the curve and face intersect, in curve coordinates
    pub intervals: Vec<CurveFaceIntersectionInterval>,
}

impl CurveFaceIntersection {
    /// Create a new instance from the intersection intervals
    ///
    /// This method is useful for test code.
    pub fn from_intervals(
        intervals: impl IntoIterator<
            Item = impl Into<CurveFaceIntersectionInterval>,
        >,
    ) -> Self {
        let intervals = intervals.into_iter().map(Into::into).collect();
        Self { intervals }
    }

    /// Compute the intersection
    pub fn compute(path: &SurfacePath, face: &Face) -> Self {
        let edges = face
            .region()
            .all_cycles()
            .flat_map(|cycle| cycle.half_edges());

        let mut intersections = Vec::new();

        for edge in edges {
            let intersection = CurveEdgeIntersection::compute(path, edge);

            if let Some(intersection) = intersection {
                match intersection {
                    CurveEdgeIntersection::Point { point_on_curve } => {
                        intersections.push(point_on_curve);
                    }
                    CurveEdgeIntersection::Coincident { points_on_curve } => {
                        intersections.extend(points_on_curve);
                    }
                }
            }
        }

        assert!(intersections.len() % 2 == 0);

        intersections.sort();

        let intervals = intersections
            .as_slice()
            .array_chunks_ext()
            .map(|&[start, end]| CurveFaceIntersectionInterval { start, end })
            .collect();

        Self { intervals }
    }

    /// Merge this intersection list with another
    ///
    /// The merged list will contain all overlaps of the intervals from the two
    /// other lists.
    pub fn merge(&self, other: &Self) -> Self {
        let mut self_intervals = self.intervals.iter().copied();
        let mut other_interval = other.intervals.iter().copied();

        let mut next_self = self_intervals.next();
        let mut next_other = other_interval.next();

        let mut intervals = Vec::new();

        while let (Some(self_), Some(other)) = (next_self, next_other) {
            // If we're starting another loop iteration, we have another
            // interval available from both `self` and `other` each. Only if
            // that's the case, is there a chance for an overlap.

            // Build the overlap of the two next intervals, by comparing them.
            // At this point we don't know yet, if this is a valid interval.
            let overlap_start = self_.start.max(other.start);
            let overlap_end = self_.end.min(other.end);

            if overlap_start < overlap_end {
                // This is indeed a valid overlap. Add it to our list of
                // results.
                intervals.push(CurveFaceIntersectionInterval {
                    start: overlap_start,
                    end: overlap_end,
                });
            }

            // Only if the end of the overlap interval has overtaken one of the
            // input ones are we done with it. An input interval that hasn't
            // been overtaken by the overlap, could still overlap with another
            // interval.
            if self_.end <= overlap_end {
                // Current interval from `self` has been overtaken. Let's grab
                // the next one.
                next_self = self_intervals.next();
            }
            if other.end <= overlap_end {
                // Current interval from `other` has been overtaken. Let's grab
                // the next one.
                next_other = other_interval.next();
            }
        }

        Self { intervals }
    }

    /// Indicate whether the intersection list is empty
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
}

impl IntoIterator for CurveFaceIntersection {
    type Item = CurveFaceIntersectionInterval;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intervals.into_iter()
    }
}

/// An intersection between a curve and a face
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveFaceIntersectionInterval {
    /// The start of the intersection interval, in curve coordinates
    pub start: Point<1>,

    /// The end of the intersection interval, in curve coordinates
    pub end: Point<1>,
}

impl<P> From<[P; 2]> for CurveFaceIntersectionInterval
where
    P: Into<Point<1>>,
{
    fn from(interval: [P; 2]) -> Self {
        let [start, end] = interval.map(Into::into);
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::SurfacePath,
        objects::{Cycle, Face},
        operations::{
            build::{BuildCycle, BuildFace},
            update::{UpdateFace, UpdateRegion},
        },
        Instance,
    };

    use super::CurveFaceIntersection;

    #[test]
    fn compute() {
        let mut core = Instance::new();

        let (path, _) = SurfacePath::line_from_points([[-3., 0.], [-2., 0.]]);

        #[rustfmt::skip]
        let exterior_points = [
            [-2., -2.],
            [ 2., -2.],
            [ 2.,  2.],
            [-2.,  2.],
        ];
        #[rustfmt::skip]
        let interior_points = [
            [-1., -1.],
            [-1.,  1.],
            [ 1.,  1.],
            [ 1., -1.],
        ];

        let face =
            Face::unbound(core.layers.objects.surfaces.xy_plane(), &mut core)
                .update_region(
                    |region, core| {
                        region
                            .update_exterior(
                                |_, core| Cycle::polygon(exterior_points, core),
                                core,
                            )
                            .add_interiors(
                                [Cycle::polygon(interior_points, core)],
                                core,
                            )
                    },
                    &mut core,
                );

        let expected =
            CurveFaceIntersection::from_intervals([[[1.], [2.]], [[4.], [5.]]]);
        assert_eq!(CurveFaceIntersection::compute(&path, &face), expected);
    }

    #[test]
    fn merge() {
        let a = CurveFaceIntersection::from_intervals([
            [[0.], [1.]],   // 1: `a` and `b` are equal
            [[2.], [5.]],   // 2: `a` contains `b`
            [[7.], [8.]],   // 3: `b` contains `a`
            [[9.], [11.]],  // 4: overlap; `a` is left
            [[14.], [16.]], // 5: overlap; `a` is right
            [[18.], [21.]], // 6: one of `a` partially overlaps two of `b`
            [[23.], [25.]], // 7: two of `a` partially overlap one of `b`
            [[26.], [28.]], // 7
            [[31.], [35.]], // 8: partial/complete: one of `a`, two of `b`;
            [[36.], [38.]], // 9: partial/complete: two of `a`, one of `b`
            [[39.], [40.]], // 9
            [[41.], [45.]], // 10: complete/partial: one of `a`, two of `b`
            [[48.], [49.]], // 11: complete/partial: two of `a`, one of `b`
            [[50.], [52.]], // 11
            [[53.], [58.]], // 12: one of `a` overlaps two of `b` completely
            [[60.], [61.]], // 13: one of `b` overlaps two of `a` completely
            [[62.], [63.]], // 13
            [[65.], [66.]], // 14: one of `a` with no overlap in `b`
        ]);
        let b = CurveFaceIntersection::from_intervals([
            [[0.], [1.]],   // 1
            [[3.], [4.]],   // 2
            [[6.], [9.]],   // 3
            [[10.], [12.]], // 4
            [[13.], [15.]], // 5
            [[17.], [19.]], // 6
            [[20.], [22.]], // 6
            [[24.], [27.]], // 7
            [[30.], [32.]], // 8
            [[33.], [34.]], // 8
            [[37.], [41.]], // 9
            [[42.], [43.]], // 10
            [[44.], [46.]], // 10
            [[47.], [51.]], // 11
            [[54.], [55.]], // 12
            [[56.], [57.]], // 12
            [[59.], [64.]], // 13
        ]);

        let merged = a.merge(&b);

        let expected = CurveFaceIntersection::from_intervals([
            [[0.], [1.]],   // 1
            [[3.], [4.]],   // 2
            [[7.], [8.]],   // 3
            [[10.], [11.]], // 4
            [[14.], [15.]], // 5
            [[18.], [19.]], // 6
            [[20.], [21.]], // 6
            [[24.], [25.]], // 7
            [[26.], [27.]], // 7
            [[31.], [32.]], // 8
            [[33.], [34.]], // 8
            [[37.], [38.]], // 9
            [[39.], [40.]], // 9
            [[42.], [43.]], // 10
            [[44.], [45.]], // 10
            [[48.], [49.]], // 11
            [[50.], [51.]], // 11
            [[54.], [55.]], // 12
            [[56.], [57.]], // 12
            [[60.], [61.]], // 13
            [[62.], [63.]], // 13
        ]);
        assert_eq!(merged, expected);
    }
}
