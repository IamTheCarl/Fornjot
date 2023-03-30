use std::ops::Deref;

use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    objects::{Cycle, Face, HalfEdge, Objects, Sketch},
    operations::{BuildCycle, BuildHalfEdge, Insert, UpdateCycle},
    services::Service,
};
use fj_math::{Aabb, Point};
use itertools::Itertools;

use super::Shape;

impl Shape for fj::Sketch {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        objects: &mut Service<Objects>,
        _: &mut DebugInfo,
    ) -> Self::Brep {
        let surface = objects.surfaces.xy_plane();

        let face = match self.chain() {
            fj::Chain::Circle(circle) => {
                let half_edge =
                    HalfEdge::circle(circle.radius(), objects).insert(objects);
                let exterior = Cycle::new([half_edge]).insert(objects);

                Face::new(
                    surface,
                    exterior,
                    Vec::new(),
                    Some(Color(self.color())),
                )
            }
            fj::Chain::PolyChain(poly_chain) => {
                let segments = poly_chain.to_segments();
                assert!(
                    !segments.is_empty(),
                    "Attempted to compute a Brep from an empty sketch"
                );

                let exterior = {
                    let mut cycle = Cycle::empty();

                    let segments = poly_chain
                        .to_segments()
                        .into_iter()
                        .map(|fj::SketchSegment { endpoint, route }| {
                            let endpoint = Point::from(endpoint);
                            (endpoint, route)
                        })
                        .circular_tuple_windows();

                    for ((start, route), (end, _)) in segments {
                        let half_edge = match route {
                            fj::SketchSegmentRoute::Direct => {
                                HalfEdge::line_segment(
                                    [start, end],
                                    None,
                                    objects,
                                )
                            }
                            fj::SketchSegmentRoute::Arc { angle } => {
                                HalfEdge::arc(start, end, angle.rad(), objects)
                            }
                        };
                        let half_edge = half_edge.insert(objects);

                        cycle = cycle.add_half_edges([half_edge]);
                    }

                    cycle.insert(objects)
                };

                Face::new(
                    surface,
                    exterior,
                    Vec::new(),
                    Some(Color(self.color())),
                )
            }
        };

        let sketch = Sketch::new(vec![face.insert(objects)]).insert(objects);
        sketch.deref().clone()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        match self.chain() {
            fj::Chain::Circle(circle) => Aabb {
                min: Point::from([-circle.radius(), -circle.radius(), 0.0]),
                max: Point::from([circle.radius(), circle.radius(), 0.0]),
            },
            fj::Chain::PolyChain(poly_chain) => {
                let segments = poly_chain.to_segments();
                assert!(
                    !segments.is_empty(),
                    "Attempted to compute a bounding box from an empty sketch"
                );

                let mut points = vec![];

                let mut start_point = segments[segments.len() - 1].endpoint;
                segments.iter().for_each(|segment| {
                    match segment.route {
                        fj::SketchSegmentRoute::Direct => (),
                        fj::SketchSegmentRoute::Arc { angle } => {
                            use std::f64::consts::PI;
                            let arc = fj_math::Arc::from_endpoints_and_angle(
                                start_point,
                                segment.endpoint,
                                fj_math::Scalar::from_f64(angle.rad()),
                            );
                            for circle_min_max_angle in
                                [0., PI / 2., PI, 3. * PI / 2.]
                            {
                                let mm_angle = fj_math::Scalar::from_f64(
                                    circle_min_max_angle,
                                );
                                if arc.start_angle < mm_angle
                                    && mm_angle < arc.end_angle
                                {
                                    points.push(
                                        arc.center
                                            + [
                                                arc.radius
                                                    * circle_min_max_angle
                                                        .cos(),
                                                arc.radius
                                                    * circle_min_max_angle
                                                        .sin(),
                                            ],
                                    );
                                }
                            }
                        }
                    }
                    points.push(Point::from(segment.endpoint));
                    start_point = segment.endpoint;
                });

                Aabb::<3>::from_points(points.into_iter().map(Point::to_xyz))
            }
        }
    }
}
