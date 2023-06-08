// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use cpp::cpp;

use crate::{macros::proxy_obj, OptimizableGraph, OptimizationAlgorithm};

cpp! {{
    #include "g2o/core/sparse_optimizer.h"
    using namespace g2o;
}}

proxy_obj!(SparseOptimizer<'stored>, OptimizableGraph<'stored>);

impl<'stored> SparseOptimizer<'stored> {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "SparseOptimizer*" {
            return new SparseOptimizer();
        })
    }

    pub fn set_algorithm(&mut self, algorithm: &'stored OptimizationAlgorithm) {
        let obj = self.obj();
        let algorithm = algorithm.obj();
        cpp!( unsafe [obj as "SparseOptimizer*", algorithm as "OptimizationAlgorithm*"] {
            obj->setAlgorithm(algorithm);
        })
    }

    pub fn initialize_optimization(&mut self, level: i32) -> bool {
        let obj = self.obj();
        cpp!( unsafe [obj as "SparseOptimizer*", level as "int"] -> bool as "bool"{
            return obj->initializeOptimization(level);
        })
    }
}
