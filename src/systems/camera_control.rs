use crate::prelude::*;
use bevy::math::ops::powf;
use bevy::input::*;

pub fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (_, mut transform, mut projection) = camera_query.into_inner();

    let fspeed = 100.0 * time.delta_secs();

    // Camera movement controls
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += fspeed;
    }

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }

    // if let Some(viewport) = camera.viewport.as_mut() {
    //     // Reset viewport size on window resize
    //     if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
    //         viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
    //     }
    //
    //     // Viewport movement controls
    //     if input.pressed(KeyCode::KeyW) {
    //         viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
    //     }
    //     if input.pressed(KeyCode::KeyS) {
    //         viewport.physical_position.y += uspeed;
    //     }
    //     if input.pressed(KeyCode::KeyA) {
    //         viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
    //     }
    //     if input.pressed(KeyCode::KeyD) {
    //         viewport.physical_position.x += uspeed;
    //     }
    //
    //     // Bound viewport position so it doesn't go off-screen
    //     viewport.physical_position = viewport
    //         .physical_position
    //         .min(window_size - viewport.physical_size);
    //
    //     // Viewport size controls
    //     if input.pressed(KeyCode::KeyI) {
    //         viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
    //     }
    //     if input.pressed(KeyCode::KeyK) {
    //         viewport.physical_size.y += uspeed;
    //     }
    //     if input.pressed(KeyCode::KeyJ) {
    //         viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
    //     }
    //     if input.pressed(KeyCode::KeyL) {
    //         viewport.physical_size.x += uspeed;
    //     }
    //
    //     // Bound viewport size so it doesn't go off-screen
    //     viewport.physical_size = viewport
    //         .physical_size
    //         .min(window_size - viewport.physical_position)
    //         .max(UVec2::new(20, 20));
    // }
}

