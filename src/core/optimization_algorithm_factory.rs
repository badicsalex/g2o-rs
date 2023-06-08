// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::ffi::{c_void, CString};

use cpp::cpp;

use crate::macros::{proxy_obj, proxy_obj_abstract};

cpp! {{
    #include "g2o/core/optimization_algorithm.h"
    #include "g2o/core/optimization_algorithm_factory.h"
    using namespace g2o;

    G2O_USE_OPTIMIZATION_LIBRARY(eigen);
}}

proxy_obj_abstract!(OptimizationAlgorithm);
impl OptimizationAlgorithm {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "OptimizationAlgorithm*"] {
            delete obj;
        })
    }
}

proxy_obj!(OptimizationAlgorithmProperty);

impl OptimizationAlgorithmProperty {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "OptimizationAlgorithmProperty*" {
            return new OptimizationAlgorithmProperty();
        })
    }

    fn destruct(obj: *mut c_void) {
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
        let solver_property = solver_property.obj();
        let obj = cpp!( unsafe [name as  "char*", solver_property as "OptimizationAlgorithmProperty*"] -> *mut c_void as "OptimizationAlgorithm*"{
            return OptimizationAlgorithmFactory::instance()->construct(name, *solver_property);
        });
        OptimizationAlgorithm::new_from(obj)
    }
}
