// use crate::prelude::*;
//
// pub fn log_state(boid_query: Query<Entity, With<Flock>>) {
//     boid_query.iter().for_each(|x| info!("Name: {:?}", x));
//
//     //info!("All boids: {:?}", names);
// }
//
//
    // // // Logging
    // if let Some(frame) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
    //     let frame = frame.value().unwrap();
    //     if frame % (60.0 * 2.0) == 0.0 {
    //         let others = grid.get_neighboids(boid_prime.0.position, 100);
    //         let cell = (
    //             (boid_prime.0.position.x / grid.cell_size) as i32,
    //             (boid_prime.0.position.y / grid.cell_size) as i32,
    //         );
    //         info!("Boid Prime cell: {:?}", cell);
    //         info!("Neighboids: {:?}", others.len());
    //     }
    // }
