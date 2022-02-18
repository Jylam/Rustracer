#![allow(dead_code)]

use crate::vec3::Vec3;
use std::sync::atomic::{AtomicU64, Ordering};

static RAY_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir:  Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray{orig: orig, dir: dir}
    }

    // Accessors
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Operations
    pub fn at(&self, t: f64) -> Vec3 {
        RAY_COUNT.store(RAY_COUNT.load(Ordering::Relaxed)+1, Ordering::Relaxed);

        self.orig + (self.dir*t)
    }
    pub fn get_count() -> u64 {
        RAY_COUNT.load(Ordering::Relaxed)
    }
    pub fn reset_count() {
        RAY_COUNT.store(0, Ordering::Relaxed);
    }

}
