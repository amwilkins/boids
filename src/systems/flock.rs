// use crate::prelude::*;
//
// pub fn flock(
//     mut mobs: Query<&mut Transform, With<Mob>>,
//     player: Query<&Transform, (With<Player>, Without<Mob>)>,
//     time: Res<Time>,
// ) {
//     let move_speed = 5.;
//
//     // Cohesion
//     //
//     // Separation
//     //
//     // Alignment
//
//
// let player_position = if let Ok(player_transform) = player.single() {
//         player_transform.translation.truncate()
//     } else {
//         Vec2::ZERO
//     };
//
//     for mut mob_pos in mobs.iter_mut() {
//         let mut target = player_position - mob_pos.translation.truncate();//.normalize_or_zero();
//         if target.x.abs() < 0.15 {
//             target.x = 0.;
//         }
//         if target.y.abs() < 0.15 {
//             target.y = 0.;
//         }
//         let move_delta = target.normalize_or_zero() * move_speed * time.delta_secs();
//         mob_pos.translation += move_delta.extend(0.);
//     }
// }

