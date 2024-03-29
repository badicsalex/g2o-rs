// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use cpp::cpp;

use crate::{macros::proxy_obj, OptimizableGraphEdge, OptimizableGraphVertex};

cpp! {{
    #include "g2o/types/vio/types_vio.h"
    using namespace g2o;

    G2O_USE_TYPE_GROUP(vio);
}}

proxy_obj!(VertexSpeed, OptimizableGraphVertex);

impl VertexSpeed {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "VertexSpeed*" {
            return new VertexSpeed();
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

proxy_obj!(EdgeSpeed, OptimizableGraphEdge);

impl EdgeSpeed {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeSpeed*" {
            return new EdgeSpeed();
        })
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_measurement(
        &mut self,
        preintegrated_speed: nalgebra::Vector3<f64>,
        delta_t: f64,
        acc_bias_covariance: nalgebra::Matrix3<f64>,
    ) {
        self.set_measurement_data(&[
            preintegrated_speed.x,
            preintegrated_speed.y,
            preintegrated_speed.z,
            delta_t,
            acc_bias_covariance.m11,
            acc_bias_covariance.m21,
            acc_bias_covariance.m31,
            acc_bias_covariance.m12,
            acc_bias_covariance.m22,
            acc_bias_covariance.m32,
            acc_bias_covariance.m13,
            acc_bias_covariance.m23,
            acc_bias_covariance.m33,
        ]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_information(&mut self, weight: f32) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "EdgeSpeed*",
              weight as "float"
        ]{
            Vector3 diagonal(weight, weight, weight);
            obj->setInformation(diagonal.asDiagonal());
        })
    }
}

proxy_obj!(VertexImuBias, OptimizableGraphVertex);

impl VertexImuBias {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "VertexImuBias*" {
            return new VertexImuBias();
        })
    }
    #[cfg(feature = "nalgebra")]
    pub fn set_estimate(
        &mut self,
        gyro_bias: nalgebra::Vector3<f64>,
        accelerometer_bias: nalgebra::Vector3<f64>,
    ) {
        self.set_estimate_data(&[
            gyro_bias.x,
            gyro_bias.y,
            gyro_bias.z,
            accelerometer_bias.x,
            accelerometer_bias.y,
            accelerometer_bias.z,
        ]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn get_estimate(&self) -> (nalgebra::Vector3<f64>, nalgebra::Vector3<f64>) {
        let mut raw_data = [0.0; 6];
        self.get_estimate_data(&mut raw_data);
        (
            nalgebra::Vector3::from_row_slice(&raw_data[0..3]),
            nalgebra::Vector3::from_row_slice(&raw_data[3..6]),
        )
    }
}

proxy_obj!(EdgeImuBias, OptimizableGraphEdge);

impl EdgeImuBias {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeImuBias*" {
            return new EdgeImuBias();
        })
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_measurement(&mut self, bias: nalgebra::Vector6<f64>) {
        self.set_measurement_data(bias.as_slice());
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_information(&mut self, rotation_weight: f32, translation_weight: f32) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "EdgeImuBias*",
              rotation_weight as "float",
              translation_weight as "float"
        ]{
            Vector6 diagonal(
                rotation_weight, rotation_weight, rotation_weight,
                translation_weight, translation_weight, translation_weight
            );
            obj->setInformation(diagonal.asDiagonal());
        })
    }
}

proxy_obj!(EdgeImuMeasurement, OptimizableGraphEdge);

impl EdgeImuMeasurement {
    fn construct() -> *mut c_void {
        cpp!( unsafe [] -> *mut c_void as "EdgeImuMeasurement*" {
            return new EdgeImuMeasurement();
        })
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_measurement(
        &mut self,
        preintegrated_position: nalgebra::Vector3<f64>,
        preintegrated_rotation: nalgebra::UnitQuaternion<f64>,
        delta_t: f64,
        used_gyro_bias: nalgebra::Vector3<f64>,
        acc_bias_covariance: nalgebra::Matrix3<f64>,
    ) {
        self.set_measurement_data(&[
            preintegrated_position.x,
            preintegrated_position.y,
            preintegrated_position.z,
            preintegrated_rotation.i,
            preintegrated_rotation.j,
            preintegrated_rotation.k,
            preintegrated_rotation.w,
            delta_t,
            used_gyro_bias.x,
            used_gyro_bias.y,
            used_gyro_bias.z,
            acc_bias_covariance.m11,
            acc_bias_covariance.m21,
            acc_bias_covariance.m31,
            acc_bias_covariance.m12,
            acc_bias_covariance.m22,
            acc_bias_covariance.m32,
            acc_bias_covariance.m13,
            acc_bias_covariance.m23,
            acc_bias_covariance.m33,
        ]);
    }

    #[cfg(feature = "nalgebra")]
    pub fn set_information(&mut self, rotation_weight: f32, translation_weight: f32) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "EdgeImuMeasurement*",
              rotation_weight as "float",
              translation_weight as "float"
        ]{
            Vector6 diagonal(
                translation_weight, translation_weight, translation_weight,
                rotation_weight, rotation_weight, rotation_weight
            );
            obj->setInformation(diagonal.asDiagonal());
        })
    }
}
