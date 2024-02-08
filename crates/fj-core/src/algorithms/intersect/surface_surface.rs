use fj_math::{Line, Plane, Point, Scalar};

use crate::{
    geometry::{GlobalPath, SurfacePath},
    objects::Surface,
    storage::Handle,
};

/// The intersection between two surfaces
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceSurfaceIntersection {
    /// The intersection curves
    pub intersection_curves: [SurfacePath; 2],
}

impl SurfaceSurfaceIntersection {
    /// Compute the intersection between two surfaces
    pub fn compute(surfaces: [Handle<Surface>; 2]) -> Option<Self> {
        // Algorithm from Real-Time Collision Detection by Christer Ericson. See
        // section 5.4.4, Intersection of Two Planes.
        //
        // Adaptations were made to get the intersection curves in local
        // coordinates for each surface.

        let planes = surfaces.map(|surface| plane_from_surface(&surface));

        let [(a_distance, a_normal), (b_distance, b_normal)] =
            planes.map(|plane| plane.constant_normal_form());

        let direction = a_normal.cross(&b_normal);

        let denom = direction.dot(&direction);
        if denom == Scalar::ZERO {
            // Comparing `denom` against zero looks fishy. It's probably better
            // to compare it against an epsilon value, but I don't know how
            // large that epsilon should be.
            //
            // I'll just leave it like that, until we had the opportunity to
            // collect some experience with this code.
            // - @hannobraun
            return None;
        }

        let origin = (b_normal * a_distance - a_normal * b_distance)
            .cross(&direction)
            / denom;
        let origin = Point { coords: origin };

        let line = Line::from_origin_and_direction(origin, direction);

        let curves =
            planes.map(|plane| SurfacePath::Line(plane.project_line(&line)));

        Some(Self {
            intersection_curves: curves,
        })
    }
}

fn plane_from_surface(surface: &Surface) -> Plane {
    let (line, path) = {
        let line = match surface.geometry().u {
            GlobalPath::Line(line) => line,
            _ => todo!("Only plane-plane intersection is currently supported."),
        };

        (line, surface.geometry().v)
    };

    Plane::from_parametric(line.origin(), line.direction(), path)
}

#[cfg(test)]
mod tests {
    use fj_math::Transform;
    use pretty_assertions::assert_eq;

    use crate::{
        geometry::SurfacePath, operations::transform::TransformObject, Instance,
    };

    use super::SurfaceSurfaceIntersection;

    #[test]
    fn plane_plane() {
        let mut core = Instance::new();

        let xy = core.services.objects.surfaces.xy_plane();
        let xz = core.services.objects.surfaces.xz_plane();

        // Coincident and parallel planes don't have an intersection curve.
        assert_eq!(
            SurfaceSurfaceIntersection::compute([
                xy.clone(),
                xy.clone().transform(
                    &Transform::translation([0., 0., 1.],),
                    &mut core
                )
            ],),
            None,
        );

        let expected_xy = SurfacePath::u_axis();
        let expected_xz = SurfacePath::u_axis();

        assert_eq!(
            SurfaceSurfaceIntersection::compute([xy, xz],),
            Some(SurfaceSurfaceIntersection {
                intersection_curves: [expected_xy, expected_xz],
            })
        );
    }
}
