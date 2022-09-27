use crate::{Scene, Stage};

#[derive(Default)]
pub struct Schedule {
    stages: Vec<Stage>,
}

impl Schedule {
    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self, scene: &Scene) {
        for stage in &self.stages {
            stage.run(scene);
        }
    }
}
