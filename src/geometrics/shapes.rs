use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, Mesh2d, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MeshMaterial2d;

pub struct Geometrics;

impl Geometrics {
    /// Cria um círculo 2D
    pub fn circle(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(size))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria um quadrado 2D
    pub fn square(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(size, size))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria um retângulo 2D
    pub fn rectangle(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(width, height))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria um triângulo 2D
    pub fn triangle(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        // Criar um triângulo equilátero
        let height = size * (3.0_f32).sqrt() / 2.0;
        let half_base = size / 2.0;

        commands
            .spawn((
                Mesh2d(meshes.add(Triangle2d::new(
                    Vec2::new(0.0, height / 2.0),         // Topo
                    Vec2::new(-half_base, -height / 2.0), // Esquerda inferior
                    Vec2::new(half_base, -height / 2.0),  // Direita inferior
                ))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria um polígono regular 2D
    pub fn polygon(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        sides: usize,
        size: f32,
        position: Vec2,
    ) -> Entity {
        commands
            .spawn((
                Mesh2d(meshes.add(RegularPolygon::new(size, sides as u32))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria uma elipse 2D
    pub fn ellipse(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity {
        commands
            .spawn((
                Mesh2d(meshes.add(Ellipse::new(width / 2.0, height / 2.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Desenha uma forma customizada baseada em coordenadas
    ///
    /// # Parâmetros
    /// - `coordinates`: Lista de pontos Vec2 que definem os vértices da forma
    /// - `color`: Cor da forma
    /// - `position`: Posição central da forma no mundo
    /// - `scale`: Escala para aplicar aos pontos (1.0 = tamanho original)
    /// - `filled`: Se true, preenche a forma; se false, desenha apenas o contorno
    pub fn draw(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        coordinates: Vec<Vec2>,
        color: Color,
        position: Vec2,
        scale: f32,
        filled: bool,
    ) -> Entity {
        if coordinates.len() < 3 {
            panic!("Uma forma precisa de pelo menos 3 pontos!");
        }

        let mesh = if filled {
            Self::create_filled_mesh(coordinates, scale)
        } else {
            Self::create_outline_mesh(coordinates, scale)
        };

        commands
            .spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ))
            .id()
    }

    /// Cria uma mesh preenchida a partir de coordenadas
    fn create_filled_mesh(coordinates: Vec<Vec2>, scale: f32) -> Mesh {
        let vertices: Vec<[f32; 3]> = coordinates
            .iter()
            .map(|v| [v.x * scale, v.y * scale, 0.0])
            .collect();

        // Criar índices para triangulação (fan triangulation)
        let mut indices = Vec::new();
        for i in 1..coordinates.len() - 1 {
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));

        // Adicionar normais e UVs
        let normals = vec![[0.0, 0.0, 1.0]; coordinates.len()];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        let uvs: Vec<[f32; 2]> = coordinates
            .iter()
            .map(|v| [(v.x + 1.0) * 0.5, (v.y + 1.0) * 0.5])
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }

    /// Cria uma mesh de contorno a partir de coordenadas
    fn create_outline_mesh(coordinates: Vec<Vec2>, scale: f32) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let line_width = 2.0; // Largura da linha

        for i in 0..coordinates.len() {
            let current = coordinates[i] * scale;
            let next = coordinates[(i + 1) % coordinates.len()] * scale;

            // Calcular direção perpendicular para criar espessura
            let direction = (next - current).normalize();
            let perpendicular = Vec2::new(-direction.y, direction.x) * line_width * 0.5;

            // Adicionar 4 vértices para cada segmento
            let base_index = (i * 4) as u32;

            vertices.push([
                current.x - perpendicular.x,
                current.y - perpendicular.y,
                0.0,
            ]);
            vertices.push([
                current.x + perpendicular.x,
                current.y + perpendicular.y,
                0.0,
            ]);
            vertices.push([next.x + perpendicular.x, next.y + perpendicular.y, 0.0]);
            vertices.push([next.x - perpendicular.x, next.y - perpendicular.y, 0.0]);

            // Criar dois triângulos para formar um retângulo
            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
        mesh.insert_indices(Indices::U32(indices));

        // Adicionar normais e UVs
        let normals = vec![[0.0, 0.0, 1.0]; vertices.len()];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        let uvs = vec![[0.5, 0.5]; vertices.len()];
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }

    /// Método auxiliar para criar uma estrela usando draw
    pub fn draw_star(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        points: usize,
        outer_radius: f32,
        inner_radius: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity {
        let mut coordinates = Vec::new();
        let angle_step = std::f32::consts::TAU / (points * 2) as f32;

        for i in 0..points * 2 {
            let angle = angle_step * i as f32 - std::f32::consts::FRAC_PI_2;
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };
            coordinates.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
        }

        Self::draw(
            commands,
            meshes,
            materials,
            coordinates,
            color,
            position,
            1.0,
            filled,
        )
    }

    /// Método auxiliar para criar um coração usando draw
    pub fn draw_heart(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        size: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity {
        let mut coordinates = Vec::new();
        let steps = 30;

        for i in 0..=steps {
            let t = i as f32 / steps as f32 * std::f32::consts::TAU;
            let x = 16.0 * t.sin().powi(3);
            let y =
                13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
            coordinates.push(Vec2::new(x * 3.0, y * 3.0));
        }

        Self::draw(
            commands,
            meshes,
            materials,
            coordinates,
            color,
            position,
            size / 50.0,
            filled,
        )
    }
}
