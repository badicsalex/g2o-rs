use std::pin::Pin;

use autocxx::WithinUniquePtr;
use cxx::let_cxx_string;
use g2o_sys::{OptimizableGraph, OptimizationAlgorithmProperty, SparseOptimizer};

fn main() {
    let mut optimizer = SparseOptimizer::new().within_unique_ptr();
    let mut solver_property = OptimizationAlgorithmProperty::new().within_unique_ptr();
    let_cxx_string!(optimizer_name = "lm_var");
    let algorithm = unsafe {
        (*g2o_sys::OptimizationAlgorithmFactory::instance())
            .construct(&optimizer_name, solver_property.pin_mut())
    };

    unsafe { optimizer.pin_mut().setAlgorithm(algorithm) }
    unsafe {
        optimizer
            .pin_mut()
            .as_superclass_hack::<OptimizableGraph>()
            .load("in.g2o\0".as_ptr() as *const i8);
    }
    optimizer.pin_mut().initializeOptimization2(0.into());
    optimizer.pin_mut().optimize(10.into(), false);
    unsafe {
        optimizer
            .as_ref()
            .unwrap()
            .as_ref()
            .save("out.g2o\0".as_ptr() as *const i8, 0.into());
    }
}

trait SuperclassHack {
    fn as_superclass_hack<T>(self: Pin<&mut Self>) -> Pin<&mut T>
    where
        Self: AsRef<T>,
    {
        let converted_ref = (*self).as_ref();
        let immutable_ptr = converted_ref as *const T;
        let mutable_ptr = immutable_ptr as *mut T;
        unsafe { Pin::new_unchecked(&mut *mutable_ptr) }
    }
}

impl SuperclassHack for SparseOptimizer {}
