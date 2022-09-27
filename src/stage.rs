use crate::{
    system::{IntoSystem, System},
    Scene,
};

#[derive(Default)]
pub struct Stage {
    systems: Vec<Box<dyn System>>,
}

impl Stage {
    pub fn add_system<Args>(&mut self, system: impl IntoSystem<Args>) {
        self.systems.push(system.into_system());
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self, scene: &Scene) {
        for system in &self.systems {
            system.run(scene);
        }
    }
}
