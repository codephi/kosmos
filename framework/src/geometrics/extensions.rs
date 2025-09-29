use bevy::prelude::*;
use super::shapes::Geometrics;

/// Extensão trait para facilitar o uso com Commands
pub trait GeometricsExt {
    fn spawn_circle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity;

    fn spawn_square(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity;

    fn spawn_rectangle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity;

    fn spawn_triangle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity;

    fn spawn_polygon(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        sides: usize,
        size: f32,
        position: Vec2,
    ) -> Entity;

    fn spawn_ellipse(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity;

    fn draw(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        coordinates: Vec<Vec2>,
        color: Color,
        position: Vec2,
        scale: f32,
        filled: bool,
    ) -> Entity;

    fn draw_star(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        points: usize,
        outer_radius: f32,
        inner_radius: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity;

    fn draw_heart(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        size: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity;
}

impl GeometricsExt for Commands<'_, '_> {
    fn spawn_circle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::circle(self, meshes, materials, color, size, position)
    }

    fn spawn_square(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::square(self, meshes, materials, color, size, position)
    }

    fn spawn_rectangle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::rectangle(self, meshes, materials, color, width, height, position)
    }

    fn spawn_triangle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        size: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::triangle(self, meshes, materials, color, size, position)
    }

    fn spawn_polygon(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        sides: usize,
        size: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::polygon(self, meshes, materials, color, sides, size, position)
    }

    fn spawn_ellipse(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
        width: f32,
        height: f32,
        position: Vec2,
    ) -> Entity {
        Geometrics::ellipse(self, meshes, materials, color, width, height, position)
    }

    fn draw(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        coordinates: Vec<Vec2>,
        color: Color,
        position: Vec2,
        scale: f32,
        filled: bool,
    ) -> Entity {
        Geometrics::draw(self, meshes, materials, coordinates, color, position, scale, filled)
    }

    fn draw_star(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        points: usize,
        outer_radius: f32,
        inner_radius: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity {
        Geometrics::draw_star(self, meshes, materials, points, outer_radius, inner_radius, color, position, filled)
    }

    fn draw_heart(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        size: f32,
        color: Color,
        position: Vec2,
        filled: bool,
    ) -> Entity {
        Geometrics::draw_heart(self, meshes, materials, size, color, position, filled)
    }
}
