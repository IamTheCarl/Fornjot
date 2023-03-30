//! Operations to update shapes

mod build;
mod insert;
mod update;

pub use self::{
    build::{
        BuildCycle, BuildFace, BuildHalfEdge, BuildShell, BuildSurface,
        Tetrahedron, Triangle,
    },
    insert::Insert,
    update::{UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateShell},
};
