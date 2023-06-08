// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use cpp::cpp;

use crate::{macros::proxy_obj, OptimizableGraphVertex};

cpp! {{
    #include "g2o/types/slam3d/types_slam3d.h"
    using namespace g2o;
}}

proxy_obj!(VertexSE3, OptimizableGraphVertex);

impl VertexSE3 {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "VertexSE3*" {
            return new VertexSE3();
        })
    }
}

proxy_obj!(VertexPointXYZ, OptimizableGraphVertex);

impl VertexPointXYZ {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "VertexPointXYZ*" {
            return new VertexPointXYZ();
        })
    }
}
