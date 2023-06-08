// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
};

use cpp::cpp;

use crate::OptimizationAlgorithm;

cpp! {{
    #include "g2o/core/sparse_optimizer.h"
    using namespace g2o;
}}

pub struct SparseOptimizer<'stored> {
    obj: *mut c_void,
    stored_stuff_tag: PhantomData<&'stored ()>,
}

impl<'stored> SparseOptimizer<'stored> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let obj = cpp!( unsafe [] -> *mut c_void as "SparseOptimizer*" {
            return new SparseOptimizer();
        });
        Self {
            obj,
            stored_stuff_tag: PhantomData,
        }
    }

    pub fn set_algorithm(&mut self, algorithm: &'stored OptimizationAlgorithm) {
        let obj = self.obj;
        let algorithm = algorithm.obj;
        cpp!( unsafe [obj as "SparseOptimizer*", algorithm as "OptimizationAlgorithm*"] {
            obj->setAlgorithm(algorithm);
        })
    }

    pub fn initialize_optimization(&mut self, level: i32) -> bool {
        let obj = self.obj;
        cpp!( unsafe [obj as "SparseOptimizer*", level as "int"] -> bool as "bool"{
            return obj->initializeOptimization(level);
        })
    }

    pub fn optimize(&mut self, iterations: i32, online: bool) -> i32 {
        let obj = self.obj;
        cpp!( unsafe [obj as "SparseOptimizer*", iterations as "int", online as "bool"] -> i32 as "int"{
            return obj->optimize(iterations, online);
        })
    }

    // TODO: this is actually a method of the parent class, so we should
    //       instead implement it as such, and also use Deref.
    pub fn load(&mut self, filename: &str) -> bool {
        let obj = self.obj;
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "SparseOptimizer*", filename as "char*"] -> bool as "bool"{
            return obj->load(filename);
        })
    }

    // TODO: this is actually a method of the parent class, so we should
    //       instead implement it as such, and also use Deref.
    pub fn save(&mut self, filename: &str) -> bool {
        let obj = self.obj;
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "SparseOptimizer*", filename as "char*"] -> bool as "bool"{
            return obj->save(filename);
        })
    }
}

impl<'stored> Drop for SparseOptimizer<'stored> {
    fn drop(&mut self) {
        let obj = self.obj;
        cpp!( unsafe [obj as "SparseOptimizer*"] {
            delete obj;
        })
    }
}
