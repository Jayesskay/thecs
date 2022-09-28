use crate::Scene;

pub trait SystemArg: 'static {
    fn from_scene(scene: &Scene) -> Self;
}
