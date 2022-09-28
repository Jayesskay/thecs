mod into_system;
mod system_arg;
mod system_function;

pub use into_system::IntoSystem;
pub use system_arg::SystemArg;
pub use system_function::SystemFunction;

use crate::Scene;

pub trait System {
    fn run(&self, scene: &Scene);
}
