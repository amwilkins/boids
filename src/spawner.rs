use crate::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(5., 5., 100.)),
        Sprite {
            color: Color::srgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..default()
        },
    ));
}

pub fn spawn_mob(mut commands: Commands){
    for num in 1..MOB_COUNT {

        let x = (num * 3) as f32;
        let y = (num * 3) as f32;
        let pos = Vec3::new(x, y, 50.);
        commands.spawn((
            Mob,
            ChasePlayer,
            Transform::from_translation(pos),
            Sprite {
                color: Color::srgb(1., 0.2, 0.2),
                custom_size: Some(Vec2::new(0.4, 0.4)),
                ..default()
            },
        ));
    }
}
