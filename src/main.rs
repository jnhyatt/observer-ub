//! This is a minimal reproduction of a bug I've encountered in Bevy. The bug seems to be related to
//! removing an `Observer` component from an entity. When the event is triggered, the application
//! enters unreachable code, resulting in a debug panic and UB in release. To trigger the bug,
//! run the app and click anywhere on the window. The bug is triggered by the `Pointer<Down>` event.

use bevy::{
    picking::{
        backend::HitData,
        pointer::{Location, PointerId},
    },
    prelude::*,
    render::camera::NormalizedRenderTarget,
    window::WindowRef,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, disable_observer).chain())
        .add_systems(Update, fire_observer)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let pick_target = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .id();
    // The reason I picked `Pointer<Down>` here is that triggering the observer manually (via
    // `world.trigger`) doesn't reproduce the issue. I'm not sure exactly what breaks it, but
    // pointer events break it reliably.
    let observer = Observer::new(|_: Trigger<Pointer<Down>>| {}).with_entity(pick_target);
    commands.spawn((PickObserver, observer));
}

fn fire_observer(
    window: Single<Entity, With<Window>>,
    camera: Single<Entity, With<Camera>>,
    picked: Single<Entity, With<Node>>,
    mut commands: Commands,
) {
    commands.trigger(Pointer::<Down>::new(
        *picked,
        PointerId::Mouse,
        Location {
            target: NormalizedRenderTarget::Window(
                WindowRef::Primary.normalize(Some(*window)).unwrap(),
            ),
            position: Vec2::ZERO,
        },
        Down {
            button: PointerButton::Primary,
            hit: HitData {
                camera: *camera,
                depth: 0.0,
                position: None,
                normal: None,
            },
        },
    ));
}

#[derive(Component)]
pub struct PickObserver;

pub fn disable_observer(pick_observer: Single<Entity, With<PickObserver>>, mut commands: Commands) {
    commands.entity(*pick_observer).remove::<Observer>();
}
