use crate::{
    actor, archetype::Archetype, resource::Resource, type_info::TypeInfo, Actor, Batch, Component,
};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Scene {
    actor_factory: actor::Factory,
    archetypes: Vec<Archetype>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Scene {
    const DEFAULT_ARCHETYPE_CAPACITY: usize = 16_usize;
}

impl Scene {
    #[must_use]
    pub fn component<C: Component>(&self, actor: Actor) -> &C {
        self.get_component(actor).unwrap()
    }

    #[must_use]
    pub fn component_mut<C: Component>(&mut self, actor: Actor) -> &mut C {
        self.get_component_mut(actor).unwrap()
    }

    #[allow(clippy::single_match_else)]
    pub fn create_actor(&mut self, batch: impl Batch) -> Actor {
        let actor = self.actor_factory.create();
        let types = batch.types();
        let archetype = match self.get_archetype_mut_with_types(&types) {
            Some(archetype) => archetype,
            None => {
                self.archetypes
                    .push(Archetype::new(Scene::DEFAULT_ARCHETYPE_CAPACITY, &types));
                self.archetypes.last_mut().unwrap()
            }
        };

        archetype.create_actor(actor, batch, &types);
        actor
    }

    pub fn destroy_actor(&mut self, actor: Actor) {
        if let Some(archetype) = self.get_archetype_mut_of(actor) {
            archetype.destroy_actor(actor);
        }
    }

    #[must_use]
    fn get_archetype_of(&self, actor: Actor) -> Option<&Archetype> {
        self.archetypes
            .iter()
            .find(|archetype| archetype.has_actor(actor))
    }

    #[must_use]
    fn get_archetype_mut_of(&mut self, actor: Actor) -> Option<&mut Archetype> {
        self.archetypes
            .iter_mut()
            .find(|archetype| archetype.has_actor(actor))
    }

    #[must_use]
    fn get_archetype_mut_with_types(&mut self, types: &[TypeInfo]) -> Option<&mut Archetype> {
        self.archetypes
            .iter_mut()
            .find(|archetype| archetype.has_types(types))
    }

    #[must_use]
    pub fn get_component<C: Component>(&self, actor: Actor) -> Option<&C> {
        self.get_archetype_of(actor)?.get_component(actor)
    }

    #[must_use]
    pub fn get_component_mut<C: Component>(&mut self, actor: Actor) -> Option<&mut C> {
        self.get_archetype_mut_of(actor)?.get_component_mut(actor)
    }

    #[must_use]
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref::<R>()
    }

    #[must_use]
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())?
            .downcast_mut::<R>()
    }

    pub fn insert_resource<R: Resource>(&mut self, res: R) -> Option<Box<R>> {
        self.resources
            .insert(res.type_id(), Box::new(res))?
            .downcast::<R>()
            .ok()
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remove_resource<R: Resource>(&mut self) -> Option<Box<R>> {
        self.resources
            .remove(&TypeId::of::<R>())?
            .downcast::<R>()
            .ok()
    }

    #[must_use]
    pub fn resource<R: Resource>(&self) -> &R {
        self.get_resource().unwrap()
    }

    #[must_use]
    pub fn resource_mut<R: Resource>(&mut self) -> &mut R {
        self.get_resource_mut().unwrap()
    }
}
