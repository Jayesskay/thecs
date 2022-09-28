use crate::{Actor, Component, TypeInfo};

use std::{
    alloc::{alloc, dealloc, Layout},
    any::TypeId,
    collections::HashMap,
};

pub struct Archetype {
    actors: HashMap<Actor, usize>,
    capacity: usize,
    entries: Vec<Entry>,
    memory: Memory,
}

impl Archetype {
    #[must_use]
    unsafe fn component_array_ptr(&self, entry: &Entry) -> *mut u8 {
        self.memory.ptr.add(entry.offset)
    }

    #[must_use]
    unsafe fn component_ptr_from_entry(&self, entry: &Entry, index: usize) -> *mut u8 {
        self.component_array_ptr(entry)
            .add(index * entry.element_size())
    }

    #[must_use]
    unsafe fn component_ptr_from_id(&self, type_id: TypeId, index: usize) -> *mut u8 {
        let entry = self.get_entry_by_id(type_id).unwrap();
        self.memory
            .ptr
            .add(entry.offset + entry.element_size() * index)
    }

    pub fn create_actor<'a, Iter>(&mut self, actor: Actor, types: Iter)
    where
        Iter: Iterator<Item = &'a (TypeInfo, *const u8)>,
    {
        let index = self.register_actor(actor);
        for ty in types {
            let type_info = ty.0;
            let component_data = ty.1;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    component_data,
                    self.component_ptr_from_id(type_info.id, index),
                    type_info.size(),
                );
            }
        }
    }

    pub fn destroy_actor(&mut self, actor: Actor) {
        if let Some(index) = self.actors.remove(&actor) {
            for entry in &self.entries {
                if let Some(drop_fn) = entry.ty.drop {
                    unsafe {
                        drop_fn(self.component_ptr_from_entry(entry, index));
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn get_component<C: Component>(&self, actor: Actor) -> Option<&C> {
        let index = self.actors.get(&actor)?;
        let entry = self.get_entry::<C>()?;
        unsafe {
            self.component_ptr_from_entry(entry, *index)
                .cast::<C>()
                .as_ref()
        }
    }

    #[must_use]
    pub fn get_component_mut<C: Component>(&mut self, actor: Actor) -> Option<&mut C> {
        let index = self.actors.get(&actor)?;
        let entry = self.get_entry::<C>()?;
        unsafe {
            self.component_ptr_from_entry(entry, *index)
                .cast::<C>()
                .as_mut()
        }
    }

    #[must_use]
    fn get_entry<T: Component>(&self) -> Option<&Entry> {
        self.entries.iter().find(|e| e.ty.id == TypeId::of::<T>())
    }

    #[must_use]
    fn get_entry_by_id(&self, type_id: TypeId) -> Option<&Entry> {
        self.entries.iter().find(|e| e.ty.id == type_id)
    }

    #[must_use]
    pub fn has_actor(&self, actor: Actor) -> bool {
        self.actors.contains_key(&actor)
    }

    #[must_use]
    pub fn has_type(&self, ty: TypeInfo) -> bool {
        self.entries.iter().any(|entry| entry.ty.id == ty.id)
    }

    #[must_use]
    fn layout_from_iter<'a, Iter>(capacity: usize, types: Iter) -> (Layout, Vec<Entry>)
    where
        Iter: Iterator<Item = &'a TypeInfo>,
    {
        let mut memory_layout = unsafe { Layout::from_size_align_unchecked(0, 0) };
        let mut entries = Vec::new();
        for ty in types {
            let array_layout =
                Layout::from_size_align(ty.layout.size() * capacity, ty.layout.align()).unwrap();
            let (new_memory_layout, offset) = memory_layout.extend(array_layout).unwrap();
            memory_layout = new_memory_layout;
            entries.push(Entry {
                ty: *ty,
                size: array_layout.size(),
                offset,
            });
        }
        (memory_layout, entries)
    }

    #[must_use]
    pub fn new<'a, Iter>(capacity: usize, types: Iter) -> Self
    where
        Iter: Iterator<Item = &'a TypeInfo>,
    {
        let (memory_layout, entries) = Archetype::layout_from_iter(capacity, types);
        Self {
            actors: HashMap::new(),
            capacity,
            entries,
            memory: Memory::new(memory_layout),
        }
    }

    #[must_use]
    fn register_actor(&mut self, actor: Actor) -> usize {
        for i in 0..self.capacity {
            if !self.actors.values().any(|index| *index == i) {
                self.actors.insert(actor, i);
                return i;
            }
        }

        let new_capacity = self.capacity * 2;
        let (new_memory_layout, new_entries) =
            Archetype::layout_from_iter(new_capacity, self.entries.iter().map(|entry| &entry.ty));
        let new_memory = Memory::new(new_memory_layout);
        for (old_entry, new_entry) in self.entries.iter().zip(new_entries.iter()) {
            unsafe {
                let old_entry_array = self.memory.ptr.add(old_entry.offset);
                let new_entry_array = new_memory.ptr.add(new_entry.offset);
                std::ptr::copy_nonoverlapping(old_entry_array, new_entry_array, old_entry.size);
            }
        }

        let old_capacity = self.capacity;
        self.actors.insert(actor, old_capacity);
        self.capacity = new_capacity;
        self.entries = new_entries;
        self.memory = new_memory;
        old_capacity
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        self.entries
            .iter()
            .filter_map(|entry| entry.ty.drop.map(|drop| (entry, drop)))
            .for_each(|(entry, drop_fn)| {
                let array_start = unsafe { self.component_array_ptr(entry) };
                self.actors.values().for_each(|i| unsafe {
                    let component_ptr = array_start.add(i * entry.element_size());
                    drop_fn(component_ptr);
                });
            });

        /*
        for entry in &self.entries {
            if let Some(drop_fn) = entry.ty.drop {
                let array_start = unsafe { self.component_array_ptr(&entry) };
                for i in self.actors.values() {
                    unsafe {
                        let component_ptr = array_start.add(i * entry.element_size());
                        drop_fn(component_ptr);
                    }
                }
            }
        }
        */
    }
}

struct Entry {
    ty: TypeInfo,
    size: usize,
    offset: usize,
}

impl Entry {
    fn element_size(&self) -> usize {
        self.ty.layout.size()
    }
}

struct Memory {
    ptr: *mut u8,
    layout: Layout,
}

impl Memory {
    #[must_use]
    fn new(layout: Layout) -> Self {
        Self {
            ptr: unsafe { alloc(layout) },
            layout,
        }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) }
    }
}
