use acap::{kd::KdTree, Coordinates, Euclidean, EuclideanDistance, NearestNeighbors, Proximity};

use crate::{colour::Colour, ray::Ray, scene::Scene, Vertex};

pub struct PhotonMap {
    tree: KdTree<PhotonHit>,
}

pub struct PhotonHit {
    photon: Photon,
    position: Vertex,
}

pub struct Photon {
    pub ray: Ray,
    pub colour: Colour,
    pub type_: Type,
}

pub enum Type {
    Direct,
    Indirect,
    Shadow,
}

impl PhotonMap {
    pub fn build(scene: &Scene) -> Self {
        let mut tree = KdTree::new();

        // direct hits
        for light in &scene.light_list {
            for _ in 0..5000 {
                let p = light.generate_photon();
                if let Some(h) = scene.trace(&p.ray) {
                    tree.push(PhotonHit {
                        photon: p,
                        position: h.position,
                    })
                }
            }
        }

        // indirect and shadow hits
        tree.balance();
        PhotonMap { tree }
    }
    pub fn visualise(&self, pos: Vertex) -> (Colour, usize) {
        let nearest = self.tree.k_nearest_within(&[pos.x, pos.y, pos.z], 1000, 1.);
        let mut colour = Colour::default();
        let n = nearest.len();
        for hit in nearest {
            colour += hit.item.photon.colour / n as f32;
        }
        (colour, n)
    }
}

impl Coordinates for PhotonHit {
    type Value = f32;

    fn dims(&self) -> usize {
        3
    }

    fn coord(&self, i: usize) -> Self::Value {
        match i {
            0 => self.position.x,
            1 => self.position.y,
            2 => self.position.z,
            _ => unreachable!(),
        }
    }
}

impl Proximity<PhotonHit> for [f32; 3] {
    type Distance = EuclideanDistance<f32>;

    fn distance(&self, other: &PhotonHit) -> Self::Distance {
        Euclidean::new(*self).distance(&Euclidean::new(other.position.to_array()))
    }
}
