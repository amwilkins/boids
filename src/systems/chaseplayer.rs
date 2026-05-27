// use crate::prelude::*;
//
// pub fn chase_player(
//     mut mobs: Query<&mut Transform, With<Mob>>,
//     player: Query<&Transform, (With<Player>, Without<Mob>)>,
//     time: Res<Time>,
// ) {
//     let move_speed = 5.;
//
//     // player position
//     let player_position = if let Ok(player_transform) = player.single() {
//         player_transform.translation.truncate()
//     } else {
//         Vec2::ZERO
//     };
//
//     // move each mob
//     for mut mob_pos in mobs.iter_mut() {
//         let mut target = player_position - mob_pos.translation.truncate(); //.normalize_or_zero();
//
//         // stop when close
//         if target.x.abs() < 1.0 {
//             target.x = 0.;
//         }
//         if target.y.abs() < 1.0 {
//             target.y = 0.;
//         }
//
//         let move_delta = target.normalize_or_zero() * move_speed * time.delta_secs();
//         mob_pos.translation += move_delta.extend(0.);
//     }
// }
