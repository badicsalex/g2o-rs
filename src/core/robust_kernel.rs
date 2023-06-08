// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::ffi::{c_void, CString};

use cpp::cpp;

use crate::macros::proxy_obj_abstract;

cpp! {{
    #include "g2o/core/robust_kernel.h"
    #include "g2o/core/robust_kernel_factory.h"
    using namespace g2o;

    G2O_USE_ROBUST_KERNEL(RobustKernelHuber)
}}

proxy_obj_abstract!(RobustKernel);
impl RobustKernel {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "RobustKernel*"] {
            delete obj;
        })
    }
}

pub struct RobustKernelFactory;

impl RobustKernelFactory {
    pub fn construct(name: &str) -> RobustKernel {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let obj = cpp!( unsafe [name as  "char*"] -> *mut c_void as "RobustKernel*"{
            return RobustKernelFactory::instance()->construct(name);
        });
        RobustKernel::new_from(obj)
    }
}
