use fj_math::{Point, Scalar};

use crate::objects::{GlobalEdge, HalfEdge};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for HalfEdge {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        HalfEdgeValidationError::check_vertex_coincidence(self, config, errors);
    }
}

impl Validate for GlobalEdge {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}

/// [`HalfEdge`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum HalfEdgeValidationError {
    /// [`HalfEdge`]'s vertices are coincident
    #[error(
        "Vertices of `HalfEdge` on curve are coincident\n\
        - Position of back vertex: {back_position:?}\n\
        - Position of front vertex: {front_position:?}\n\
        - `HalfEdge`: {half_edge:#?}"
    )]
    VerticesAreCoincident {
        /// The position of the back vertex
        back_position: Point<1>,

        /// The position of the front vertex
        front_position: Point<1>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The half-edge
        half_edge: HalfEdge,
    },
}

impl HalfEdgeValidationError {
    fn check_vertex_coincidence(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let [back_position, front_position] = half_edge.boundary();
        let distance = (back_position - front_position).magnitude();

        if distance < config.distinct_min_distance {
            errors.push(
                Self::VerticesAreCoincident {
                    back_position,
                    front_position,
                    distance,
                    half_edge: half_edge.clone(),
                }
                .into(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        assert_contains_err,
        objects::HalfEdge,
        operations::BuildHalfEdge,
        services::Services,
        validate::{HalfEdgeValidationError, Validate, ValidationError},
    };

    #[test]
    fn half_edge_vertices_are_coincident() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = HalfEdge::line_segment(
            [[0., 0.], [1., 0.]],
            None,
            &mut services.objects,
        );
        let invalid = {
            let boundary = [Point::from([0.]); 2];

            HalfEdge::new(
                valid.curve(),
                boundary,
                valid.start_vertex().clone(),
                valid.global_form().clone(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::HalfEdge(
                HalfEdgeValidationError::VerticesAreCoincident { .. }
            )
        );

        Ok(())
    }
}
