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

    #[cfg(feature = "nalgebra")]
    pub fn set_estimate(&mut self, estimate: nalgebra::Isometry3<f64>) {
        self.set_estimate_data(&[
            estimate.translation.x,
            estimate.translation.y,
            estimate.translation.z,
            estimate.rotation.i,
            estimate.rotation.j,
            estimate.rotation.k,
            estimate.rotation.w,
        ]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn get_estimate(&self) -> nalgebra::Isometry3<f64> {
        let mut raw_data = [0.0; 7];
        self.get_estimate_data(&mut raw_data);
        nalgebra::Isometry3::from_parts(
            nalgebra::Translation3::new(raw_data[0], raw_data[1], raw_data[2]),
            nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
                raw_data[6],
                raw_data[3],
                raw_data[4],
                raw_data[5],
            )),
        )
    }
}

proxy_obj!(VertexPointXYZ, OptimizableGraphVertex);

impl VertexPointXYZ {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "VertexPointXYZ*" {
            return new VertexPointXYZ();
        })
    }
    #[cfg(feature = "nalgebra")]
    pub fn set_estimate(&mut self, estimate: nalgebra::Vector3<f64>) {
        self.set_estimate_data(&[estimate.x, estimate.y, estimate.z]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn get_estimate(&self) -> nalgebra::Vector3<f64> {
        let mut raw_data = [0.0; 3];
        self.get_estimate_data(&mut raw_data);
        nalgebra::Vector3::new(raw_data[0], raw_data[1], raw_data[2])
    }
}

proxy_obj!(EdgeSE3PointXYZDisparity, OptimizableGraphEdge);

impl EdgeSE3PointXYZDisparity {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeSE3PointXYZDisparity*" {
            return new EdgeSE3PointXYZDisparity();
        })
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_measurement(&mut self, position: nalgebra::Vector2<f64>, disparity: f64) {
        self.set_measurement_data(&[position.x, position.y, disparity]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_information(&mut self, position_weight: f32, disparity_weight: f32) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "EdgeSE3PointXYZDisparity*",
              position_weight as "float",
              disparity_weight as "float"
        ]{
            Vector3 diagonal(position_weight, position_weight, disparity_weight);
            obj->setInformation(diagonal.asDiagonal());
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
