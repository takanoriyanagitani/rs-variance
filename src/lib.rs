#[cfg(feature = "ext_wasm")]
static mut BUF32F: Vec<f32> = vec![];

#[cfg(feature = "ext_wasm")]
static mut BUF64F: Vec<f64> = vec![];

#[cfg(feature = "ext_wasm")]
pub mod buf32f {
    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn ptr32f() -> *mut f32 {
        let mv: &mut Vec<f32> = unsafe { &mut crate::BUF32F };
        mv.as_mut_ptr()
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn resize32f(sz: i32) -> i32 {
        let mv: &mut Vec<f32> = unsafe { &mut crate::BUF32F };
        mv.resize(sz as usize, 0.0);
        mv.capacity().try_into().ok().unwrap_or(-1)
    }
}

#[cfg(feature = "ext_wasm")]
pub mod buf64f {
    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn ptr64f() -> *mut f64 {
        let mv: &mut Vec<f64> = unsafe { &mut crate::BUF64F };
        mv.as_mut_ptr()
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn resize64f(sz: i32) -> i32 {
        let mv: &mut Vec<f64> = unsafe { &mut crate::BUF64F };
        mv.resize(sz as usize, 0.0);
        mv.capacity().try_into().ok().unwrap_or(-1)
    }
}

#[cfg(feature = "simple")]
pub mod simple;

#[cfg(feature = "two-pass")]
pub mod two_pass;

#[cfg(feature = "shift")]
pub mod shift;
