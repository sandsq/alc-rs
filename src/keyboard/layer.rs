use array2d::{Array2D, Error as Array2DError};
use delegate::delegate;
use std::ops::Index;
use rand::prelude::*;
use std::error::Error;
use std::fmt;

use crate::text_processor::keycode::Keycode::{self, *};
use super::key::{KeyValue, KeycodeKey};
use super::LayoutPosition;

#[derive(Debug, PartialEq)]
pub enum KeyboardError {
	SymmetryError(usize, usize, usize, usize)
}
impl Error for KeyboardError {}
impl fmt::Display for KeyboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			KeyboardError::SymmetryError(r1, c1, r2, c2) =>
					write!(f, "Position ({r1}, {c1}) is marked as symmetric but its corresponding symmetric position ({r2}, {c2}) is not.")
		}
    }
}

/// Layers are grids. For non-grid keyboard layouts, create the largest grid that fits and block unused cells with dummy keys. Works for anything implementing [KeyValue]
#[derive(Debug, PartialEq, Clone)]
pub struct Layer<const R: usize, const C: usize, K: KeyValue> {
	layer: Array2D<K>
}
impl<const R: usize, const C: usize, K: KeyValue + std::clone::Clone> Layer<R, C, K> {
	pub fn from_rows(elements: &[Vec<K>]) -> Result<Self, Array2DError> {
		let layer_array2d = Array2D::from_rows(elements)?;
		Ok(Layer::<R, C, K> { layer: layer_array2d })
	}
	// maybe just return Option like Array2D?
	pub fn get(&self, r: usize, c: usize) -> Result<K, Array2DError> {
		match self.layer.get(r, c) {
			Some(v) => Ok(v.clone()),
			None => Err(Array2DError::IndicesOutOfBounds(r, c)),
		}
	}
	pub fn get_mut(&mut self, r: usize, c: usize) -> Result<&mut K, Array2DError> {
		match self.layer.get_mut(r, c) {
			Some(v) => Ok(v),
			None => Err(Array2DError::IndicesOutOfBounds(r, c)),
		}
	}
	pub fn set(&mut self, row: usize, col: usize, element: K) -> Result<(), Array2DError> {
		self.layer.set(row, col, element)
	}
	pub fn get_from_layout_position(&self, l: &LayoutPosition) -> 
			Result<K, Array2DError> {
		self.get(l.row_index, l.col_index)
	}
	pub fn num_rows(&self) -> usize {
		R
	}
	pub fn num_columns(&self) -> usize {
		C
	}
	// Specifically, mirrored left-right
	pub fn symmetric_position(&self, l: LayoutPosition) -> LayoutPosition {
		let num_rows = self.num_rows();
		let num_cols = self.num_columns();
		let orig_row = l.row_index;
		let orig_col = l.col_index;
		let symm_col = (num_cols - 1) - orig_col;
		LayoutPosition { layer_index: l.layer_index, row_index: orig_row, col_index: symm_col }
	}
}
impl<const R: usize, const C: usize> Layer<R, C, KeycodeKey> {
	pub fn init_blank() -> Self {
		let default_key = KeycodeKey::from_keycode(_NO);
		let mut layer_array2d = Array2D::filled_with(default_key.clone(), R, C);
		Layer::<R, C, KeycodeKey> { layer: layer_array2d }
	}
	pub fn randomize(&mut self, rng: &mut impl Rng, valid_keycodes: Vec<Keycode>) -> Result<(), KeyboardError> {
		for i in 0..R {
			for j in 0..C {
				let key = self.get(i, j).unwrap();
				let lp = LayoutPosition::for_layer(i, j);
				if key.is_symmetric() {
					let symm_lp = self.symmetric_position(lp);
					let symm_key = self.get_from_layout_position(&symm_lp).unwrap();
					// println!("{} {} is symmetric, checking {} {}: {:?}", i, j, &symm_lp.row_index, &symm_lp.col_index, &symm_key);
					if !symm_key.is_symmetric() {
						return Err(KeyboardError::SymmetryError(i, j, symm_lp.row_index, symm_lp.col_index));
					} else {
						continue;
					}
				}
				if  !key.is_moveable() {
					continue;
				}
				if let Some(random_keycode) = valid_keycodes.choose(rng) {
					let replacement_key = KeycodeKey::from_keycode(*random_keycode);
					self.set(i, j, replacement_key);
				}
			}
		}
		Ok(())
	}
}
impl<const R: usize, const C: usize> fmt::Display for Layer<R, C, KeycodeKey> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in self.layer.rows_iter() {
			for element in row {
				write!(f, "{}", element);
				write!(f, " ");
			}
			writeln!(f);
		}
		write!(f, "")
    }
}


#[cfg(test)]
mod tests {
	use super::*;

	// don't test things with square dimensions as doing so makes it easier for incorrect logic to still give the expected outcome
	#[test]
	fn test_keycode_key_layer() {
		let l = LayoutPosition::for_layer(0, 1);
		let key1: KeycodeKey = KeycodeKey::from_keycode(_A);
		let key2: KeycodeKey = KeycodeKey::from_keycode(_B);
		let key3: KeycodeKey = KeycodeKey::from_keycode(_C);
		let key4: KeycodeKey = KeycodeKey::from_keycode(_D);
		let key5: KeycodeKey = KeycodeKey::from_keycode(_E);
		let key1again = key1.clone();
		let vec_vec_layer: Vec<Vec<KeycodeKey>> = vec![vec![key1, key2, key3], vec![key5, key4, key1again]];
		let expected_layer: Layer::<2, 3, KeycodeKey> = Layer::<2, 3, KeycodeKey> { layer: Array2D::from_rows(&vec_vec_layer).unwrap() };
		let expected_layer_again = expected_layer.clone();
		fn from_rows_test(l: Vec<Vec<KeycodeKey>>, e: Layer<2, 3, KeycodeKey>) {
			assert_eq!(Layer::<2, 3, KeycodeKey>::from_rows(&l).unwrap(), e);
		}
		from_rows_test(vec_vec_layer, expected_layer);
		fn access_test(e: Layer<2, 3, KeycodeKey>, l: LayoutPosition, k: KeycodeKey) {
			assert_eq!(e.get_from_layout_position(&l).unwrap(), k);
		}
		access_test(expected_layer_again, l, KeycodeKey::from_keycode(_B));
	}

	#[test]
	fn test_float_layer() {
		let expected_layer = Layer::<1, 2, f32> { layer: Array2D::from_rows(&vec![vec![0.4, 0.5]]).unwrap() };
		assert_eq!(expected_layer.get_from_layout_position(&LayoutPosition::for_layer(0, 0)).unwrap(), 0.4);
	}

	#[test]
	fn test_init_random() {
		let mut rng = StdRng::seed_from_u64(0);
		let random_layer = Layer::<2, 3, KeycodeKey>::init_blank();
		assert_eq!(random_layer.get(0, 0).unwrap().value(), _NO);
	}

	#[test]
	fn test_symmetry() {
		let layer = Layer::<4, 6, KeycodeKey>::init_blank();
		let query_layout_pos = LayoutPosition { layer_index: 0, row_index: 2, col_index: 5 };
		let expected_layout_pos = LayoutPosition { layer_index: 0, row_index: 2, col_index: 0 };
		assert_eq!(layer.symmetric_position(query_layout_pos.clone()), expected_layout_pos.clone());
		assert_eq!(layer.symmetric_position(expected_layout_pos.clone()), query_layout_pos.clone());
	}

	#[test]
	fn test_randomize() {
		let mut rng = StdRng::seed_from_u64(0);
		let mut layer = Layer::<2, 2, KeycodeKey>::init_blank();
		// let mut target_key = layer.get(0, 0).unwrap();
		// target_key.set_is_symmetric(true);
		// layer.set(0, 0, target_key);
		layer.get_mut(0, 0).unwrap().set_is_symmetric(true);
		assert_eq!(layer.randomize(&mut rng, vec![_E]).unwrap_err(), KeyboardError::SymmetryError(0, 0, 0, 1));
		layer.get_mut(0, 1).unwrap().set_is_symmetric(true);
		layer.get_mut(1, 1).unwrap().set_is_moveable(false);
		layer.randomize(&mut rng, vec![_E]);
		println!("{}", layer);
		assert_eq!(layer.get(0, 0).unwrap().value(), _NO);
		assert_eq!(layer.get(0, 1).unwrap().value(), _NO);
		assert_eq!(layer.get(1, 1).unwrap().value(), _NO);
		assert_eq!(layer.get(1, 0).unwrap().value(), _E);
		
	}
}