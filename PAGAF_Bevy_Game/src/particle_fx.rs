use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::prelude::{ParticleEffect, EffectAsset, ColorOverLifetimeModifier, SpawnerSettings};
use bevy_hanabi::CompiledParticleEffect;
use bevy::prelude::AlphaMode;
use rand::Rng;
use rand::thread_rng;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource)]
pub struct ParticleEffects {
    pub spawn_handle: Handle<EffectAsset>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_particle_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.5, 1.0, 0.0));

    let mut module = Module::default();
    // Radius spherical volume position = 0.5
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(2.0),
    };
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(1.5));
    let update_accel = AccelModifier::new(module.lit(Vec3::Y * 1.5));

    let effect = EffectAsset::new(
        512, // Max capacity of particles
        SpawnerSettings::burst(100.0.into(), 1000.0.into()), // emits 100 particles once
        module,
    )
        .with_name("spawn_effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient, ..default() });

    let handle = effects.add(effect);
    commands.insert_resource(ParticleEffects { spawn_handle: handle });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_on_place(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    handle: Handle<EffectAsset>,
    position: Vec3,
) {
    commands.spawn((
        ParticleEffect::new(handle),
        CompiledParticleEffect::default(),
        Transform::from_translation(position),
        GlobalTransform::default(),
    ));
}


#[cfg(target_arch = "wasm32")]
pub fn spawn_on_place(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let particle_count = 150;

    use rand::random;
    use rand::Rng;
    let hues = [285.0, 320.0, 190.0, 130.0, 220.0];
    let index = thread_rng().gen_range(0..hues.len());
    let hue = hues[index];
    let base_color = Color::hsl(hue, 1.0, 0.6);
    let linear = base_color.to_linear();
    let emissive_color = Color::linear_rgba(
        linear.red * 0.5,
        linear.green * 0.5,
        linear.blue * 0.5,
        linear.alpha,
    );

    let material = materials.add(StandardMaterial {
        base_color,
        perceptual_roughness: 1.0,
        metallic: 0.5,
        emissive: emissive_color.to_linear(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mesh = meshes.add(Cuboid::new(0.05, 0.05, 0.05));

    for _ in 0..particle_count {
        let offset = Vec3::new(
            (random::<f32>() - 0.5),
            0.5 + random::<f32>() * 0.5,
            //(random::<f32>() * 0.5),
            (random::<f32>() - 0.5),
        );

        let velocity = Vec3::new(
            (random::<f32>() - 0.5) * 0.5,
            1.0 + random::<f32>(),
            (random::<f32>() - 0.5) * 0.5,
        );

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform {
                translation: position + offset,
                ..default()
            },
            GlobalTransform::default(),
            Velocity(velocity),
            FadeOutTimer(Timer::from_seconds(3.0, TimerMode::Once)),
        ));
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

#[derive(Component)]
pub struct FadeOutTimer(Timer);

pub fn fade_out_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FadeOutTimer, &mut Transform)>,
) {
    for (entity, mut timer, mut transform) in &mut query {
        timer.0.tick(time.delta());
        let shrink_rate = 2.5; // Higher = faster shrinking
        let factor = (1.0 - time.delta_secs()).powf(shrink_rate);
        transform.scale *= factor;

        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}