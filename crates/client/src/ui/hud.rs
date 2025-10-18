// =============================================================================
// UI - HUD (Heads-Up Display)
// =============================================================================

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::camera::MainCamera;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct CameraInfoText;

pub fn setup_hud(mut commands: Commands) {
    // Root node pour HUD
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Top-left: FPS
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                FpsText,
            ));

            // Top-left sous FPS: Camera info
            parent.spawn((
                Text::new("Camera\nPos: (0, 0)\nZoom: 1.0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 1.0)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                CameraInfoText,
            ));
        });
}

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {:.0}", value);
            }
        }
    }
}

pub fn update_camera_info_text(
    camera: Query<(&Transform, &Projection), With<MainCamera>>,
    mut query: Query<&mut Text, With<CameraInfoText>>,
) {
    let Ok((transform, projection)) = camera.single() else {
        return;
    };

    for mut text in &mut query {
        
        let scale = if let Projection::Orthographic(ortho) = projection {
            ortho.scale
        } else {
            1.0
        };

        **text = format!(
            "Camera\nPos: ({:.0}, {:.0})\nZoom: {:.2}",
            transform.translation.x,
            transform.translation.y,
            scale
        );
    }
}