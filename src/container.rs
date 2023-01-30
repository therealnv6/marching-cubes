#[cfg(not(feature = "glam"))]
use inner::*;

#[cfg(feature = "glam")]
use glam::*;

pub type Vec3 = Vector3;
pub type Vec2 = Vector2;

pub fn vector3(x: f32, y: f32, z: f32) -> Vec3 {
    #[cfg(feature = "glam")]
    {
        return glam::Vec3::new(x, y);
    }

    #[cfg(not(feature = "glam"))]
    {
        return [x, y, z];
    }
}

pub fn vector2(x: f32, y: f32) -> Vec2 {
    #[cfg(feature = "glam")]
    {
        return glam::Vec2::new(x, y);
    }

    #[cfg(not(feature = "glam"))]
    {
        return [x, y];
    }
}

pub fn empty_vec3() -> Vector3 {
    vector3(0.0, 0.0, 0.0)
}

pub fn empty_vec2() -> Vector2 {
    vector2(0.0, 0.0)
}

#[cfg(feature = "glam")]
pub mod glam {
    pub type Vector3 = glam::Vec3;
    pub type Vector2 = glam::Vec2;
}

pub mod inner {
    pub type Vector3 = [f32; 3];
    pub type Vector2 = [f32; 2];
}

pub trait TriangleShape {
    fn get_points() -> (f32, f32, f32);
}
