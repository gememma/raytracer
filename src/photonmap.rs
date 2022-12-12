use acap::{kd::KdTree, Coordinates, Euclidean, EuclideanDistance, NearestNeighbors, Proximity};

use crate::{colour::Colour, hit::Hit, ray::Ray, scene::Scene, Vertex};

pub struct PhotonMap<'a> {
    tree: KdTree<PhotonHit<'a>>,
}

pub struct PhotonHit<'a> {
    photon: Photon,
    hit: Hit<'a>,
}

pub struct Photon {
    pub ray: Ray,
    pub colour: Colour,
    pub type_: Type,
}

#[derive(Eq, PartialEq)]
pub enum Type {
    Direct,
    Indirect,
    Shadow,
}

#[derive(Debug)]
pub enum Interaction {
    Reflected { ray: Ray, attenuation: Colour },
    Transmitted { ray: Ray, attenuation: Colour },
    Absorbed,
}

impl<'a> PhotonMap<'a> {
    pub fn build(scene: &'a Scene) -> Self {
        let mut tree = KdTree::new();
        let photon_number = 50000;

        for light in &scene.light_list {
            for _ in 0..photon_number {
                let mut p = light.generate_photon();
                let mut depth = 5;
                'l: while let Some(ph) = Self::photon_trace(scene, p) {
                    let c = ph.photon.colour;
                    if ph.photon.type_ == Type::Direct {
                        // fire shadow photon
                        if let Some(ph) = Self::shadow_photon_trace(
                            scene,
                            Photon {
                                ray: Ray::new(
                                    ph.hit.position + 0.0001 * ph.hit.incident.direction,
                                    ph.hit.incident.direction,
                                ),
                                colour: Colour::from_rgb(0., 0., 0.),
                                type_: Type::Shadow,
                            },
                        ) {
                            tree.push(ph);
                        }
                    }

                    let hit = ph.hit.clone();
                    tree.push(ph);

                    if depth < 1 {
                        break 'l;
                    } else {
                        depth -= 1;
                    }

                    match hit.material.interact(&hit) {
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

        tree.balance();
        PhotonMap { tree }
    }

    pub fn build_caustics(scene: &'a Scene) -> Self {
        // build a photon map for caustics by firing lots of photons at metals and glass
        let mut tree = KdTree::new();
        let photon_number = 10000000;

        for light in &scene.light_list {
            for _ in 0..photon_number {
                let mut p = light.generate_photon();
                let mut depth = 5;

                'l: while let Some(ph) = Self::photon_trace(scene, p) {
                    let c = ph.photon.colour;
                    let hit = ph.hit.clone();

                    // discard absorbed direct hits for caustics
                    if ph.photon.type_ == Type::Direct {
                        match ph.hit.material.interact(&ph.hit) {
                            Interaction::Transmitted { .. } => {
                                tree.push(ph);
                            }
                            Interaction::Reflected { .. } | Interaction::Absorbed => {
                                break 'l;
                            }
                        }
                    } else {
                        tree.push(ph);
                    }

                    if depth < 1 {
                        break 'l;
                    } else {
                        depth -= 1;
                    }

                    match hit.material.interact(&hit) {
                        Interaction::Transmitted { ray, attenuation } => {
                            p = Photon {
                                ray,
                                colour: attenuation * c,
                                type_: Type::Indirect,
                            }
                        }
                        Interaction::Reflected { .. } | Interaction::Absorbed => {
                            break 'l;
                        }
                    }
                }
            }
        }

        tree.balance();
        PhotonMap { tree }
    }

    pub fn photon_trace(scene: &'a Scene, photon: Photon) -> Option<PhotonHit<'a>> {
        // indirect photons
        if let Some(h) = scene.trace(&photon.ray) {
            Some(PhotonHit {
                photon: photon,
                hit: h,
            })
        } else {
            None
        }
    }

    pub fn shadow_photon_trace(scene: &'a Scene, photon: Photon) -> Option<PhotonHit<'a>> {
        // shadow photons
        if let Some(h) = scene.trace(&photon.ray) {
            if h.entering {
                Some(PhotonHit {
                    photon: photon,
                    hit: h,
                })
            } else {
                PhotonMap::shadow_photon_trace(
                    scene,
                    Photon {
                        ray: Ray::new(
                            h.position + 0.0001 * h.incident.direction,
                            h.incident.direction,
                        ),
                        colour: photon.colour,
                        type_: photon.type_,
                    },
                )
            }
        } else {
            None
        }
    }

    pub fn visualise(&self, pos: Vertex) -> (Colour, usize) {
        let neighbours = 1000;
        let radius = 0.3;
        let nearest = self
            .tree
            .k_nearest_within(&[pos.x, pos.y, pos.z], neighbours, radius);
        let mut colour = Colour::default();
        let n = nearest.len();
        for hit in nearest {
            colour += hit.item.photon.colour / n as f32;
        }
        (colour, n)
    }

    pub fn visualise_caustics(&self, pos: Vertex) -> (Colour, usize) {
        let neighbours = 10000;
        let radius = 0.4;
        let nearest = self
            .tree
            .k_nearest_within(&[pos.x, pos.y, pos.z], neighbours, radius);
        let mut colour = Colour::default();
        let mut n = 0;
        for hit in nearest {
            match hit.item.hit.material.interact(&hit.item.hit) {
                Interaction::Reflected { .. } | Interaction::Transmitted { .. } => {}
                Interaction::Absorbed => {
                    n += 1;
                    colour += hit.item.photon.colour;
                }
            }
        }
        (colour / n as f32, n)
    }

    pub fn get_radiance_est(&self, _h: Hit) -> Colour {
        // used during rendering (second pass)
        unimplemented!()
    }
}

impl<'a> Coordinates for PhotonHit<'a> {
    type Value = f32;

    fn dims(&self) -> usize {
        3
    }

    fn coord(&self, i: usize) -> Self::Value {
        match i {
            0 => self.hit.position.x,
            1 => self.hit.position.y,
            2 => self.hit.position.z,
            _ => unreachable!(),
        }
    }
}

impl<'a> Proximity<PhotonHit<'a>> for [f32; 3] {
    type Distance = EuclideanDistance<f32>;

    fn distance(&self, other: &PhotonHit) -> Self::Distance {
        Euclidean::new(*self).distance(&Euclidean::new(other.hit.position.to_array()))
    }
}
