use bevaders::{BevadersPlugin, BillBoardQuad, Webcam, WindowDimensions};
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDimensions::default())
        .insert_non_send_resource(Webcam::new())
        .add_plugins((
            DefaultPlugins,
            BevadersPlugin,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut msd: ResMut<WindowDimensions>,
    mut webcam: NonSendMut<Webcam>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let win = windows
        .get_single()
        .expect("Should be impossible to NOT get a window");
    let (width, height) = (win.width(), win.height());

    *msd = WindowDimensions(Vec2 {
        x: width / 2.0,
        y: height / 2.0,
    });

    let texture = asset_server.add(webcam.get().unwrap());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default(),
            material: materials.add(CustomMaterial { img: texture }),
            ..default()
        },
        BillBoardQuad,
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(1, dimension = "2d")]
    #[sampler(2)]
    pub img: Handle<Image>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "webcam_kaleidoscope.wgsl".into()
    }
}

pub fn capture(
    shader_handle: Query<&Handle<CustomMaterial>>,
    mut shader_material: ResMut<Assets<CustomMaterial>>,
    mut webcam: NonSendMut<Webcam>,
    asset_server: Res<AssetServer>,
) {
    let Ok(handle) = shader_handle.get_single() else {
        return;
    };
    if let Some(shader_material) = shader_material.get_mut(handle) {
        shader_material.img = asset_server.add(webcam.get().unwrap());
    }
}
