use crate::{colour::Colour, hit::Hit, light::Light, object::Object, ray::Ray, Vertex};

#[derive(Debug)]
pub struct Scene {
    pub object_list: Vec<Box<dyn Object + Send + Sync>>,
    pub light_list: Vec<Box<dyn Light + Send + Sync>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            object_list: Vec::default(),
            light_list: Vec::default(),
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Hit> {
        let mut best_hit = None;

        for object in &self.object_list {
            if let Some(new_hit) = Self::select_first(object.intersection(ray)) {
                match best_hit {
                    None => {
                        best_hit = Some(new_hit);
                    }
                    Some(old_best) if new_hit.t < old_best.t => {
                        best_hit = Some(new_hit);
                    }
                    _ => {}
                }
            }
        }

        best_hit
    }

    pub fn select_first(hits: Vec<Hit>) -> Option<Hit> {
        let mut result: Option<Hit> = None;
        for hit in hits {
            if let Some(h) = &result {
                if hit.t < h.t && hit.t >= 0. {
                    result = Some(hit);
                }
            } else if hit.t >= 0. {
                result = Some(hit);
            }
        }
        result
    }
}

impl Scene {
    pub fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool {
        for object in &self.object_list {
            if let Some(hit) = Self::select_first(object.intersection(ray)) {
                if hit.t > 0.0000001 && hit.t < limit {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn raytrace(&self, ray: Ray, recurse: usize, viewer: Vertex) -> (Colour, f32) {
        // a default colour if we hit nothing.
        let mut colour = Colour::from_rgba(0., 0., 0., 0.);
        let mut depth = 0.;

        // first step, find the closest primitive
        let best_hit = self.trace(&ray);

        // if we found a primitive then compute the colour we should see
        if let Some(best) = best_hit {
            let viewer = (viewer - best.position).normalize();
            depth = best.t;
            colour += best
                .object_hit
                .material()
                .compute_once(viewer, &best, recurse);

            // next, compute the light contribution for each light in the scene.
            for light in &self.light_list {
                // ldir is direction towards the light
                let (ldir, mut lit) = light.get_direction(best.position);

                if ldir.dot(best.normal) < 0. {
                    // Light is facing wrong way.
                    lit = false;
                }

                if lit {
                    lit = !self.shadowtrace(
                        &Ray::new(best.position + 0.0001 * ldir, ldir),
                        f32::INFINITY,
                    );
                }

                if lit {
                    let intensity = light.get_intensity(best.position);
                    colour += intensity
                        * best
                            .object_hit
                            .material()
                            .compute_per_light(viewer, &best, ldir);
                }
            }
        } else {
            // background colour
            colour.r = 0.;
            colour.g = 0.;
            colour.b = 0.;
            colour.a = 1.;
        }

        (colour, depth)
    }

    pub fn add_object<O: Object + Send + Sync + 'static>(&mut self, object: O) {
        self.object_list.push(Box::new(object));
    }

    pub fn add_light<L: Light + Send + Sync + 'static>(&mut self, light: L) {
        self.light_list.push(Box::new(light));
    }
}
