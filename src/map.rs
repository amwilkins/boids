use crate::prelude::*;

pub fn generate_map(mut commands: Commands, walls: Query<Entity, With<Wall>>) {
    for wall in &walls {
        commands.entity(wall).despawn();
    }

    let width = 3.0;
    let height = 3.0;

    commands.spawn((
        Wall,
        Collider,
        Transform::from_translation(Vec3::new(2.0 + (width / 2.0), 8.0 + (height / 2.0), 1.0)),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
    ));
}
