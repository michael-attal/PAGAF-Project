use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::prelude::{ParticleEffect, EffectAsset, ColorOverLifetimeModifier, SpawnerSettings};
use bevy_hanabi::CompiledParticleEffect;

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
        SpawnerSettings::burst(30.0.into(), 0.0.into()), // emits 30 particles once
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
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    use rand::random;

    let texture = asset_server.load("spark.png");
    let particle_count = 100;

    for _ in 0..particle_count {
        // Spawn higher above the placement point
        let offset = Vec3::new(
            (random::<f32>() - 0.5) * 1.0,
            1.0 + random::<f32>() * 0.5, // +1.0 to lift above ground
            (random::<f32>() - 0.5) * 1.0,
        );

        // Strong upward motion, low lateral movement
        let velocity = Vec3::new(
            (random::<f32>() - 0.5) * 0.5,
            3.0 + random::<f32>() * 1.0, // goes up
            (random::<f32>() - 0.5) * 0.5,
        );

        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform {
                translation: position + offset + Vec3::new(0.0, 0.0, 10.0),
                scale: Vec3::splat(10.0), // Big for debug ftm
                ..default()
            },
            Velocity(velocity),
            FadeOutTimer(Timer::from_seconds(15.0, TimerMode::Once)), // Visible longer ftm
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
        transform.scale *= 0.9; // shrink

        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}