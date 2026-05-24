use crate::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(-40., -40., 100.)),
        Sprite {
            color: Color::srgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(0.7, 0.7)),
            ..default()
        },
    ));
}


pub fn spawn_mob(mut commands: Commands) {
    commands.spawn((
        Mob,
        ChasePlayer,
        Transform::from_translation(Vec3::new(-30., -30., 100.)),
        Sprite {
            color: Color::srgb(1., 0.2, 0.2),
            custom_size: Some(Vec2::new(0.9, 0.9)),
            ..default()
        },
    ));
}
