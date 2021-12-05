//This file was automatically generated. Please do not modify it directly
use ::phf::{{phf_map, Map}};
pub const BIT_VEC_LEN: usize = 1;
pub static CRN_TIMES: Map<u32, [u64; BIT_VEC_LEN]> = phf_map! {
	90004u32 => [6],
	90006u32 => [3],
	90014u32 => [5],
	90008u32 => [4],
	90009u32 => [4],
	90012u32 => [1],

};
pub static CRN_COURSES: Map<u32, &'static str> = phf_map! {
	90004u32 => "ARCH-1960",
	90006u32 => "ARCH-1960",
	90014u32 => "ARTS-2350",
	90008u32 => "MGMT-1960",
	90009u32 => "PHYS-2960",
	90012u32 => "STSO-2960",
};