use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub use crate::prelude::*;

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
    ) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) && let Some(value) = fps.smoothed(){
            **span = format!("{value:.2}");
        }
    }
}
