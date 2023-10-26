//! Operations to update shapes

mod build;
mod insert;
mod join;
mod merge;
mod reverse;
mod split;
mod update;

pub use self::{
    build::{
        cycle::BuildCycle,
        edge::BuildHalfEdge,
        face::{BuildFace, Polygon},
        region::BuildRegion,
        shell::{BuildShell, TetrahedronShell},
        sketch::BuildSketch,
        solid::{BuildSolid, Tetrahedron},
        surface::BuildSurface,
    },
    insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
    join::cycle::JoinCycle,
    merge::Merge,
    reverse::Reverse,
    split::edge::SplitHalfEdge,
    update::{
        cycle::UpdateCycle, edge::UpdateHalfEdge, face::UpdateFace,
        region::UpdateRegion, shell::UpdateShell, sketch::UpdateSketch,
        solid::UpdateSolid,
    },
};
