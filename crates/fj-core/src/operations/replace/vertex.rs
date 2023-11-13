use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid, Vertex},
    operations::{insert::Insert, update::UpdateHalfEdge},
    services::Services,
    storage::Handle,
};

use super::ReplaceOutput;

/// Replace a [`Vertex`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceVertex: Sized {
    /// The bare object type that this trait is implemented for
    type BareObject;

    /// Replace the vertex
    #[must_use]
    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject>;
}

impl ReplaceVertex for Handle<HalfEdge> {
    type BareObject = HalfEdge;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        if original.id() == self.start_vertex().id() {
            ReplaceOutput::Updated(
                self.update_start_vertex(|_| replacement).insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Cycle> {
    type BareObject = Cycle;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for half_edge in self.half_edges() {
            let half_edge = half_edge.clone().replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= half_edge.was_updated();
            half_edges.push(half_edge.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Cycle::new(half_edges).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Region> {
    type BareObject = Region;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().clone().replace_vertex(
            original,
            replacement.clone(),
            services,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle = cycle.clone().replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= cycle.was_updated();
            interiors.push(cycle.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(
                Region::new(
                    exterior.into_inner(services),
                    interiors,
                    self.color(),
                )
                .insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Sketch> {
    type BareObject = Sketch;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region = region.clone().replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= region.was_updated();
            regions.push(region.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Sketch::new(regions).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Face> {
    type BareObject = Face;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let region = self.region().clone().replace_vertex(
            original,
            replacement,
            services,
        );

        if region.was_updated() {
            ReplaceOutput::Updated(
                Face::new(self.surface().clone(), region.into_inner(services))
                    .insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Shell> {
    type BareObject = Shell;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face = face.clone().replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= face.was_updated();
            faces.push(face.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Shell::new(faces).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceVertex for Handle<Solid> {
    type BareObject = Solid;

    fn replace_vertex(
        self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell = shell.clone().replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= shell.was_updated();
            shells.push(shell.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Solid::new(shells).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}
