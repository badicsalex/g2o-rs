// Copyright (C) 2023, Alex Badics
// This file is part of g2o-rs
// Licensed under the BSD 2 Clause license. See LICENSE file in the project root for details.

macro_rules! proxy_obj_no_constructor {
    ($name: ident $(<$nlt: lifetime>)?, $parent: ty) => {
        pub struct $name $(<$nlt>)? {
            parent: $parent,
        }

        impl $(<$nlt>)? $name $(<$nlt>)? {
            #[allow(dead_code)]
            pub(crate) fn obj_mut(&mut self) -> *mut c_void {
                // NOTE: this is only correct if this class has only one parent
                //       on C++ side. The correct thing to do would be a
                //       dynamic_cast or similar.
                self.parent.obj_mut()
            }

            #[allow(dead_code)]
            pub(crate) fn obj(&self) -> *const c_void {
                // NOTE: this is only correct if this class has only one parent
                //       on C++ side. The correct thing to do would be a
                //       dynamic_cast or similar.
                self.parent.obj()
            }
        }

        impl $(<$nlt>)? Deref for $name $(<$nlt>)? {
            type Target = $parent;

            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }

        impl $(<$nlt>)? DerefMut for $name $(<$nlt>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
    };
    ($name: ident $(<$nlt: lifetime>)?) => {
        pub struct $name $(<$nlt>)? {
            _obj: *mut c_void,
            $(_stored_data_tag: PhantomData<&$nlt ()>,)?
        }

        impl $(<$nlt>)? $name $(<$nlt>)? {
            #[allow(dead_code)]
            pub(crate) fn obj_mut(&mut self) -> *mut c_void {
                self._obj
            }

            #[allow(dead_code)]
            pub(crate) fn obj(&self) -> *const c_void {
                self._obj
            }
        }

        impl $(<$nlt>)? Drop for $name $(<$nlt>)? {
            fn drop(&mut self) {
                Self::destruct(self._obj)
            }
        }

    };
}

macro_rules! proxy_obj_abstract {
    ($name: ident $(<$nlt: lifetime>)?, $parent: ty) => {
        impl $(<$nlt>)? $name $(<$nlt>)? {
            pub fn new_from(obj: *mut c_void) -> Self {
                Self {
                    parent: <$parent>::new_from(obj),
                }
            }
        }
        crate::macros::proxy_obj_no_constructor!($name $(<$nlt>)?, $parent);
    };
    ($name: ident $(<$nlt: lifetime>)?) => {
        impl $(<$nlt>)? $name $(<$nlt>)? {
            pub fn new_from(obj: *mut c_void) -> Self {
                Self {
                    _obj: obj,
                    $(_stored_data_tag: {
                        // Trick so that the $()? macro expansion works
                        let _: &$nlt u8;
                        PhantomData
                    },)?
                }
            }
        }
        crate::macros::proxy_obj_no_constructor!($name $(<$nlt>)?);
    };
}

macro_rules! proxy_obj {
    ($name: ident $(<$nlt: lifetime>)?, $parent: ty) => {
        impl $(<$nlt>)? $name $(<$nlt>)? {
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl $(<$nlt>)? Default for $name $(<$nlt>)? {
            fn default() -> Self {
                Self::new_from(Self::construct())
            }
        }
        crate::macros::proxy_obj_abstract!($name $(<$nlt>)?, $parent);
    };
    ($name: ident $(<$nlt: lifetime>)?) => {
        impl $(<$nlt>)? $name $(<$nlt>)? {
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl $(<$nlt>)? Default for $name $(<$nlt>)? {
            fn default() -> Self {
                Self::new_from(Self::construct())
            }
        }
        crate::macros::proxy_obj_abstract!($name $(<$nlt>)?);

    };
}

pub(crate) use proxy_obj;
pub(crate) use proxy_obj_abstract;
pub(crate) use proxy_obj_no_constructor;
