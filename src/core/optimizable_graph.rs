// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
};

use cpp::cpp;

use crate::macros::proxy_obj_abstract;

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

    pub fn set_estimate_data(&mut self, estimate: &[f64]) -> bool {
        assert!(estimate.len() >= self.estimate_dimension());
        let obj = self.obj_mut();
        let estimate = estimate.as_ptr();
        cpp!( unsafe [obj as "OptimizableGraph::Vertex*", estimate as "double*"] -> bool as "bool" {
            return obj->setEstimateData(estimate);
        })
    }

    pub fn get_estimate_data(&mut self, estimate: &mut [f64]) -> bool {
        assert!(estimate.len() >= self.estimate_dimension());
        let obj = self.obj_mut();
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
}
