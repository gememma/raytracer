use acap::{kd::KdTree, Coordinates, Euclidean, EuclideanDistance, NearestNeighbors, Proximity};

use crate::{colour::Colour, hit::Hit, ray::Ray, scene::Scene, Vertex};

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

pub enum Interaction {
    Reflected { ray: Ray, attenuation: Colour },
    Transmitted { ray: Ray, attenuation: Colour },
    Absorbed,
}

impl PhotonMap {
    pub fn build(scene: &Scene) -> Self {
        let mut tree = KdTree::new();

        // direct hits
        for light in &scene.light_list {
            for _ in 0..50000 {
                let mut p = light.generate_photon();
                let mut depth = 5;
                'l: while let Some((ph, h)) = Self::photon_trace(scene, p) {
                    let c = ph.photon.colour;
                    tree.push(ph);

                    if depth < 1 {
                        break 'l;
                    } else {
                        depth -= 1;
                    }

                    match h.material.interact(&h) {
                        Interaction::Reflected { ray, attenuation }
                        | Interaction::Transmitted { ray, attenuation } => {
                            p = Photon {
                                ray,
                                colour: attenuation * c,
                                type_: Type::Indirect,
                            }
                        }
                        Interaction::Absorbed => {
                            break 'l;
                        }
                    }
                }
            }
        }

        // indirect and shadow hits
        tree.balance();
        PhotonMap { tree }
    }

    pub fn photon_trace<'a>(scene: &'a Scene, photon: Photon) -> Option<(PhotonHit, Hit<'a>)> {
        // indirect and shadow photons
        if let Some(h) = scene.trace(&photon.ray) {
            Some((
                PhotonHit {
                    photon: photon,
                    position: h.position,
                },
                h,
            ))
        } else {
            None
        }
    }

    pub fn visualise(&self, pos: Vertex) -> (Colour, usize) {
        let nearest = self
            .tree
            .k_nearest_within(&[pos.x, pos.y, pos.z], 1000, 0.5);
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
