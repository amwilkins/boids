use crate::prelude::*;

pub fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 5.;
    let move_delta = direction * move_speed * time.delta_secs();

    // let old_pos = transform.translation.xy();
    // let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
    // let new_pos = (old_pos + move_delta).clamp(-limit, limit);
    //
    // transform.translation.x = new_pos.x;
    // transform.translation.y = new_pos.y;

    for mut transform in &mut players {
        transform.translation += move_delta.extend(0.);
    }
}
