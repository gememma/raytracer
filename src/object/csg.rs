use std::{cmp::Ordering, collections::VecDeque};

use glam::Affine3A;

use crate::{
    hit::Hit,
    material::Material,
    object::{
        plane::Plane, polymesh::PolyMesh, quadratic::Quadratic, sphere::Sphere, triangle::Triangle,
        Object,
    },
    ray::Ray,
};

#[derive(Debug)]
pub enum Csg {
    Branch {
        child1: Box<Csg>,
        child2: Box<Csg>,
        operation: Op,
    },
    Leaf {
        object: Box<dyn Object + Send + Sync + 'static>,
    },
}

#[derive(Debug)]
pub enum Op {
    Union,
    Intersection,
    Difference,
}

impl Csg {
    pub fn new_branch<T1, T2>(child1: T1, child2: T2, operation: Op) -> Self
    where
        T1: Into<Csg>,
        T2: Into<Csg>,
    {
        Csg::Branch {
            child1: Box::new(child1.into()),
            child2: Box::new(child2.into()),
            operation,
        }
    }

    pub fn new_leaf(object: Box<dyn Object + Send + Sync + 'static>) -> Self {
        Csg::Leaf { object }
    }
}

pub trait FromCsg: Object {}
impl FromCsg for Plane {}
impl FromCsg for PolyMesh {}
impl FromCsg for Quadratic {}
impl FromCsg for Sphere {}
impl FromCsg for Triangle {}

impl<T> From<T> for Csg
where
    T: FromCsg + Send + Sync + 'static,
{
    fn from(object: T) -> Self {
        Self::new_leaf(Box::new(object))
    }
}

impl Object for Csg {
    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        match self {
            Csg::Branch {
                child1,
                child2,
                operation,
            } => {
                let int_a = VecDeque::from(child1.intersection(ray));
                let int_b = VecDeque::from(child2.intersection(ray));

                // perform operation to merge intersections
                match operation {
                    Op::Union => Op::union(int_a, int_b),
                    Op::Intersection => Op::intersection(int_a, int_b),
                    Op::Difference => Op::difference(int_a, int_b),
                }
            }
            Csg::Leaf { object } => object.intersection(ray),
        }
    }

    fn apply_transform(&mut self, t: Affine3A) {
        match self {
            Csg::Branch { child1, child2, .. } => {
                child1.apply_transform(t);
                child2.apply_transform(t);
            }
            Csg::Leaf { object } => object.apply_transform(t),
        }
    }
}

impl Op {
    pub fn union<'a>(mut int_a: VecDeque<Hit<'a>>, mut int_b: VecDeque<Hit<'a>>) -> Vec<Hit<'a>> {
        use Ordering::*;
        let mut int_result = Vec::new();
        let mut inside_a = false;
        let mut inside_b = false;
        while !(int_a.is_empty() || int_b.is_empty()) {
            match (
                inside_a,
                inside_b,
                int_a[0].t.partial_cmp(&int_b[0].t).unwrap(),
            ) {
                (false, false, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (false, false, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (false, false, Equal) => keep(&mut int_a, &mut int_result, &mut inside_a),

                (true, false, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (true, false, Greater) => discard(&mut int_b, &mut inside_b),
                (true, false, Equal) => discard(&mut int_a, &mut inside_a),

                (true, true, Less) => discard(&mut int_a, &mut inside_a),
                (true, true, Greater) => discard(&mut int_b, &mut inside_b),
                (true, true, Equal) => discard(&mut int_a, &mut inside_a),

                (false, true, Less) => discard(&mut int_a, &mut inside_a),
                (false, true, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (false, true, Equal) => discard(&mut int_b, &mut inside_b),
            }
        }
        while !int_a.is_empty() {
            keep(&mut int_a, &mut int_result, &mut inside_a);
        }
        while !int_b.is_empty() {
            keep(&mut int_b, &mut int_result, &mut inside_b);
        }
        int_result
    }
    pub fn intersection<'a>(
        mut int_a: VecDeque<Hit<'a>>,
        mut int_b: VecDeque<Hit<'a>>,
    ) -> Vec<Hit<'a>> {
        use Ordering::*;
        let mut int_result = Vec::new();
        let mut inside_a = false;
        let mut inside_b = false;
        while !(int_a.is_empty() || int_b.is_empty()) {
            match (
                inside_a,
                inside_b,
                int_a[0].t.partial_cmp(&int_b[0].t).unwrap(),
            ) {
                (false, false, Less) => discard(&mut int_a, &mut inside_a),
                (false, false, Greater) => discard(&mut int_b, &mut inside_b),
                (false, false, Equal) => keep(&mut int_a, &mut int_result, &mut inside_a),

                (true, false, Less) => discard(&mut int_a, &mut inside_a),
                (true, false, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (true, false, Equal) => discard(&mut int_a, &mut inside_a),

                (true, true, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (true, true, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (true, true, Equal) => keep(&mut int_a, &mut int_result, &mut inside_a),

                (false, true, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (false, true, Greater) => discard(&mut int_b, &mut inside_b),
                (false, true, Equal) => discard(&mut int_b, &mut inside_b),
            }
        }
        drop(int_a);
        drop(int_b);
        int_result
    }
    pub fn difference<'a>(
        mut int_a: VecDeque<Hit<'a>>,
        mut int_b: VecDeque<Hit<'a>>,
    ) -> Vec<Hit<'a>> {
        use Ordering::*;
        let mut int_result = Vec::new();
        let mut inside_a = false;
        let mut inside_b = false;
        while !(int_a.is_empty() || int_b.is_empty()) {
            match (
                inside_a,
                inside_b,
                int_a[0].t.partial_cmp(&int_b[0].t).unwrap(),
            ) {
                (false, false, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (false, false, Greater) => discard(&mut int_b, &mut inside_b),
                (false, false, Equal) => discard(&mut int_b, &mut inside_b),

                (true, false, Less) => keep(&mut int_a, &mut int_result, &mut inside_a),
                (true, false, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (true, false, Equal) => keep(&mut int_a, &mut int_result, &mut inside_a),

                (true, true, Less) => discard(&mut int_a, &mut inside_a),
                (true, true, Greater) => keep(&mut int_b, &mut int_result, &mut inside_b),
                (true, true, Equal) => discard(&mut int_a, &mut inside_a),

                (false, true, Less) => discard(&mut int_a, &mut inside_a),
                (false, true, Greater) => discard(&mut int_b, &mut inside_b),
                (false, true, Equal) => discard(&mut int_b, &mut inside_b),
            }
        }
        while !int_a.is_empty() {
            keep(&mut int_a, &mut int_result, &mut inside_a);
        }
        drop(int_b);
        int_result
    }
}

fn keep<'a>(source: &mut VecDeque<Hit<'a>>, dest: &mut Vec<Hit<'a>>, inside: &mut bool) {
    // keep the remaining intersection points in source
    *inside = !*inside;
    let val = source.pop_front().unwrap();
    dest.push(val);
}

fn discard(source: &mut VecDeque<Hit>, inside: &mut bool) {
    *inside = !*inside;
    // pop and discard unneeded intersection points with _
    let _ = source.pop_front().unwrap();
}
