//! Computes variance using shifted values.
//!
//! ```
//! mean(Xi) = sum(Xi)/N
//! Var(Xi) = mean(Xi Xi) - mean(Xi) mean(Xi)
//!         = sum(Xi Xi)/N - sum(Xi) sum(Xi) / NN
//!           sum(Xi Xi) - sum(Xi) sum(Xi)/N
//!         = ------------------------------------
//!                            N
//!             sum((Xi-C) (Xi-C)) - sum(Xi-C) sum(Xi-C)/N
//! Var(Xi-C) = ------------------------------------------
//!                                 N
//!             sum(Xi Xi + CC - 2CXi) - (-NC+sum(Xi))(-NC+sum(Xi))/N
//!           = ---------------------------------------------------
//!                                     N
//!             sum(Xi Xi + CC - 2CXi) - (NNCC + sum(Xi)sum(Xi) - 2NCsum(Xi))/N
//!           = ---------------------------------------------------------------
//!                                            N
//!             sum(Xi Xi + CC - 2CXi) - NCC + 2Csum(Xi) - sum(Xi)sum(Xi)/N
//!           = ---------------------------------------------------------------
//!                                            N
//!             sum(Xi Xi) + NCC - 2Csum(Xi) - NCC + 2Csum(Xi) - sum(Xi)sum(Xi)/N
//!           = ---------------------------------------------------------------
//!                                            N
//!             sum(Xi Xi) - sum(Xi)sum(Xi)/N
//!           = -----------------------------
//!                          N
//!           = Var(Xi)
//! ```

pub fn variance32f_shift_unbiased(v: &[f32], shift: f32) -> f32 {
    let (sumsq, sum) = v.iter().fold((0.0, 0.0), |state, next| {
        let (sq, sum) = state;
        let shifted: f32 = next - shift;
        (sq + shifted * shifted, sum + shifted)
    });
    let sz: usize = v.len();
    let n: f32 = sz as f32;
    let rcp: Option<f32> = match sz {
        0 => None,
        _ => Some(1.0 / n),
    };
    let rcp2: Option<f32> = match sz {
        0 => None,
        1 => None,
        _ => Some(1.0 / (n - 1.0)),
    };
    rcp.and_then(|r| {
        rcp2.map(|r2| {
            let sub: f32 = sumsq - (r * sum * sum);
            sub * r2
        })
    })
    .unwrap_or(f32::NAN)
}

#[cfg(feature = "ext_wasm")]
pub mod shift_wasm {
    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_shift_unbiased(shift: f32) -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_shift_unbiased(v, shift)
    }
}
