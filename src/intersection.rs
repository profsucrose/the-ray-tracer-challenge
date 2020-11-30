use crate::{tuples::*, sphere::*};
use std::cmp::{Ordering, Ord};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub object: Sphere,
    pub t: f32
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        fequals(self.t, other.t)
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if fequals(self.t, other.t) {
            Ordering::Equal
        } else {
            if self.t > other.t {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections.iter()
        .fold(None, |acc, el| {
            match el.t {
                _ if el.t < 0.0 => acc,
                _ => {
                    if let Some(acc) = acc {
                        if el.t < acc.t {
                            Some(*el)
                        } else {
                            Some(acc)
                        }
                    } else {
                        Some(*el)
                    }
                }
            }
        })
}