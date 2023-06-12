// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use cpp::cpp;

use crate::{macros::proxy_obj, OptimizableGraphEdge, OptimizableGraphVertex, Parameter};

cpp! {{
    #include "g2o/types/slam3d/types_slam3d.h"
    using namespace g2o;

    G2O_USE_TYPE_GROUP(slam3d);
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

proxy_obj!(EdgeSE3PointXYZDisparity, OptimizableGraphEdge);

impl EdgeSE3PointXYZDisparity {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeSE3PointXYZDisparity*" {
            return new EdgeSE3PointXYZDisparity();
        })
    }
}

proxy_obj!(EdgeSE3, OptimizableGraphEdge);

impl EdgeSE3 {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeSE3*" {
            return new EdgeSE3();
        })
    }
}

proxy_obj!(ParameterCamera, Parameter);

impl ParameterCamera {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "ParameterCamera*" {
            return new ParameterCamera();
        })
    }

    pub fn set_kcam(&mut self, fx: f64, fy: f64, cx: f64, cy: f64) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "ParameterCamera*",
              fx as "double",
              fy as "double",
              cx as "double",
              cy as "double"
        ]{
            obj->setKcam(fx, fy, cx, cy);
        })
    }
}
