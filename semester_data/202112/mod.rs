//This file was automatically generated. Please do not modify it directly
use ::phf::{{phf_map, Map}};
pub const BIT_VEC_LEN: usize = 1;
pub static CRN_TIMES: Map<u32, [u64; BIT_VEC_LEN]> = phf_map! {
	90004u32 => [2],
	90006u32 => [3],
	90012u32 => [1],

};
pub static CRN_COURSES: Map<u32, &'static str> = phf_map! {
	90004u32 => "ARCH-1960",
	90006u32 => "ARCH-1960",
	90012u32 => "STSO-2960",
};