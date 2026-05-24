use crate::prelude::*;

pub fn generate_map(
    mut commands: Commands,
    walls: Query<Entity, With<Walls>>,
    //mut rng: GlobalEntropy<WyRand>,
) {
    for wall in &walls {
        commands.entity(wall).despawn();
    }
}
