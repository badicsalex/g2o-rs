// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::ffi::{c_void, CString};

use cpp::cpp;

cpp! {{
    #include "g2o/core/optimization_algorithm_factory.h"
    using namespace g2o;
}}

pub struct OptimizationAlgorithm {
    pub(crate) obj: *mut c_void,
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
