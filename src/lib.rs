#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

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
mod system;
mod type_info;

pub use actor::Actor;
pub use batch::Batch;
pub use component::Component;
pub use macros::Component;
pub use scene::Scene;
