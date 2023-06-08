// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

mod core;

use cpp::cpp;

cpp! {{
    #include "g2o/core/factory.h"
    #include "g2o/core/optimization_algorithm_factory.h"
    using namespace g2o;

    G2O_USE_TYPE_GROUP(slam2d);
    G2O_USE_TYPE_GROUP(slam3d);
    G2O_USE_OPTIMIZATION_LIBRARY(eigen);
}}

pub use crate::core::*;
