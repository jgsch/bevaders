use bevy::{
    asset::load_internal_asset,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{render_resource::Shader, view::screenshot::ScreenshotManager},
    window::{PrimaryWindow, WindowMode, WindowResized},
};

pub mod webcam;

pub struct ShaderLibraryPlugin;

impl Plugin for ShaderLibraryPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            Handle::weak_from_u128(16813517719070609599),
            "common.wgsl",
            Shader::from_wgsl
        );
    }
}

pub struct BevadersPlugin;

impl Plugin for BevadersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDimensions::default())
            .add_plugins(ShaderLibraryPlugin)
            .add_systems(
                Update,
                (
                    quit,
                    fullscreen,
                    size_quad.run_if(on_event::<WindowResized>()),
                ),
            );

        #[cfg(debug_assertions)] // debug/dev builds only
        {
            app.add_plugins(LogDiagnosticsPlugin::default());
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
    }
}

#[derive(Component)]
pub struct BillBoardQuad;

#[derive(Resource, DerefMut, Deref, Default, Debug)]
pub struct WindowDimensions(pub Vec2);

pub fn size_quad(
    windows: Query<&Window>,
    mut query: Query<&mut Transform, With<BillBoardQuad>>,
    mut msd: ResMut<WindowDimensions>,
) {
    let win = windows
        .get_single()
        .expect("Should be impossible to NOT get a window");
    let (width, height) = (win.width(), win.height());

    query.iter_mut().for_each(|mut transform| {
        *msd = WindowDimensions(Vec2 {
            x: width,
            y: height,
        });
        transform.scale = Vec3::new(width, height, 1.0);
    });
}

pub fn quit(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyQ) {
        std::process::exit(0)
    }
}

pub fn screenshot(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}

pub fn fullscreen(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::KeyF) {
        let mut window = windows.single_mut();
        match window.mode {
            WindowMode::BorderlessFullscreen => {
                window.mode = WindowMode::Windowed;
            }
            WindowMode::Windowed => {
                window.mode = WindowMode::BorderlessFullscreen;
            }
            _ => (),
        }
    }
}
