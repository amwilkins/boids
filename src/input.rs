use crate::prelude::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    colliders: Query<(&Transform, &Sprite), (With<Collider>, Without<Player>)>,
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
    let mut move_delta = direction * move_speed * time.delta_secs(); //.clamp(Vec2::ZERO, Vec2::new(5.0,5.0));

    for mut transform in &mut player {
        // set target position
        let target = transform.translation + move_delta.extend(0.0);
        // loop over all colliders
        for (wall_transform, wall_sprite) in &colliders {
            // only check if within 50
            if target.distance(wall_transform.translation) < 50.0 {
                // create bounding boxes
                let target_box =
                    Aabb2d::new(target.xy(), Vec2::new(PLAYER_SIZE, PLAYER_SIZE) / 2.0);
                let wall_bound = Aabb2d::new(
                    wall_transform.translation.xy(),
                    wall_sprite.custom_size.unwrap() / 2.0,
                );
                let collision = check_for_collision(target_box, wall_bound);

                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => move_delta.x = move_delta.x.clamp(-move_speed, 0.0),
                        Collision::Right => move_delta.x = move_delta.x.clamp(0.0, move_speed),
                        Collision::Top => move_delta.y = move_delta.y.clamp(0.0, move_speed),
                        Collision::Bottom => move_delta.y = move_delta.y.clamp(-move_speed, 0.0),
                    }
                }
            }
        }
        transform.translation += move_delta.extend(0.);
        // stay in walls
        transform.translation = transform
            .translation
            .clamp(Vec3::splat(0.3), Vec3::splat(MAP_SIZE as f32 - 0.3));
    }
}

fn check_for_collision(self_bounding_box: Aabb2d, collider: Aabb2d) -> Option<Collision> {
    if !self_bounding_box.intersects(&collider) {
        return None;
    }
    // which wall is closest to the edge
    let closest_on_player = self_bounding_box.closest_point(collider.bounding_circle().center);
    let closest = collider.closest_point(closest_on_player);
    let offset = collider.bounding_circle().center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x > 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y < 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };
    Some(side)
}
