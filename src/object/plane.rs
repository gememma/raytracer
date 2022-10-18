use super::Object;
use crate::{
    hit::Hit,
    material::{falsecolour::FalseColour, Material},
    ray::Ray,
    Vertex,
};
use glam::{Affine3A, Vec3A};

#[derive(Debug)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            material: Box::new(FalseColour::default()),
        }
    }
}

impl Object for Plane {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let u =
            self.a * ray.position.x + self.b * ray.position.y + self.c * ray.position.z + self.d;
        let v = self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z;

        if v == 0. {
            // parallel to plane
            if u < 0. {
                // is inside
                let hit1 = Hit {
                    t: -10000000000., // infinity
                    entering: true,
                    object_hit: self,
                    position: Vertex::default(),
                    normal: Vec3A::default(),
                };
                let hit2 = Hit {
                    t: 10000000000., // infinity
                    entering: false,
                    object_hit: self,
                    position: Vertex::default(),
                    normal: Vec3A::default(),
                };
                return vec![hit1, hit2];
            } else {
                // is outside
                return vec![];
            }
        }

        if v > 0. {
            let t = u / -v;
            let hit1 = Hit {
                t: -10000000000., // infinity
                entering: true,
                object_hit: self,
                position: Vertex::default(),
                normal: Vec3A::default(),
            };
            let mut hit2 = Hit {
                t, // infinity (this is the C++ comment, but I'm pretty sure this is not infinity)
                entering: false,
                object_hit: self,
                position: ray.position + t * ray.direction,
                normal: Vec3A::new(self.a, self.b, self.c),
            };
            if hit2.normal.dot(ray.direction) > 0. {
                hit2.normal *= -1.;
            }
            return vec![hit1, hit2];
        } else {
            let t = u / -v;
            let mut hit1 = Hit {
                t, // infinity (this is the C++ comment, but I'm pretty sure this is not infinity)
                entering: true,
                object_hit: self,
                position: ray.position + t * ray.direction,
                normal: Vec3A::new(self.a, self.b, self.c),
            };
            if hit1.normal.dot(ray.direction) > 0. {
                hit1.normal *= -1.;
            }
            let hit2 = Hit {
                t: 10000000000., // infinity
                entering: false,
                object_hit: self,
                position: Vertex::default(),
                normal: Vec3A::default(),
            };
            return vec![hit1, hit2];
        }
    }

    fn apply_transform(&mut self, _transform: Affine3A) {
        todo!()
    }
}
