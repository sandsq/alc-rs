use super::{key::PhalanxKey, layer::Layer, layout::Layout};
use strum::IntoEnumIterator;
// #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter, Serialize, Deserialize)]
// pub enum LayoutPreset {
// 	FerrisSweep,
// }

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
pub enum LayoutSizePresets {
	FourByTen,
	FourByTwelve,
}

pub fn get_all_layout_size_presets() -> Vec<LayoutSizePresets> {
	LayoutSizePresets::iter().collect()
}


impl Default for Layout<4, 12> {
	fn default() -> Self {
		Layout::try_from(
		"
		___Layer 0___
				0       1       2       3       4       5       6       7       8       9      10      11 
		0| __10  __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		1| __10  __10    __10    __10    __10    __10    __10    __10    __10    __10    __10  __10
		2| SFT_11    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    SFT_11 
		3|   __10    __10    __10    __10    LS1_10  SPC_00  BSPC_00  LS2_10    __10    __10    __10    __10 

		___Layer 1___
				0       1       2       3       4       5       6       7       8       9      10      11 
		0|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		1|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		2|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		3|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 

		___Layer 2___
				0       1       2       3       4       5       6       7       8       9      10      11 
		0|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		1|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		2|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		3|   __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10    __10 
		").unwrap()
	}
}

impl Default for Layer<4, 12, f64> {
	fn default() -> Self {
		Layer::try_from("
		12 7 2 2 2 7 7 2 2 2 7 12
		6 3 1 1 1 3 3 1 1 1 3 6
		13 5 3 3 3 8 8 3 3 3 5 13
		14 10 7 4 2 1 1 2 4 7 10 14
		").unwrap()
	}
}
impl Default for Layer<4, 12, PhalanxKey> {
	fn default() -> Self {
		Layer::try_from("
		L:P L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P R:P
		L:P L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P R:P
		L:P L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P R:P
		L:J L:P L:R L:T L:T L:T R:T R:T R:T R:R R:P R:J
		").unwrap()
	}
}

impl Default for Layout<4, 10> {
	fn default() -> Self {
		Layout::try_from("
		___Layer 0___
		__10 __10 __10 __10   __10   __10    __10   __10 __10 __10 
		__10 __10 LS3_10 __10 __10   __10    __10 __10 __10 __10 
		SFT_11 __10 __10 __10   __10   __10    __10   __10 __10 SFT_11
		__00 __00 __00 LS1_00 SPC_00 BSPC_00 LS2_00 __00 __00 __00 
		___Layer 1___
		__10 __10 __10 __10 __10 __10 __10 __10 __10 __10 
		__10 LCBR_00 LBRC_00 LPRN_00 __10 __10 RPRN_00 RBRC_00 RCBR_00 __10 
		__10 __10 __10 __10 __10 __10 __10 __10 __10 __10 
		__00 __00 __00 __10 __10 __10 __10 __00 __00 __00 
		___Layer 2___
		1_00 2_00 3_00 4_00 5_00 __10 __10 __10 __10 __10 
		6_00 7_00 8_00 9_00 ZERO_00 __10 LEFT_00 DOWN_00 UP_00 RGHT_00 
		__10 __10 __10 __10 __10 __10 HOME_00 PGDN_00 PGUP_00 END_00 
		__00 __00 __00 __10 __10 __10 __10 __00 __00 __00 
		___Layer 3___
		__10 __10 __10 __10 __10 __10 __10 __10 __10 __10 
		__10 __10 __10 __10 __10 __10 __10 __10 __10 __10 
		__10 __10 __10 __10 __10 __10 __10 __10 __10 __10 
		__00 __00 __00 __10 __10 __10 __10 __00 __00 __00 
		").unwrap()
	}
}
impl Default for Layer<4, 10, f64> {
	fn default()-> Self {
		Layer::try_from("
		7  2  2  2  7  7  2  2  2  7
		3  1  1  1  3  3  1  1  1  3
		5  3  3  3  8  8  3  3  3  5
		10 7  4  2  1  1  2  4  7  10
		").unwrap()
	}
}

impl Default for Layer<4, 10, PhalanxKey> {
	fn default() -> Self {
		Layer::try_from("
		L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P
		L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P
		L:P L:R L:M L:I L:I R:I R:I R:M R:R R:P
		L:P L:R L:T L:T L:T R:T R:T R:T R:R R:P
		").unwrap()
	}
}

