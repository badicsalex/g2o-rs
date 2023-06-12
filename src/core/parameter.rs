// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::ffi::c_void;

use cpp::cpp;

use crate::macros::proxy_obj_abstract;

cpp! {{
    #include "g2o/core/parameter.h"
    using namespace g2o;
}}

proxy_obj_abstract!(Parameter);

impl Parameter {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "Parameter*"] {
            delete obj;
        })
    }

    pub fn set_id(&mut self, id: i32) {
        let obj = self.obj();
        cpp!( unsafe [obj as "Parameter*", id as "int"] {
            obj->setId(id);
        })
    }

    pub fn id(&mut self) -> i32{
        let obj = self.obj();
        cpp!( unsafe [obj as "Parameter*"] -> i32 as "int" {
            return obj->id();
        })
    }
}
