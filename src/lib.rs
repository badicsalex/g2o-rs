// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

mod core;
pub(crate) mod macros;
mod types;

use cpp::cpp;

cpp! {{
    #include "g2o/core/factory.h"
    #include "g2o/core/optimization_algorithm_factory.h"
    using namespace g2o;

}}

pub use crate::core::*;
pub use crate::types::*;
