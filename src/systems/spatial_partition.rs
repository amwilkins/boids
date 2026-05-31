use bevy::diagnostic::DiagnosticsStore;

use crate::prelude::*;

pub fn create_partitions(
    boid_query: Query<EntityRef, With<Flock>>,
    mut grid: ResMut<Grid>,
    _diagnostics: Res<DiagnosticsStore>,
) {
    grid.cells.clear();

    boid_query.iter().for_each(|entity| {
        let boid_position = entity.get::<Boid>().unwrap().position;

        grid.insert(&entity.id(), &boid_position);
    });
}

