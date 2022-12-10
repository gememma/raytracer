use glam::{Affine3A, Vec3A};

use super::Object;
use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    ray::Ray,
    Vertex,
};

#[derive(Debug)]
pub struct Sphere {
    center: Vertex,
    radius: f32,
    material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self {
            center,
            radius,
            material: Box::new(NormalShading::default()),
        }
    }
}

impl Object for Sphere {
    fn set_material(&mut self, material: Box<dyn Material + Send + Sync>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        // offset ray by sphere position
        let ro = Vec3A::new(
            ray.position.x - self.center.x,
            ray.position.y - self.center.y,
            ray.position.z - self.center.z,
        );

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(ro);
        let c = ro.dot(ro) - self.radius.powi(2);

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            Vec::new()
        } else {
            let ds = disc.sqrt();

            let t0 = (-b - ds) / 2.;
            let t1 = (-b + ds) / 2.;

            let position0 = ray.position + t0 * ray.direction;
            let mut hit0 = Hit {
                t: t0,
                entering: true,
                object_hit: self,
                material: &*self.material,
                position: position0,
                normal: (position0 - self.center).normalize(),
                incident: ray.clone(),
            };

            if hit0.normal.dot(ray.direction) > 0. {
                hit0.normal *= -1.;
            }

            let position1 = ray.position + t1 * ray.direction;
            let mut hit1 = Hit {
                t: t1,
                entering: false,
                object_hit: self,
                material: &*self.material,
                position: position1,
                normal: (position1 - self.center).normalize(),
                incident: ray.clone(),
            };

            if hit1.normal.dot(ray.direction) > 0. {
                hit1.normal *= -1.;
            }

            vec![hit0, hit1]
        }
    }

    fn apply_transform(&mut self, t: Affine3A) {
        let new = t.transform_point3a(self.center);
        self.center = new;
    }
}
