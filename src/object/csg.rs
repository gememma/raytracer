use glam::Affine3A;

use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    object::Object,
    ray::Ray,
};

#[derive(Debug)]
pub struct Csg {
    objects: CsgNode,
    material: Box<dyn Material>,
}

#[derive(Debug)]
pub enum CsgNode {
    Branch {
        child1: Option<Box<CsgNode>>,
        child2: Option<Box<CsgNode>>,
        operation: Op,
    },
    Leaf {
        object: Box<dyn Object>,
    },
}

#[derive(Debug)]
pub enum Op {
    Union,
    Intersection,
    Negation,
}

impl Csg {
    pub fn new(objects: CsgNode) -> Self {
        Csg {
            objects,
            material: Box::new(NormalShading::default()),
        }
    }
}

impl Object for Csg {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        todo!()
    }

    fn apply_transform(&mut self, t: Affine3A) {
        todo!()
    }
}

impl CsgNode {
    pub fn new_branch(
        child1: Option<Box<CsgNode>>,
        child2: Option<Box<CsgNode>>,
        operation: Op,
    ) -> Self {
        CsgNode::Branch {
            child1,
            child2,
            operation,
        }
    }

    pub fn new_leaf(object: Box<dyn Object>) -> Self {
        CsgNode::Leaf { object }
    }
}
