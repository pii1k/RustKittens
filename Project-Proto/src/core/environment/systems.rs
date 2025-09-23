use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::components::*;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

const TILE_WIDTH: f32 = 64.0;
const TILE_HEIGHT: f32 = 32.0;

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let diamond_mesh = meshes.add(tile_mesh(TILE_WIDTH, TILE_HEIGHT));
    let diamond_mat = materials.add(ColorMaterial::from(Color::srgb(0.5, 0.7, 0.9)));

    for x in -10..=10 {
        for y in -10..=10 {
            let iso_x = (x - y) as f32 * (TILE_WIDTH / 2.0);
            let iso_y = (x + y) as f32 * (TILE_HEIGHT / 2.0);

            commands.spawn((
                Mesh2d(diamond_mesh.clone()),
                MeshMaterial2d(diamond_mat.clone()),
                Transform::from_translation(Vec3::new(iso_x, iso_y, 0.0)),
                GlobalTransform::default(),
                EnvironmentComponent,
            ));
        }
    }
}

fn tile_mesh(w: f32, h: f32) -> Mesh {
    let hw = w / 2.0;
    let hh = h / 2.0;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());

    let vertices = vec![
        [0.0, hh, 0.0],  // top
        [hw, 0.0, 0.0],  // right
        [0.0, -hh, 0.0], // bottom
        [-hw, 0.0, 0.0], // left
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let indices = vec![0, 1, 2, 0, 2, 3];
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
