use crate::{
    objects::{AboutToBeStored, AnyObject, Objects},
    validate::{Validation, ValidationConfig, ValidationErrors},
};

use super::{objects::ObjectsCommand, Layer};

/// # Loosely coupled layers, that together define shapes
///
/// Shapes are not a monolithic thing in Fornjot, but instead are defined by
/// several, loosely coupled layers. These layers are owned by this struct.
///
/// ## Implementation Note
///
/// It is totally conceivable that one day, this system of layers is extensible
/// and more layers can be defined by third-party code. The foundation for that,
/// the loose coupling and inter-layer communication via events, is already
/// there, conceptually.
///
/// For now, there is no need for this, and all layers are just hardcoded here.
/// That can be changed, once necessary.
#[derive(Default)]
pub struct Layers {
    /// The objects layers
    ///
    /// Manages the stores of topological and geometric objects that make up
    /// shapes.
    pub objects: Layer<Objects>,

    /// The validation layer
    ///
    /// Monitors objects and validates them, as they are inserted.
    pub validation: Layer<Validation>,
}

impl Layers {
    /// Construct an instance of `Layers`
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an instance of `Layers`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let objects = Layer::default();
        let validation = Layer::new(Validation::with_validation_config(config));

        Self {
            objects,
            validation,
        }
    }

    /// Insert an object into the stores
    pub fn insert_object(&mut self, object: AnyObject<AboutToBeStored>) {
        let mut objects_events = Vec::new();
        self.objects.process(
            ObjectsCommand::InsertObject { object },
            &mut objects_events,
        );

        for objects_event in objects_events {
            self.validation.on_objects_event(objects_event);
        }
    }

    /// Drop `Layers`; return any unhandled validation error
    pub fn drop_and_validate(self) -> Result<(), ValidationErrors> {
        let errors = self.validation.into_state().into_errors();

        if errors.0.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
