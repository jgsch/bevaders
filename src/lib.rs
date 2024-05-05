use anyhow::Result;
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages, render_resource::Shader,
        view::screenshot::ScreenshotManager,
    },
    window::{PrimaryWindow, WindowMode},
};
use image::{DynamicImage, RgbImage};
use opencv::{
    core::Mat,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

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

pub struct Webcam {
    cam: VideoCapture,
}

impl Webcam {
    pub fn new() -> Self {
        Self {
            cam: VideoCapture::new(0, CAP_ANY).unwrap(),
        }
    }

    pub fn get(&mut self) -> Result<Image> {
        let mut frame = Mat::default();
        self.cam.read(&mut frame)?;

        let data = frame.data_bytes()?;
        let (w, h) = (frame.cols() as u32, frame.rows() as u32);
        let mut image = RgbImage::new(w, h);
        for (pixi, i) in (0..data.len()).step_by(3).enumerate() {
            let b = data[i];
            let g = data[i + 1];
            let r = data[i + 2];
            let impix = image::Rgb([r, g, b]);
            let x = pixi as u32 % w;
            let y = pixi as u32 / w;
            image.put_pixel(x, y, impix);
        }
        let dynamic_image = DynamicImage::ImageRgb8(image);

        Ok(Image::from_dynamic(
            dynamic_image,
            true,
            RenderAssetUsages::all(),
        ))
    }
}

impl Default for Webcam {
    fn default() -> Self {
        Self::new()
    }
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