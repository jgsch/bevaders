use anyhow::Result;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use opencv::{
    core::Mat,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture},
};

pub struct Webcam {
    receiver: std::sync::mpsc::Receiver<Mat>,
}

#[cfg(feature = "webcam_gstreamer")]
fn gstreamer_pipeline(
    sensor_id: i32,
    capture_width: i32,
    capture_height: i32,
    display_width: i32,
    display_height: i32,
    framerate: i32,
    flip_method: i32,
) -> String {
    format!(
        "nvarguscamerasrc sensor-id={} ! video/x-raw(memory:NVMM), width=(int){}, height=(int){}, framerate=(fraction){}/1 ! nvvidconv flip-method={} ! video/x-raw, width=(int){}, height=(int){}, format=(string)BGRx ! videoconvert ! video/x-raw, format=(string)BGR ! appsink",
        sensor_id, capture_width, capture_height, framerate, flip_method, display_width, display_height
    )
}

impl Webcam {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let mut cam;

            #[cfg(feature = "webcam_gstreamer")]
            {
                let pipeline = gstreamer_pipeline(0, 1920, 1080, 480, 360, 30, 0);
                cam = VideoCapture::from_file(&pipeline, videoio::CAP_GSTREAMER).unwrap();
            }

            #[cfg(not(feature = "webcam_gstreamer"))]
            {
                cam = VideoCapture::new(0, videoio::CAP_ANY).unwrap();
            }

            loop {
                let mut frame = Mat::default();
                cam.read(&mut frame).unwrap();

                let mut rgba_frame = Mat::default();
                imgproc::cvt_color(&frame, &mut rgba_frame, imgproc::COLOR_BGR2RGBA, 0).unwrap();

                tx.send(rgba_frame).unwrap();
            }
        });

        Self { receiver: rx }
    }

    pub fn get(&mut self) -> Result<Image> {
        // Retrieve the most recent Image from the receiver channel, to get the last
        // available frame if any, or blocking with recv() to wait for a frame if the
        // channel is currently empty
        let frame = match self.receiver.try_iter().last() {
            Some(last_frame) => last_frame,
            None => self.receiver.recv().unwrap(),
        };

        let (width, height) = (frame.cols(), frame.rows());
        let data = frame.data_bytes().unwrap();

        let image = Image::new(
            Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data.into(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        );

        Ok(image)
    }
}

impl Default for Webcam {
    fn default() -> Self {
        Self::new()
    }
}
