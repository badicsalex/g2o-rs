// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

use g2o::{OptimizationAlgorithmFactory, OptimizationAlgorithmProperty, SparseOptimizer};

fn main() {
    let mut solver_property = OptimizationAlgorithmProperty::new();
    let algo = OptimizationAlgorithmFactory::construct(
        "lm_var",
        &mut solver_property,
    );
    let mut optimizer = SparseOptimizer::new();
    optimizer.set_algorithm(&algo);
    optimizer.load("in.g2o");
    optimizer.initialize_optimization(0);
    optimizer.optimize(10, false);
    optimizer.save("out.g2o");
}
