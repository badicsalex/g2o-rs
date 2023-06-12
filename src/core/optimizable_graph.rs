// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
};

use cpp::cpp;

use crate::{macros::proxy_obj_abstract, Parameter, RobustKernel};

cpp! {{
    #include "g2o/core/optimizable_graph.h"
    using namespace g2o;
}}

proxy_obj_abstract!(OptimizableGraph<'stored>);

impl<'stored> OptimizableGraph<'stored> {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "OptimizableGraph*"] {
            delete obj;
        })
    }

    // XXX: This _will_ leak memory, because the created edges and
    //      vertexes are never destroyed (due to
    //      G2O_NO_IMPLICIT_OWNERSHIP_OF_OBJECTS=ON )
    //      We should probably keep track of these loaded vertices.
    pub fn load(&mut self, filename: &str) -> bool {
        let obj = self.obj();
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph*", filename as "char*"] -> bool as "bool"{
            return obj->load(filename);
        })
    }

    pub fn save(&mut self, filename: &str) -> bool {
        let obj = self.obj();
        let filename = CString::new(filename).unwrap();
        let filename = filename.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph*", filename as "char*"] -> bool as "bool"{
            return obj->save(filename);
        })
    }

    pub fn add_vertex(&mut self, vertex: &'stored mut OptimizableGraphVertex) -> bool {
        let obj = self.obj();
        let vertex = vertex.obj();
        cpp!( unsafe [obj as "OptimizableGraph*", vertex as "OptimizableGraph::Vertex*"] -> bool as "bool"{
            return obj->addVertex(vertex);
        })
    }

    pub fn add_edge(&mut self, edge: &'stored mut OptimizableGraphEdge) -> bool {
        let obj = self.obj();
        let edge = edge.obj();
        cpp!( unsafe [obj as "OptimizableGraph*", edge as "OptimizableGraph::Edge*"] -> bool as "bool"{
            return obj->addEdge(edge);
        })
    }

    // OptimizableGraph takes ownership of parameters.
    // It's only edges and vertices that get spared.
    pub fn add_parameter(&mut self, parameter: Parameter) -> bool {
        let obj = self.obj();
        let parameter = parameter.into_obj();
        cpp!( unsafe [obj as "OptimizableGraph*", parameter as "Parameter*"] -> bool as "bool"{
            return obj->addParameter(parameter);
        })
    }

    pub fn optimize(&mut self, iterations: i32, online: bool) -> i32 {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph*", iterations as "int", online as "bool"] -> i32 as "int"{
            return obj->optimize(iterations, online);
        })
    }
}

proxy_obj_abstract!(OptimizableGraphVertex);

impl OptimizableGraphVertex {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*"] {
            delete obj;
        })
    }

    pub fn set_id(&mut self, id: i32) {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", id as "int"] {
            obj->setId(id);
        })
    }

    pub fn id(&mut self) -> i32 {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*"] -> i32 as "int" {
            return obj->id();
        })
    }

    pub fn set_marginalized(&mut self, marginalized: bool) {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", marginalized as "bool"] {
            obj->setMarginalized(marginalized);
        })
    }

    pub fn set_fixed(&mut self, fixed: bool) {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", fixed as "bool"] {
            obj->setFixed(fixed);
        })
    }

    pub fn set_estimate_data(&mut self, estimate: &[f64]) -> bool {
        assert!(estimate.len() >= self.estimate_dimension());
        let obj = self.obj();
        let estimate = estimate.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", estimate as "double*"] -> bool as "bool" {
            return obj->setEstimateData(estimate);
        })
    }

    pub fn get_estimate_data(&mut self, estimate: &mut [f64]) -> bool {
        assert!(estimate.len() >= self.estimate_dimension());
        let obj = self.obj();
        let estimate = estimate.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", estimate as "double*"] -> bool as "bool" {
            return obj->getEstimateData(estimate);
        })
    }

    pub fn estimate_dimension(&self) -> usize {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*"] -> i32 as "int" {
            return obj->estimateDimension();
        }) as usize
    }
}

proxy_obj_abstract!(OptimizableGraphEdge);

impl OptimizableGraphEdge {
    fn destruct(obj: *mut c_void) {
        cpp!( unsafe [obj as "OptimizableGraph::Edge*"] {
            delete obj;
        })
    }

    pub fn set_id(&mut self, id: i32) {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*", id as "int"] {
            obj->setId(id);
        })
    }

    pub fn id(&mut self) -> i32 {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*"] -> i32 as "int" {
            return obj->id();
        })
    }

    // XXX: not having a 'stored lifetime on vertex is incorrect, because the pointer
    //      is obviously stored, but that would prevent putting the vertex into the
    //      graph later.
    pub fn set_vertex(&mut self, index: i32, vertex: &mut OptimizableGraphVertex) {
        let obj = self.obj();
        let vertex = vertex.obj();
        cpp!( unsafe [
              obj as "OptimizableGraph::Edge*",
              index as "int",
              vertex as "OptimizableGraph::Vertex*"
        ] {
            obj->setVertex(index, vertex);
        })
    }

    pub fn set_parameter_id(&mut self, arg_num: i32, parameter_id: i32) {
        let obj = self.obj();
        cpp!( unsafe [
              obj as "OptimizableGraph::Edge*",
              arg_num as "int",
              parameter_id as "int"
        ] {
            obj->setParameterId(arg_num, parameter_id);
        })
    }

    pub fn dimension(&self) -> usize {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*"] -> i32 as "int" {
            return obj->dimension();
        }) as usize
    }

    pub fn chi2(&self) -> f64 {
        let obj = self.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*"] -> f64 as "double" {
            return obj->chi2();
        })
    }

    pub fn set_measurement_data(&mut self, measurement: &[f64]) -> bool {
        assert!(measurement.len() >= self.dimension());
        let obj = self.obj();
        let measurement = measurement.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*", measurement as "double*"] -> bool as "bool" {
            return obj->setMeasurementData(measurement);
        })
    }

    pub fn get_measurement_data(&mut self, measurement: &mut [f64]) -> bool {
        assert!(measurement.len() >= self.dimension());
        let obj = self.obj();
        let measurement = measurement.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*", measurement as "double*"] -> bool as "bool" {
            return obj->getMeasurementData(measurement);
        })
    }

    pub fn set_robust_kernel(&mut self, rk: &RobustKernel) {
        let obj = self.obj();
        let rk = rk.obj();
        cpp!( unsafe [obj as "OptimizableGraph::Edge*", rk as "RobustKernel*"] {
            obj->setRobustKernel(rk);
        })
    }
}
