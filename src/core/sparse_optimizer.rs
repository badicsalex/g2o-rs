// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use cpp::cpp;

use crate::{OptimizableGraph, OptimizationAlgorithm};

cpp! {{
    #include "g2o/core/sparse_optimizer.h"
    using namespace g2o;
}}

pub struct SparseOptimizer<'stored> {
    parent: OptimizableGraph<'stored>,
}

impl<'stored> SparseOptimizer<'stored> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let obj = cpp!( unsafe [] -> *mut c_void as "SparseOptimizer*" {
            return new SparseOptimizer();
        });
        Self {
            parent: OptimizableGraph::new_from_child(obj),
        }
    }

    fn obj(&mut self) -> *mut c_void {
        // NOTE: this is only correct because on C++ side this
        //       class has only one parent. The correct thing to
        //       do would be a dynamic_cast or similar.
        self.parent.obj()
    }

    pub fn set_algorithm(&mut self, algorithm: &'stored OptimizationAlgorithm) {
        let obj = self.obj();
        let algorithm = algorithm.obj;
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

    pub fn optimize(&mut self, iterations: i32, online: bool) -> i32 {
        let obj = self.obj();
        cpp!( unsafe [obj as "SparseOptimizer*", iterations as "int", online as "bool"] -> i32 as "int"{
            return obj->optimize(iterations, online);
        })
    }
}

impl<'stored> Deref for SparseOptimizer<'stored> {
    type Target = OptimizableGraph<'stored>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl<'stored> DerefMut for SparseOptimizer<'stored> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}
