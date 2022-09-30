#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

pub mod prelude {
    pub use crate::{
        actor::Actor,
        component::{Component, ComponentSource},
        resource::Resource,
        scene::Scene,
    };

    pub use macros::Component;
}

mod actor;
mod archetype;
mod component;
mod resource;
mod scene;
mod type_info;

pub use actor::Actor;
pub use component::{Component, ComponentSource};
pub use macros::Component;
pub use resource::Resource;
pub use scene::Scene;

use actor::ActorFactory;
use archetype::Archetype;
use type_info::TypeInfo;
