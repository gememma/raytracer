use glam::{Affine3A, Mat4, Vec3A};

use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    object::Object,
    ray::Ray,
};

#[derive(Debug)]
pub struct Quadratic {
    coeffs: [f32; 10],
    material: Box<dyn Material + Send + Sync>,
}

impl Quadratic {
    pub fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
        i: f32,
        j: f32,
    ) -> Self {
        Self {
            coeffs: [a, b, c, d, e, f, g, h, i, j],
            material: Box::new(NormalShading::default()),
        }
    }
}

impl Object for Quadratic {
    fn set_material(&mut self, material: Box<dyn Material + Send + Sync>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let epsilon = 0.0000001;
        let [dx, dy, dz] = [ray.direction.x, ray.direction.y, ray.direction.z];
        let [px, py, pz] = [ray.position.x, ray.position.y, ray.position.z];

        // compute parts
        let aq = self.coeffs[0] * dx.powi(2)
            + 2. * self.coeffs[1] * dx * dy
            + 2. * self.coeffs[2] * dx * dz
            + self.coeffs[4] * dy.powi(2)
            + 2. * self.coeffs[5] * dy * dz
            + self.coeffs[7] * dz.powi(2);
        if aq > -epsilon && aq < epsilon {
            // only one intersection, ignore
            return Vec::new();
        }
        let bq = 2.
            * (self.coeffs[0] * px * dx
                + self.coeffs[1] * (px * dy + dx * py)
                + self.coeffs[2] * (px * dz + dx * pz)
                + self.coeffs[3] * dx
                + self.coeffs[4] * py * dy
                + self.coeffs[5] * (py * dz + dy * pz)
                + self.coeffs[6] * dy
                + self.coeffs[7] * pz * dz
                + self.coeffs[8] * dz);
        let cq = self.coeffs[0] * px.powi(2)
            + 2. * self.coeffs[1] * px * py
            + 2. * self.coeffs[2] * px * pz
            + 2. * self.coeffs[3] * px
            + self.coeffs[4] * py.powi(2)
            + 2. * self.coeffs[5] * py * pz
            + 2. * self.coeffs[6] * py
            + self.coeffs[7] * pz.powi(2)
            + 2. * self.coeffs[8] * pz
            + self.coeffs[9];
        let discrim = bq.powi(2) - 4. * aq * cq;
        if discrim < epsilon {
            // no intersection (no real roots)
            return Vec::new();
        }
        // two intersections exist
        let t0 = (-bq - (bq.powi(2) - 4. * aq * cq).powf(0.5)) / (2. * aq);
        let t1 = (-bq + (bq.powi(2) - 4. * aq * cq).powf(0.5)) / (2. * aq);

        let mut hits = vec![];
        for t in vec![t0, t1] {
            let hit_pos = ray.position + t * ray.direction;
            let mut normal = Vec3A::new(
                self.coeffs[0] * hit_pos.x
                    + self.coeffs[1] * hit_pos.y
                    + self.coeffs[2] * hit_pos.z
                    + self.coeffs[3],
                self.coeffs[1] * hit_pos.x
                    + self.coeffs[4] * hit_pos.y
                    + self.coeffs[5] * hit_pos.z
                    + self.coeffs[6],
                self.coeffs[2] * hit_pos.x
                    + self.coeffs[5] * hit_pos.y
                    + self.coeffs[7] * hit_pos.z
                    + self.coeffs[8],
            )
            .normalize();
            let entering = normal.dot(ray.direction) < 0.;
            if !entering {
                normal = -normal
            }
            let h = Hit {
                t,
                entering,
                object_hit: self,
                material: &*self.material,
                position: hit_pos,
                normal,
                incident: ray.clone(),
            };
            hits.push(h)
        }
        hits
    }

    fn apply_transform(&mut self, t: Affine3A) {
        // TODO: fix inverted translation
        let t_matrix = Mat4::from(t);
        let q_matrix = Mat4::from_cols_array(&[
            self.coeffs[0],
            self.coeffs[1],
            self.coeffs[2],
            self.coeffs[3],
            self.coeffs[1],
            self.coeffs[4],
            self.coeffs[5],
            self.coeffs[6],
            self.coeffs[2],
            self.coeffs[5],
            self.coeffs[7],
            self.coeffs[8],
            self.coeffs[3],
            self.coeffs[6],
            self.coeffs[8],
            self.coeffs[9],
        ]);
        let q_prime = t_matrix.transpose() * q_matrix * t_matrix;

        // extract coefficients from answer
        [
            self.coeffs[0],
            self.coeffs[1],
            self.coeffs[2],
            self.coeffs[3],
        ] = [
            q_prime.x_axis.x,
            q_prime.x_axis.y,
            q_prime.x_axis.z,
            q_prime.x_axis.w,
        ];
        [self.coeffs[4], self.coeffs[5], self.coeffs[6]] =
            [q_prime.y_axis.y, q_prime.y_axis.z, q_prime.y_axis.w];
        [self.coeffs[7], self.coeffs[8], self.coeffs[9]] =
            [q_prime.z_axis.z, q_prime.z_axis.w, q_prime.w_axis.w];
    }
}
