pub fn mean32f(v: &[f32]) -> f32 {
    let sz: usize = v.len();
    let n: f32 = sz as f32;
    let tot: f32 = v.iter().sum();
    let rcp: Option<f32> = match sz {
        0 => None,
        _ => Some(1.0 / n),
    };
    rcp.map(|r| r * tot).unwrap_or(f32::NAN)
}

pub fn mean64f(v: &[f64]) -> f64 {
    let sz: usize = v.len();
    let n: f64 = sz as f64;
    let tot: f64 = v.iter().sum();
    let rcp: Option<f64> = match sz {
        0 => None,
        _ => Some(1.0 / n),
    };
    rcp.map(|r| r * tot).unwrap_or(f64::NAN)
}

pub fn variance32f_2pass_unbiased(v: &[f32]) -> f32 {
    let mean: f32 = mean32f(v);
    let sum: f32 = v.iter().map(|f| (f - mean) * (f - mean)).sum();
    let sz: usize = v.len();
    let n: f32 = sz as f32;
    match sz {
        0 => None,
        1 => None,
        _ => Some(sum / (n - 1.0)),
    }
    .unwrap_or(f32::NAN)
}

pub fn variance32f_2pass_unbiased_partial64f(v: &[f32]) -> f32 {
    let mean: f64 = mean32f(v).into();
    let sum: f64 = v
        .iter()
        .map(|f| *f as f64)
        .map(|f| (f - mean) * (f - mean))
        .sum();
    let sz: usize = v.len();
    let n: f64 = sz as f64;
    match sz {
        0 => None,
        1 => None,
        _ => Some(sum / (n - 1.0)),
    }
    .map(|f| f as f32)
    .unwrap_or(f32::NAN)
}

pub fn variance64f_2pass_unbiased(v: &[f64]) -> f64 {
    let mean: f64 = mean64f(v);
    let sum: f64 = v.iter().map(|f| (f - mean) * (f - mean)).sum();
    let sz: usize = v.len();
    let n: f64 = sz as f64;
    match sz {
        0 => None,
        1 => None,
        _ => Some(sum / (n - 1.0)),
    }
    .unwrap_or(f64::NAN)
}

#[cfg(feature = "ext_wasm")]
pub mod wasm2pass {
    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_2pass_unbiased() -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_2pass_unbiased(v)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var64f_2pass_unbiased() -> f64 {
        let v: &[f64] = unsafe { &crate::BUF64F };
        super::variance64f_2pass_unbiased(v)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_2pass_unbiased_partial64f() -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_2pass_unbiased_partial64f(v)
    }
}
