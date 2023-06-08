// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
};

use cpp::cpp;

cpp! {{
    #include "g2o/core/optimizable_graph.h"
    using namespace g2o;
}}

pub struct OptimizableGraph<'stored> {
    obj: *mut c_void,
    stored_stuff_tag: PhantomData<&'stored ()>,
}

impl<'stored> OptimizableGraph<'stored> {
    pub(crate) fn new_from_child(obj: *mut c_void) -> Self {
        Self {
            obj,
            stored_stuff_tag: PhantomData,
        }
    }
    pub(crate) fn obj(&mut self) -> *mut c_void {
        self.obj
    }
    pub fn load(&mut self, filename: &str) -> bool {
        let obj = self.obj;
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph*", filename as "char*"] -> bool as "bool"{
            return obj->load(filename);
        })
    }

    pub fn save(&mut self, filename: &str) -> bool {
        let obj = self.obj;
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph*", filename as "char*"] -> bool as "bool"{
            return obj->save(filename);
        })
    }
}
impl<'stored> Drop for OptimizableGraph<'stored> {
    fn drop(&mut self) {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph*"] {
            delete obj;
        })
    }
}
