use autocxx::prelude::*;

include_cpp! {
    #include "g2o/core/optimization_algorithm_factory.h"
    #include "g2o/core/optimizable_graph.h"
    #include "g2o/core/sparse_optimizer.h"
    #include "g2o/core/hyper_graph.h"
    #include "g2o/types/slam2d/types_slam2d.h"
    safety!(unsafe_ffi)
    generate!("g2o::OptimizationAlgorithmFactory")
    generate!("g2o::OptimizationAlgorithmProperty")
    generate!("g2o::SparseOptimizer")
    generate!("g2o::OptimizableGraph")
}

#[link(name = "g2o_core", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_cholmod", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_dense", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_eigen", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_pcg", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_slam2d_linear", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_solver_structure_only", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_stuff", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_data", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_icp", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_sba", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_sclam2d", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_sim3", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_slam2d", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_slam2d_addons", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_slam3d", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}
#[link(name = "g2o_types_slam3d_addons", kind = "static", modifiers = "+whole-archive,-bundle")]
extern {}

pub use ffi::std;
pub use ffi::g2o::*;
