#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod prelude {
    pub use crate::{actor::Actor, component::Component, scene::Scene};
    pub use macros::Component;
}

mod actor;
mod archetype;
mod batch;
mod component;
mod resource;
mod scene;
mod type_info;

pub use actor::Actor;
pub use batch::Batch;
pub use component::Component;
pub use macros::Component;
pub use scene::Scene;
