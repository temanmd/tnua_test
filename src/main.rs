//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::*;
use iyes_perf_ui::{PerfUiPlugin, prelude::PerfUiDefaultEntries};

struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_debug_ui);
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
        app.add_plugins(PerfUiPlugin);
    }
}

fn add_debug_ui(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::srgba(1.0, 0.95, 0.8, 1.0),
            brightness: 1500.0,
        })
        .insert_resource(ClearColor(Color::srgb(0.3, 0.3, 0.3)))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaRapier3dPlugin::new(FixedUpdate),
        ))
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            movement_system.in_set(TnuaUserControlsSystemSet),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(500.0, 10.0, 500.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -5.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(250.0, 5.0, 250.0),
    ));
    // player
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(10.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 50.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule_y(10.0, 10.0),
        TnuaController::default(),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        TnuaRapier3dSensorShape(Collider::capsule_y(0.0, 9.9)),
        Velocity::default(),
    ));
    // light
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::PI / 4.,
        )),
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .build(),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 130.0, 200.0).looking_at(Vec3::new(0.0, 40.0, 0.0), Vec3::Y),
    ));
}

fn movement_system(mut query: Query<&mut TnuaController, With<TnuaRapier3dSensorShape>>) {
    for mut controller in query.iter_mut() {
        let direction = Vec3::new(60.0, 0.0, 0.0);
        info!("wqdqwd");

        controller.basis(TnuaBuiltinWalk {
            // Move in the direction the player entered, at a speed of 10.0:
            desired_velocity: direction * 10.0,

            // Must be larger than the height of the entity's center from the bottom of its
            // collider, or else the character will not float and Tnua will not work properly:
            float_height: 5.0,

            // TnuaBuiltinWalk has many other fields that can be configured:
            ..Default::default()
        });
    }
}
