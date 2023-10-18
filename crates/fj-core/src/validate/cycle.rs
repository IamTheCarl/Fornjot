use fj_math::{Point, Scalar};

use crate::objects::{Cycle, HalfEdge};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Cycle {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        CycleValidationError::check_edges_disconnected(self, config, errors);
        CycleValidationError::check_enough_edges(self, config, errors);
    }
}

/// [`Cycle`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum CycleValidationError {
    /// [`Cycle`]'s edges are not connected
    #[error(
        "Adjacent `Edge`s are distinct\n\
        - End position of first `Edge`: {end_of_first:?}\n\
        - Start position of second `Edge`: {start_of_second:?}\n\
        - `Edge`s: {half_edges:#?}"
    )]
    EdgesDisconnected {
        /// The end position of the first [`HalfEdge`]
        end_of_first: Point<2>,

        /// The start position of the second [`HalfEdge`]
        start_of_second: Point<2>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The edges
        half_edges: Box<(HalfEdge, HalfEdge)>,
    },

    /// [`Cycle`]'s should have at least one [`HalfEdge`]
    #[error("Expected at least one `Edge`\n")]
    NotEnoughEdges,
}

impl CycleValidationError {
    fn check_enough_edges(
        cycle: &Cycle,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        // If there are no half edges
        if cycle.half_edges().iter().next().is_none() {
            errors.push(Self::NotEnoughEdges.into());
        }
    }

    fn check_edges_disconnected(
        cycle: &Cycle,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for (first, second) in cycle.half_edges().pairs() {
            let end_of_first = {
                let [_, end] = first.boundary().inner;
                first.path().point_from_path_coords(end)
            };
            let start_of_second = second.start_position();

            let distance = (end_of_first - start_of_second).magnitude();

            if distance > config.identical_max_distance {
                errors.push(
                    Self::EdgesDisconnected {
                        end_of_first,
                        start_of_second,
                        distance,
                        half_edges: Box::new((
                            first.clone_object(),
                            second.clone_object(),
                        )),
                    }
                    .into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        assert_contains_err,
        objects::{Cycle, HalfEdge},
        operations::{BuildCycle, BuildHalfEdge, Insert, UpdateCycle},
        services::Services,
        validate::{cycle::CycleValidationError, Validate, ValidationError},
    };

    #[test]
    fn edges_connected() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid =
            Cycle::polygon([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], &mut services);

        valid.validate_and_return_first_error()?;

        let disconnected = {
            let edges = [
                HalfEdge::line_segment(
                    [[0., 0.], [1., 0.]],
                    None,
                    &mut services,
                ),
                HalfEdge::line_segment(
                    [[0., 0.], [1., 0.]],
                    None,
                    &mut services,
                ),
            ];
            let edges = edges.map(|edge| edge.insert(&mut services));

            Cycle::empty().add_half_edges(edges)
        };

        assert_contains_err!(
            disconnected,
            ValidationError::Cycle(
                CycleValidationError::EdgesDisconnected { .. }
            )
        );

        let empty = Cycle::new([]);
        assert_contains_err!(
            empty,
            ValidationError::Cycle(CycleValidationError::NotEnoughEdges)
        );
        Ok(())
    }
}
