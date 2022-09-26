#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Actor(usize);

#[derive(Default)]
pub struct Factory {
    created: usize,
}

impl Factory {
    #[must_use]
    pub fn create(&mut self) -> Actor {
        self.created += 1;
        Actor(self.created)
    }
}
