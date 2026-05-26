// use bevy::math::bounding::{Aabb2d, IntersectsVolume};
//
// use crate::prelude::*;
//
// pub fn resolve_collisions(
//     mut player: Single<&mut Transform, (With<Player>, Without<Mob>)>,
//     walls: Query<(&Transform, &Sprite), (With<Wall>, Without<Player>)>,
// ) {
//     let mut player_transform = player.into_inner();
//
//     for (wall_transform, wall_sprite) in &walls {
//         //let wall_size = wall_sprite.custom_size.expect("Wall has no size");
//         let player_pos = player_transform.translation.xy();
//         let wall_pos = wall_transform.translation.xy();
//
//         // create bounding boxes
//         let p = Aabb2d::new(player_pos, Vec2::new(PLAYER_SIZE, PLAYER_SIZE) / 2.0);
//         let wall = Aabb2d::new(
//             wall_transform.translation.xy(),
//             wall_sprite.custom_size.unwrap() / 2.0,
//         );
//         if p.intersects(&wall) {
//             let wall_to_player = (player_pos - wall_pos).normalize();
//             let wall_dir = ops::atan2(wall_to_player.y, wall_to_player.x);
//             info!("{}", ops::atan2(wall_to_player.y, wall_to_player.x));
//
//             if wall_dir < 0.0 {
//                 player_transform.translation.y += wall_to_player.y.signum() * 0.1;
//             }
//
//             // let dir = tan(0.0);
//             // info!("{}", dir);
//
//             // player_transform.translation.x += wall_to_player.x.signum();
//             // player_transform.translation.y += wall_to_player.y.signum();
//         }
//     }
// }
//
// // // // get desired location, check against all collision entities
// // pub fn check_for_collision(
// //     self_bounding_box: Aabb2d,
// //     colliders: Query<(&Transform, &Sprite), (With<Collider>, Without<Player>)>,
// // ) -> bool {
// //     for (wall_transform, wall_sprite) in &colliders {
// //         let wall_bound = Aabb2d::new(
// //             wall_transform.translation.xy(),
// //             wall_sprite.custom_size.unwrap() / 2.0,
// //         );
// //         if self_bounding_box.intersects(&wall_bound) {
// //             return true;
// //         }
// //     }
// //     false
// // }
