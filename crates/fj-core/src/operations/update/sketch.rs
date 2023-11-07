use crate::{
    objects::{Region, Sketch},
    storage::Handle,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    #[must_use]
    fn add_region(&self, region: Handle<Region>) -> Self;

    /// Update a region of the sketch
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_region(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self;

    /// Replace a region of the sketch
    ///
    /// This is a more general version of [`UpdateSketch::update_region`] which
    /// can replace a single edge with multiple others.
    ///
    /// # Panics
    ///
    /// Uses [`Handles::replace_with_multiple`] internally, and panics for the
    /// same reasons.
    ///
    /// [`Handles::replace_with_multiple`]: crate::objects::Handles::replace_with_multiple
    #[must_use]
    fn replace_region<const N: usize>(
        &self,
        handle: &Handle<Region>,
        replace: impl FnOnce(&Handle<Region>) -> [Handle<Region>; N],
    ) -> Self;
}

impl UpdateSketch for Sketch {
    fn add_region(&self, region: Handle<Region>) -> Self {
        Sketch::new(self.regions().iter().cloned().chain([region]))
    }

    fn update_region(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
        let regions = self.regions().update(handle, update(handle));
        Sketch::new(regions)
    }

    fn replace_region<const N: usize>(
        &self,
        handle: &Handle<Region>,
        replace: impl FnOnce(&Handle<Region>) -> [Handle<Region>; N],
    ) -> Self {
        let regions = self
            .regions()
            .replace_with_multiple(handle, replace(handle))
            .expect("Region not found");
        Sketch::new(regions)
    }
}
