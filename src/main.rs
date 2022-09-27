use thecs::prelude::*;

#[derive(Component)]
struct Foo(i32);

#[derive(Component)]
struct Bar(i32);

fn main() {
    let mut scene = Scene::new();
    scene.create_actor((Foo(69_i32), Bar(420_i32)));
    let mut stage = Stage::new();
    stage.add_system(foo_system);
    stage.add_system(bar_system);
    let mut schedule = Schedule::new();
    schedule.add_stage(stage);
    schedule.run(&scene);
}

fn foo_system() {
    println! { "Foo!" };
}

fn bar_system() {
    println! { "Bar!" };
}
