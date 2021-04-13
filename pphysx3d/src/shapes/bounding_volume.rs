use core::f32;

use kiss3d::nalgebra::{self as na, Point, Point3, Vector2, Vector3};

pub trait BoundingVolume {
    fn interects(&self, other: &Self) -> bool;
    fn contains(&self, other: &Self) -> bool;
}

pub struct AABB {
    mins: Point3<f32>,
    maxs: Point3<f32>,
}

impl AABB {
    pub fn new(mins: Point3<f32>, maxs: Point3<f32>) -> AABB {
        AABB { mins, maxs }
    }
}

pub struct BoundingSphere {
    center: Point3<f32>,
    radius: f32,
}

impl BoundingSphere {
    pub fn new(radius: f32, center: Point3<f32>) -> BoundingSphere {
        BoundingSphere { center, radius }
    }
}

impl BoundingVolume for BoundingSphere {
    fn contains(&self, other: &Self) -> bool {
        let difference: Vector3<f32> = other.center - self.center;

        difference.norm() + other.radius <= self.radius
    }
    fn interects(&self, other: &Self) -> bool {
        let difference: Vector3<f32> = other.center - self.center;
        let distance_squared = difference.norm_squared();
        let radius_summed = self.radius + other.radius;

        distance_squared <= radius_summed * radius_summed
    }
}
