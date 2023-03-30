use crate::{
    objects::{GlobalEdge, HalfEdge, Vertex},
    storage::Handle,
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Update the start vertex of the half-edge
    fn update_start_vertex(&self, start_vertex: Handle<Vertex>) -> HalfEdge;

    /// Update the global form of the half-edge
    fn update_global_form(&self, global_form: Handle<GlobalEdge>) -> HalfEdge;
}

impl UpdateHalfEdge for HalfEdge {
    fn update_start_vertex(&self, start_vertex: Handle<Vertex>) -> HalfEdge {
        HalfEdge::new(
            self.curve(),
            self.boundary(),
            start_vertex,
            self.global_form().clone(),
        )
    }

    fn update_global_form(&self, global_form: Handle<GlobalEdge>) -> HalfEdge {
        HalfEdge::new(
            self.curve(),
            self.boundary(),
            self.start_vertex().clone(),
            global_form,
        )
    }
}
