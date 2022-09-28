#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Actor(usize);

#[derive(Default)]
pub struct ActorFactory {
    created: usize,
}

impl ActorFactory {
    #[must_use]
    pub fn create(&mut self) -> Actor {
        self.created += 1;
        Actor(self.created)
    }
}
