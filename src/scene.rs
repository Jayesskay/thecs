use crate::{Actor, ActorFactory, Archetype, Component, ComponentSource, Resource, TypeInfo};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Scene {
    actor_factory: ActorFactory,
    archetypes: Vec<Archetype>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Scene {
    const DEFAULT_ARCHETYPE_CAPACITY: usize = 16_usize;
}

impl Scene {
    #[must_use]
    pub fn component<T: Component>(&self, actor: Actor) -> &T {
        self.get_component(actor).unwrap()
    }

    #[must_use]
    pub fn component_mut<T: Component>(&mut self, actor: Actor) -> &mut T {
        self.get_component_mut(actor).unwrap()
    }

    #[allow(clippy::single_match_else)]
    pub fn create_actor(&mut self, src: impl ComponentSource) -> Actor {
        let actor = self.actor_factory.create();
        let types = src.types();
        let type_infos = types.iter().map(|(type_info, _)| type_info);
        let archetype = match self.get_archetype_mut_with_types(type_infos) {
            Some(archetype) => archetype,
            None => {
                let type_infos = types.iter().map(|(type_info, _)| type_info);
                let archetype =
                    Archetype::new(Scene::DEFAULT_ARCHETYPE_CAPACITY, type_infos);

                self.archetypes.push(archetype);
                self.archetypes.last_mut().unwrap()
            }
        };

        archetype.create_actor(actor, types.iter());
        std::mem::forget(src);
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
    fn get_archetype_mut_with_types<'a, Iter>(&mut self, mut types: Iter) -> Option<&mut Archetype>
    where
        Iter: Iterator<Item = &'a TypeInfo>,
    {
        self.archetypes
            .iter_mut()
            .find(|a| types.all(|ty| a.has_type(*ty)))
    }

    #[must_use]
    pub fn get_component<T: Component>(&self, actor: Actor) -> Option<&T> {
        self.get_archetype_of(actor)?.get_component(actor)
    }

    #[must_use]
    pub fn get_component_mut<T: Component>(&mut self, actor: Actor) -> Option<&mut T> {
        self.get_archetype_mut_of(actor)?.get_component_mut(actor)
    }

    #[must_use]
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    #[must_use]
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    pub fn insert_resource<T: Resource>(&mut self, res: T) -> Option<Box<T>> {
        self.resources
            .insert(res.type_id(), Box::new(res))?
            .downcast::<T>()
            .ok()
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remove_resource<T: Resource>(&mut self) -> Option<Box<T>> {
        self.resources
            .remove(&TypeId::of::<T>())?
            .downcast::<T>()
            .ok()
    }

    #[must_use]
    pub fn resource<T: Resource>(&self) -> &T {
        self.get_resource().unwrap()
    }

    #[must_use]
    pub fn resource_mut<T: Resource>(&mut self) -> &mut T {
        self.get_resource_mut().unwrap()
    }
}
