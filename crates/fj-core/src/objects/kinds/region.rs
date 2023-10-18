//! A single, continues 2d region
use fj_interop::mesh::Color;

use crate::{
    objects::{handles::Handles, Cycle},
    storage::Handle,
};

/// A single, continuous 2d region, may contain holes
///
/// Interior cycles must have the opposite winding of the exterior cycle,
/// meaning on the front side of the region, they must appear clockwise. This
/// means that all [`HalfEdge`]s that bound a `Region` have the interior of the
/// region on their left side (on the region's front side).
///
/// [`HalfEdge`]: crate::objects::HalfEdge
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Region {
    exterior: Handle<Cycle>,
    interiors: Handles<Cycle>,
    color: Option<Color>,
}

impl Region {
    /// Construct an instance of `Region`
    pub fn new(
        exterior: Handle<Cycle>,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
        color: Option<Color>,
    ) -> Self {
        Self {
            exterior,
            interiors: interiors.into_iter().collect(),
            color,
        }
    }

    /// Access the cycle that bounds the region on the outside
    pub fn exterior(&self) -> &Handle<Cycle> {
        &self.exterior
    }

    /// Access the cycles that bound the region on the inside
    ///
    /// Each of these cycles defines a hole in the region .
    pub fn interiors(&self) -> &Handles<Cycle> {
        &self.interiors
    }

    /// Access all cycles of the region (both exterior and interior)
    pub fn all_cycles(&self) -> impl Iterator<Item = &Handle<Cycle>> {
        // It would be nice to return `&Handles` here, but I don't see a way for
        // doing that here *and* in `interiors`.
        [self.exterior()].into_iter().chain(self.interiors())
    }

    /// Access the color of the region
    pub fn color(&self) -> Option<Color> {
        self.color
    }
}
