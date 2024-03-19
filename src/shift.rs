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

use core::arch::wasm32::v128;
use core::arch::wasm32::{f32x4, f32x4_add, f32x4_extract_lane, f32x4_mul};

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

/// Computes unbiased variance using simd.
///
/// This function assumes v.len() % 4 === 0.
#[cfg(all(feature = "wasm_simd", target_family = "wasm"))]
pub fn variance32f_shift_unbiased_simd4(v: &[f32], shift: f32) -> f32 {
    let init: v128 = f32x4(0.0, 0.0, 0.0, 0.0);
    let s4: v128 = f32x4(-shift, -shift, -shift, -shift);
    let (sumsq, sum) = v.chunks_exact(4).fold((init, init), |state, next| {
        let (sq, sm) = state;

        let f1: f32 = next.first().copied().unwrap_or_default();
        let f2: f32 = next.get(1).copied().unwrap_or_default();
        let f3: f32 = next.get(2).copied().unwrap_or_default();
        let f4: f32 = next.get(3).copied().unwrap_or_default();
        let n4: v128 = f32x4(f1, f2, f3, f4);
        let shifted: v128 = f32x4_add(n4, s4);
        let ssq: v128 = f32x4_mul(shifted, shifted);

        let l: v128 = f32x4_add(sq, ssq);
        let r: v128 = f32x4_add(sm, shifted);

        (l, r)
    });

    let l1: f32 = f32x4_extract_lane::<0>(sumsq);
    let l2: f32 = f32x4_extract_lane::<1>(sumsq);
    let l3: f32 = f32x4_extract_lane::<2>(sumsq);
    let l4: f32 = f32x4_extract_lane::<3>(sumsq);

    let r1: f32 = f32x4_extract_lane::<0>(sum);
    let r2: f32 = f32x4_extract_lane::<1>(sum);
    let r3: f32 = f32x4_extract_lane::<2>(sum);
    let r4: f32 = f32x4_extract_lane::<3>(sum);

    let sumsq: f32 = l1 + l2 + l3 + l4;
    let sum: f32 = r1 + r2 + r3 + r4;

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

#[cfg(all(feature = "ext_wasm", feature = "wasm_simd", target_family = "wasm"))]
pub mod shift_wasm_simd128 {
    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn var32f_shift_unbiased_simd128(shift: f32) -> f32 {
        let v: &[f32] = unsafe { &crate::BUF32F };
        super::variance32f_shift_unbiased_simd4(v, shift)
    }
}
