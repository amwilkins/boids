// use crate::prelude::*;
// //use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
//
// pub fn render_boids(
//     mut boid_query: Query<(&mut Boid, &mut Transform), With<Flock>>,
//     //mut gizmos: Gizmos,
//     //diagnostics: Res<DiagnosticsStore>,
// ) {
//     // apply changes
//     for (boid, mut transform) in boid_query.iter_mut() {
//
//         transform.translation = boid.position.extend(50.0);
//         // rotate
//         let heading = transform.translation.xy() + boid.velocity;
//         transform.look_at(heading.extend(0.0), Dir3::Z);
//     }
// }
