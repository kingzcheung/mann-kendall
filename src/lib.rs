
use libc::size_t;
use std::slice;

pub mod mann_kendall;
pub use mann_kendall::test;


#[repr(C)]
#[derive(Debug)]
pub struct Trend {
    pub norm_stat: f64,
    pub is_present: bool
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn mann_kendall_a(ptr: *mut f32, len: size_t  ,alpha:f64) ->Trend{
    let len = len as usize;
    let v = slice::from_raw_parts_mut(ptr, len);
    // let v = Vec::from_raw_parts(ptr, len, len);
    let v = v.to_vec();
    let (z,h) = test(v, alpha);

    Trend { norm_stat: z, is_present: h}
    
}

/// # Safety
/// 曼－肯德尔趋势检验
/// 
#[no_mangle]
pub unsafe extern "C" fn mann_kendall(ptr: *mut f32, len: size_t  ,alpha:f64) ->f64{
    let len = len as usize;
    let v = slice::from_raw_parts_mut(ptr, len);
    // let v = Vec::from_raw_parts(ptr, len, len);
    let v = v.to_vec();
    let (r,_) = test(v, alpha);

    r
    
}
