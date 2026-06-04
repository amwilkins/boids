use crate::prelude::*;

pub fn create_partitions(boid_query: Query<EntityRef, With<Flock>>, mut grid: ResMut<Grid>) {
    grid.cells.clear();

    boid_query.iter().for_each(|entity| {
        let boid = entity.get::<Boid>().unwrap();
        grid.insert(&boid);
    });
}

/*
 * Optimization: Attempted to clear each vec without deallocating memory, but that slowed things
 * way down. Debug info didn't really help (because I don't understand debug info);
 *
 */
