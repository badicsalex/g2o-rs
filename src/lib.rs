// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::ffi::{c_void, CString};

use cpp::cpp;

cpp! {{
    #include "g2o/core/factory.h"
    #include "g2o/core/optimization_algorithm_factory.h"
    #include "g2o/core/sparse_optimizer.h"
    using namespace g2o;

    G2O_USE_TYPE_GROUP(slam2d);
    G2O_USE_TYPE_GROUP(slam3d);
    G2O_USE_OPTIMIZATION_LIBRARY(eigen);
}}

pub struct SparseOptimizer {
    obj: *mut c_void,
}

impl SparseOptimizer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let obj = cpp!( unsafe [] -> *mut c_void as "SparseOptimizer*" {
            return new SparseOptimizer();
        });
        Self { obj }
    }

    pub fn set_algorithm(&mut self, algorithm: OptimizationAlgorithm) {
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

impl Drop for SparseOptimizer {
    fn drop(&mut self) {
        let obj = self.obj;
        cpp!( unsafe [obj as "SparseOptimizer*"] {
            delete obj;
        })
    }
}

pub struct OptimizationAlgorithm {
    obj: *mut c_void,
}

pub struct OptimizationAlgorithmProperty {
    obj: *mut c_void,
}

impl OptimizationAlgorithmProperty {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let obj = cpp!( unsafe [] -> *mut c_void as "OptimizationAlgorithmProperty*" {
            return new OptimizationAlgorithmProperty();
        });
        Self { obj }
    }
}

impl Drop for OptimizationAlgorithmProperty {
    fn drop(&mut self) {
        let obj = self.obj;
        cpp!( unsafe [obj as "OptimizationAlgorithmProperty*"] {
            delete obj;
        })
    }
}

pub struct OptimizationAlgorithmFactory;

impl OptimizationAlgorithmFactory {
    pub fn construct(
        name: &str,
        solver_property: &mut OptimizationAlgorithmProperty,
    ) -> OptimizationAlgorithm {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let solver_property = solver_property.obj;
        let obj = cpp!( unsafe [name as  "char*", solver_property as "OptimizationAlgorithmProperty*"] -> *mut c_void as "OptimizationAlgorithm*"{
            return OptimizationAlgorithmFactory::instance()->construct(name, *solver_property);
        });
        OptimizationAlgorithm { obj }
    }
}
