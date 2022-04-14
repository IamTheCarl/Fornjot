//! Minimal foreign function interface for lib3mf
//!
//! See these header files for more information:
//! - https://github.com/3MFConsortium/lib3mf/blob/master/Autogenerated/Bindings/C/lib3mf.h
//! - https://github.com/3MFConsortium/lib3mf/blob/master/Autogenerated/Bindings/C/lib3mf_types.h

use std::{ffi::c_void, os::raw::c_char};

pub type Model = Handle;
pub type Object = Handle;
pub type Reader = Handle;

pub type ObjectIterator = Handle;
pub type ResourceIterator = Handle;

pub type Handle = *const c_void;
pub type Result = i32;

extern "C" {
    pub fn lib3mf_createmodel(pModel: *mut Model) -> Result;

    pub fn lib3mf_model_queryreader(
        pModel: Model,
        pReaderClass: *const c_char,
        pReaderInstance: *mut Reader,
    ) -> Result;

    pub fn lib3mf_reader_setstrictmodeactive(
        pReader: Reader,
        bStrictModeActive: bool,
    ) -> Result;

    pub fn lib3mf_reader_readfromfile(
        pReader: Reader,
        pFilename: *const c_char,
    ) -> Result;

    pub fn lib3mf_reader_getwarningcount(
        pReader: Reader,
        pCount: *mut u32,
    ) -> Result;

    pub fn lib3mf_model_getobjects(
        pModel: Model,
        pResourceIterator: *mut ObjectIterator,
    ) -> Result;

    pub fn lib3mf_resourceiterator_movenext(
        pResourceIterator: ResourceIterator,
        pHasNext: *mut bool,
    ) -> Result;

    pub fn lib3mf_objectiterator_getcurrentobject(
        pObjectIterator: ObjectIterator,
        pResource: *mut Object,
    ) -> Result;

    pub fn lib3mf_object_isvalid(
        pObject: Object,
        pIsValid: &mut bool,
    ) -> Result;
}
