use glam::Vec3A;
use rand::random;

use crate::{colour::Colour, hit::Hit, material::Material, photonmap::Interaction, scene::Scene};

#[derive(Debug)]
pub struct Compound {
    pub material_list: Vec<Box<dyn Material + Send + Sync>>,
}

impl Compound {
    pub fn new() -> Self {
        Compound {
            material_list: vec![],
        }
    }
    pub fn add_material<M: Material + Send + Sync + 'static>(&mut self, material: M) {
        self.material_list.push(Box::new(material))
    }
}
impl Material for Compound {
    fn compute(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour {
        let n = self.material_list.len();
        let mut c = Colour::from_rgb(0., 0., 0.);
        for m in &self.material_list {
            c += m.compute(viewer, hit, recurse, scene) / n as f32;
        }
        c
    }

    fn interact(&self, hit: &Hit) -> Interaction {
        let n = self.material_list.len();
        let i = (random::<f32>() * n as f32) as usize;
        self.material_list[i].interact(hit)
    }
}
