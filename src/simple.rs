//! Computes variance using simple algorithm.
//!
//! ```
//!           sum(Xi Xi) - sum(Xi)sum(Xi)/N
//! Var(Xi) = -----------------------------
//!                        N
//! ```

pub fn compute_sum_pair(v: &[f32]) -> (f32, f32) {
    v.iter().fold((0.0, 0.0), |state, next| {
        let (sumsq, sum) = state;
        (sumsq + next * next, sum + next)
    })
}

pub fn variance32f_simple(v: &[f32]) -> f32 {
    let (sumsq, sum) = compute_sum_pair(v);
    let sqsum: f32 = sum * sum;
    let sz: usize = v.len();
    let n: f32 = sz as f32;
    let rcp: Option<f32> = match sz {
        0 => None,
        _ => Some(1.0 / n),
    };
    rcp.map(|r| {
        let sub: f32 = sumsq - sqsum * r;
        sub * r
    })
    .unwrap_or(f32::NAN)
}

pub fn variance32f_simple_unbiased(v: &[f32]) -> f32 {
    let (sumsq, sum) = compute_sum_pair(v);
    let sqsum: f32 = sum * sum;
    let sz: usize = v.len();
    let n: f32 = sz as f32;
    let rcp: Option<f32> = match sz {
        0 => None,
        _ => Some(1.0 / n),
    };
    let ratio: Option<f32> = match sz {
        0 => None,
        1 => None,
        _ => Some(n / (n - 1.0)),
    };
    rcp.and_then(|r| {
        ratio.map(|rt| {
            let sub: f32 = sumsq * r - sqsum * r * r;
            sub * rt
        })
    })
    .unwrap_or(f32::NAN)
}

#[cfg(feature = "ext_wasm")]
pub mod simple_wasm {

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_simple() -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_simple(v)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_simple_unbiased() -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_simple_unbiased(v)
    }
}
