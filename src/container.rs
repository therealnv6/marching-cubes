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
        return Vec3 { x, y, z };
    }
}

pub fn vector2(x: f32, y: f32) -> Vec2 {
    #[cfg(feature = "glam")]
    {
        return glam::Vec2::new(x, y);
    }

    #[cfg(not(feature = "glam"))]
    {
        return Vec2 { x, y };
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
    pub type Vector3 = InnerVector3;
    pub type Vector2 = InnerVector2;

    #[derive(Debug, Clone, Copy)]
    pub struct InnerVector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    pub struct InnerVector2 {
        pub x: f32,
        pub y: f32,
    }
}
