//use std::arch::

use crate::neon::simd_llvm;

use std::mem;
use std::hint::unreachable_unchecked;
use core;

#[allow(unused)]
macro_rules! types {
    ($(
        $(#[$doc:meta])*
        pub struct $name:ident($($fields:tt)*);
    )*) => ($(
        $(#[$doc])*
        #[derive(Copy, Clone, Debug)]
        #[allow(non_camel_case_types)]
        #[repr(simd)]
        #[allow(clippy::missing_inline_in_public_items)]
        pub struct $name($($fields)*);
    )*)
}

#[allow(non_camel_case_types)]
pub type poly64_t = i64;
//#[allow(non_camel_case_types)]
//type poly128_t = [u8; 16];

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.aarch64.neon.pmull64"]
    fn vmull_p64_(a: i64, b: i64) -> int8x16_t;
    #[link_name = "llvm.aarch64.neon.vshrq"]
    fn vshrq_n_u8_(a: poly128_t, b: u8) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.addp.v16i8"]
    fn vpaddq_u8_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.addp.v16u8"]
    fn vaddq_u8_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.addp.v16i8"]
    fn vaddq_s8_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.addp.v16i32"]
    fn vaddq_s32_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.vextq.v16u8"]
    fn vextq_u8_(a: poly128_t, b: poly128_t, n: u8) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.vextq.v16s8"]
    fn vextq_s8_(a: poly128_t, b: poly128_t, n: u8) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.vtstq.v16u8"]
    fn vtstq_u8_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.aarch64.neon.vtstq.v16s8"]
    fn vtstq_s8_(a: poly128_t, b: poly128_t) -> poly128_t;
    #[link_name = "llvm.ctpop.i64"]
    fn ctpop_s64_(a: i64) -> i64;
    //    #[link_name = "llvm.ctlz.u64"]
    //    fn ctlz_u64_(a: u64) -> u32;
    #[link_name = "llvm.cttz.i64"]
    fn cttz_u64_(a: i64) -> i64;
}

#[inline]
pub unsafe fn vmull_p64(a: poly64_t, b: poly64_t) -> poly128_t {
    mem::transmute(vmull_p64_(mem::transmute(a), mem::transmute(b)))
}


#[inline]
pub unsafe fn vshrq_n_u8(a: uint8x16_t, b: u8) -> uint8x16_t {
    // FIXME?
    mem::transmute(vshrq_n_u8_(mem::transmute(a), mem::transmute(b)))
}

types! {
    /// ARM-specific 64-bit wide vector of eight packed `i8`.
    pub struct int8x8_t(i8, i8, i8, i8, i8, i8, i8, i8);
    /// ARM-specific 64-bit wide vector of eight packed `u8`.
    pub struct uint8x8_t(u8, u8, u8, u8, u8, u8, u8, u8);
    /// ARM-specific 64-bit wide polynomial vector of eight packed `u8`.
    pub struct poly8x8_t(u8, u8, u8, u8, u8, u8, u8, u8);
    /// ARM-specific 64-bit wide vector of four packed `i16`.
    pub struct int16x4_t(i16, i16, i16, i16);
    /// ARM-specific 64-bit wide vector of four packed `u16`.
    pub struct uint16x4_t(u16, u16, u16, u16);
    /// ARM-specific 64-bit wide vector of four packed `u16`.
    pub struct poly16x4_t(u16, u16, u16, u16);
    /// ARM-specific 64-bit wide vector of two packed `i32`.
    pub struct int32x2_t(i32, i32);
    /// ARM-specific 64-bit wide vector of two packed `u32`.
    pub struct uint32x2_t(u32, u32);
    /// ARM-specific 64-bit wide vector of two packed `f32`.
    pub struct float32x2_t(f32, f32);
    /// ARM-specific 64-bit wide vector of one packed `i64`.
    pub struct int64x1_t(i64);
    /// ARM-specific 64-bit wide vector of one packed `u64`.
    pub struct uint64x1_t(u64);
    /// ARM-specific 128-bit wide vector of sixteen packed `i8`.
    pub struct int8x16_t(
        i8, i8 ,i8, i8, i8, i8 ,i8, i8,
        i8, i8 ,i8, i8, i8, i8 ,i8, i8,
    );
    /// ARM-specific 128-bit wide vector of sixteen packed `u8`.
    pub struct uint8x16_t(
        u8, u8 ,u8, u8, u8, u8 ,u8, u8,
        u8, u8 ,u8, u8, u8, u8 ,u8, u8,
    );
    /// ARM-specific 128-bit wide vector of sixteen packed `u8`.
    pub struct poly8x16_t(
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8, u8, u8, u8, u8, u8, u8, u8
    );
    /// ARM-specific 128-bit wide vector of eight packed `i16`.
    pub struct int16x8_t(i16, i16, i16, i16, i16, i16, i16, i16);
    /// ARM-specific 128-bit wide vector of eight packed `u16`.
    pub struct uint16x8_t(u16, u16, u16, u16, u16, u16, u16, u16);
    /// ARM-specific 128-bit wide vector of eight packed `u16`.
    pub struct poly16x8_t(u16, u16, u16, u16, u16, u16, u16, u16);
    /// ARM-specific 128-bit wide vector of four packed `i32`.
    pub struct int32x4_t(i32, i32, i32, i32);
    /// ARM-specific 128-bit wide vector of four packed `u32`.
    pub struct uint32x4_t(u32, u32, u32, u32);
    /// ARM-specific 128-bit wide vector of four packed `f32`.
    pub struct float32x4_t(f32, f32, f32, f32);
    /// ARM-specific 128-bit wide vector of two packed `i64`.
    pub struct int64x2_t(i64, i64);
    /// ARM-specific 128-bit wide vector of two packed `u64`.
    pub struct uint64x2_t(u64, u64);
    /// ARM-specific 128-bit wide vector of one packed `i128`.
    pub struct poly128_t(i128); // FIXME: check this!
}

impl uint8x16_t {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8, i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8) -> uint8x16_t {
        uint8x16_t(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
    }
}

impl int8x16_t {
    pub fn new(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> int8x16_t {
        int8x16_t(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
    }
}

impl int32x4_t {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> int32x4_t {
        int32x4_t(a, b, c, d)
    }
}

#[inline]
pub fn add_overflow(a: u64, b: u64, out: &mut u64) -> bool {
    let (carry, did_carry) = a.overflowing_add(b);

    if did_carry {
        *out = carry;
    };

    did_carry
}

#[inline]
pub unsafe fn vld1q_s8(addr: *const i8) -> int8x16_t {
    *(addr as *const int8x16_t)
}

#[inline]
pub unsafe fn vld1q_u8(addr: *const u8) -> uint8x16_t {
    *(addr as *const uint8x16_t)
}

#[inline]
pub unsafe fn vst1q_u8(addr: *mut u8, val: uint8x16_t) {
    std::ptr::write(addr as *mut uint8x16_t, val);
}

macro_rules! aarch64_simd_2 {
    ($name:ident, $type:ty, $simd_fn:ident, $intr:ident) => {
        #[inline]
        #[target_feature(enable = "neon")]
        #[cfg_attr(test, assert_instr($intr))]
        pub unsafe fn $name(a: $type, b: $type) -> $type {
            simd_llvm::$simd_fn(a, b)
        }
    };
}
macro_rules! aarch64_simd_ceq {
    ($name:ident, $type:ty) => {
        /// Compare bitwise Equal (vector)
        aarch64_simd_2!($name, $type, simd_eq, cmeq);
    };
}

aarch64_simd_ceq!(vceqq_u8, uint8x16_t);
aarch64_simd_ceq!(vceq_s64, int64x1_t);
aarch64_simd_ceq!(vceqq_s64, int64x2_t);
aarch64_simd_ceq!(vceq_u64, uint64x1_t);
aarch64_simd_ceq!(vceqq_u64, uint64x2_t);
aarch64_simd_ceq!(vceq_p64, uint64x1_t);
aarch64_simd_ceq!(vceqq_p64, uint64x2_t);

aarch64_simd_ceq!(vceqq_s8, int8x16_t);

macro_rules! aarch64_simd_cgt {
    ($name:ident, $type:ty) => {
        /// Compare signed Greater than (vector)
        aarch64_simd_2!($name, $type, simd_gt, cmgt);
    };
}
macro_rules! aarch64_simd_cgtu {
    ($name:ident, $type:ty) => {
        /// Compare Greater than (vector)
        aarch64_simd_2!($name, $type, simd_gt, cmhi);
    };
}

aarch64_simd_cgtu!(vcgtq_u8, uint8x16_t);

aarch64_simd_cgt!(vcgt_s64, int64x1_t);
aarch64_simd_cgt!(vcgtq_s64, int64x2_t);
aarch64_simd_cgtu!(vcgt_u64, uint64x1_t);
aarch64_simd_cgtu!(vcgtq_u64, uint64x2_t);

macro_rules! aarch64_simd_clt {
    ($name:ident, $type:ty) => {
        /// Compare signed Lesser than (vector)
        aarch64_simd_2!($name, $type, simd_lt, cmgt);
    };
}
macro_rules! aarch64_simd_cltu {
    ($name:ident, $type:ty) => {
        /// Compare Lesser than (vector)
        aarch64_simd_2!($name, $type, simd_lt, cmhi);
    };
}

aarch64_simd_clt!(vclt_s64, int64x1_t);
aarch64_simd_clt!(vcltq_s64, int64x2_t);
aarch64_simd_cltu!(vclt_u64, uint64x1_t);
aarch64_simd_cltu!(vcltq_u64, uint64x2_t);

macro_rules! aarch64_simd_cge {
    ($name:ident, $type:ty) => {
        /// Compare signed Greater than (vector)
        aarch64_simd_2!($name, $type, simd_ge, cmge);
    };
}
macro_rules! aarch64_simd_cgeu {
    ($name:ident, $type:ty) => {
        /// Compare Greater than (vector)
        aarch64_simd_2!($name, $type, simd_ge, cmhs);
    };
}

aarch64_simd_cge!(vcge_s64, int64x1_t);
aarch64_simd_cge!(vcgeq_s64, int64x2_t);
aarch64_simd_cgeu!(vcge_u64, uint64x1_t);
aarch64_simd_cgeu!(vcgeq_u64, uint64x2_t);

macro_rules! aarch64_simd_cle {
    ($name:ident, $type:ty) => {
        /// Compare signed Lesser than (vector)
        aarch64_simd_2!($name, $type, simd_le, cmge);
    };
}
macro_rules! aarch64_simd_cleu {
    ($name:ident, $type:ty) => {
        /// Compare Lesser than (vector)
        aarch64_simd_2!($name, $type, simd_le, cmhs);
    };
}

aarch64_simd_cleu!(vcleq_u8, uint8x16_t);
aarch64_simd_cle!(vcle_s64, int64x1_t);
aarch64_simd_cle!(vcleq_s64, int64x2_t);
aarch64_simd_cleu!(vcle_u64, uint64x1_t);
aarch64_simd_cleu!(vcleq_u64, uint64x2_t);

aarch64_simd_cleu!(vcgtq_s8, int8x16_t);

#[inline]
pub fn vdupq_n_s8(a: i8) -> int8x16_t {
    int8x16_t(a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a)
}

#[inline]
pub fn zeroi8x16() -> int8x16_t {
    int8x16_t(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
}

#[inline]
pub fn vdupq_n_u8(a: u8) -> uint8x16_t {
    uint8x16_t(a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a)
}

#[inline]
pub fn vmovq_n_u8(a: u8) -> uint8x16_t {
    uint8x16_t(a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a)
}

#[inline]
pub fn zerou8x16() -> uint8x16_t {
    uint8x16_t(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
}

#[inline]
pub unsafe fn vpaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    mem::transmute(vpaddq_u8_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn vaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    mem::transmute(vaddq_u8_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn vaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    mem::transmute(vaddq_s8_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn vaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    mem::transmute(vaddq_s32_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t { simd_llvm::simd_and(a, b) }

#[inline]
pub unsafe fn vorrq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t { simd_llvm::simd_or(a, b) }

#[inline]
pub unsafe fn vandq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t { simd_llvm::simd_and(a, b) }

#[inline]
pub unsafe fn vorrq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t { simd_llvm::simd_or(a, b) }

macro_rules! arm_reinterpret {
    ($name:ident, $from:ty, $to:ty) => {
        // Vector reinterpret cast operation
        #[inline]
        #[target_feature(enable = "neon")]
        #[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
        pub unsafe fn $name(a: $from) -> $to {
            mem::transmute(a)
        }
    };
}

arm_reinterpret!(vreinterpret_u64_u32, uint32x2_t, uint64x1_t);
arm_reinterpret!(vreinterpretq_s8_u8, uint8x16_t, int8x16_t);
arm_reinterpret!(vreinterpretq_u16_u8, uint8x16_t, uint16x8_t);
arm_reinterpret!(vreinterpretq_u32_u8, uint8x16_t, uint32x4_t);
arm_reinterpret!(vreinterpretq_u64_u8, uint8x16_t, uint64x2_t);
arm_reinterpret!(vreinterpretq_u8_s8, int8x16_t, uint8x16_t);

arm_reinterpret!(vreinterpretq_s64_s8, int8x16_t, int64x2_t);

macro_rules! arm_vget_lane {
    ($name:ident, $to:ty, $from:ty, $lanes:literal) => {
        #[inline]
        #[target_feature(enable = "neon")]
        #[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
        #[cfg_attr(test, assert_instr(umov))]
        pub unsafe fn $name(v: $from, lane: u32) -> $to {
            if lane > $lanes { unreachable_unchecked() }
            simd_llvm::simd_extract(v, lane)
        }
    };
}

arm_vget_lane!(vgetq_lane_u16, u16, uint16x8_t, 7);
arm_vget_lane!(vgetq_lane_u32, u32, uint32x4_t, 3);
arm_vget_lane!(vgetq_lane_u64, u64, uint64x2_t, 1);
arm_vget_lane!(vget_lane_u64, u64, uint64x1_t, 0);

arm_vget_lane!(vgetq_lane_s16, i16, int16x8_t, 7);
arm_vget_lane!(vgetq_lane_s32, i32, int32x4_t, 3);
arm_vget_lane!(vgetq_lane_s64, i64, int64x2_t, 1);
arm_vget_lane!(vget_lane_s64, i64, int64x1_t, 0);

pub unsafe fn vextq_u8(a: uint8x16_t, b: uint8x16_t, n: u8) -> uint8x16_t {
    mem::transmute(vextq_u8_(mem::transmute(a), mem::transmute(b), n))
}

pub unsafe fn vextq_s8(a: int8x16_t, b: int8x16_t, n: u8) -> int8x16_t {
    mem::transmute(vextq_s8_(mem::transmute(a), mem::transmute(b), n))
}

#[inline]
pub fn vqmovn_u64(a: uint64x2_t) -> uint32x2_t {
    uint32x2_t(a.0 as u32, a.1 as u32)
}

#[inline]
pub unsafe fn vqtbl1q_s8(t: int8x16_t, idx: uint8x16_t) -> int8x16_t {
    mem::transmute(core::arch::aarch64::vqtbl1q_s8(mem::transmute(t), mem::transmute(idx)))
}

#[inline]
pub unsafe fn vqtbl1q_u8(t: uint8x16_t, idx: uint8x16_t) -> uint8x16_t {
    mem::transmute(core::arch::aarch64::vqtbl1q_s8(mem::transmute(t), mem::transmute(idx)))
}

#[inline]
pub unsafe fn vqsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    // FIXME?
    simd_llvm::simd_sub(mem::transmute(a), mem::transmute(b))
}

#[inline]
pub unsafe fn vtstq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    mem::transmute(vtstq_u8_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn vtstq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    mem::transmute(vtstq_s8_(mem::transmute(a), mem::transmute(b)))
}

#[inline]
pub unsafe fn hamming(a: u64) -> u32 {
    ctpop_s64_(a as i64) as u32
}

#[inline]
pub fn trailingzeroes(a: u64) -> u32 {
    unsafe { cttz_u64_(a as i64) as u32 }
}

#[inline]
pub unsafe fn vst1q_u32(addr: *mut u8, val: uint32x4_t) {
    std::ptr::write(addr as *mut uint32x4_t, val)
}