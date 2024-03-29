use crate::{
    colour::Colour,
    hit::Hit,
    light::Light,
    object::Object,
    photonmap::{Interaction, PhotonMap},
    ray::Ray,
    Vertex,
};

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
        // find hits along the given ray with the scene
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
        // select the front-most hit (the hit with the lowest t value) along the ray
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
    pub fn shadow_trace(&self, ray: &Ray, limit: f32) -> bool {
        for object in &self.object_list {
            if let Some(hit) = Self::select_first(object.intersection(ray)) {
                if hit.t > 0.00001 && hit.t < limit {
                    match hit.material.interact(&hit) {
                        Interaction::Transmitted { .. } => continue,
                        _ => return true,
                    }
                }
            }
        }
        return false;
    }

    pub fn raytrace(
        &self,
        ray: Ray,
        recurse: usize,
        viewer: Vertex,
        pmap: &PhotonMap,
    ) -> (Colour, f32) {
        if recurse == 0 {
            return (
                // hit recursion depth, return background colour
                Colour::from_rgba(0., 0., 0., 1.),
                0.,
            );
        }
        let best_hit = self.trace(&ray);

        if let Some(best) = best_hit {
            let viewer = (viewer - best.position).normalize();
            (
                best.material.compute(viewer, &best, recurse, self, pmap),
                best.t,
            )
        } else {
            // background colour
            (Colour::from_rgba(0., 0., 0., 1.), 0.)
        }
    }

    pub fn add_object<O: Object + Send + Sync + 'static>(&mut self, object: O) {
        self.object_list.push(Box::new(object));
    }

    pub fn add_light<L: Light + Send + Sync + 'static>(&mut self, light: L) {
        self.light_list.push(Box::new(light));
    }
}
