//! obj-2-three converter
//!
//! # Examples
//!
//! ```
//! //
//! ```

/// A structure to manipulate one vertex composed of its `x`, `y` and `z` components.
#[derive(PartialEq, Copy, Clone)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

/// A structure that represent a box, composed of two vertices `min` and `max`.
/// This structure has been written mainly to reprensent a bounding box
/// (see [calculate_bounding_box](./fn.calculate_bounding_box.html) for details)
/// but may be used for other purpose as well.
#[derive(PartialEq, Copy, Clone)]
pub struct Box {
	pub min: Vertex,
	pub max: Vertex
}

/// This function calculate the bounding box of the
/// given vertices array.
///
/// # Examples
///
/// ```
/// use converter::{Vertex, Box, calculate_bounding_box};
///
/// let empty_vertices = [];
/// let vertices = [
/// 	Vertex { x: 0.0,  y: 0.0, z: 0.0 },
/// 	Vertex { x: -1.1, y: 1.1, z: 1.1 },
/// 	Vertex { x: 2.2,  y: 0.0, z: 1.1 },
/// 	Vertex { x: 1.1,  y: 0.0, z: 2.2 }
/// ];
/// 
/// let empty_bounding_box = Box {
/// 	min: Vertex { x: 0.0, y: 0.0, z: 0.0 },
/// 	max: Vertex { x: 0.0, y: 0.0, z: 0.0 }
/// };
/// let bounding_box = Box {
/// 	min: Vertex { x: -1.1, y: 0.0, z: 0.0 },
/// 	max: Vertex { x: 2.2, y: 1.1, z: 2.2 } 
/// };
/// 
/// assert!(bounding_box 		== calculate_bounding_box(&vertices)		);
/// assert!(empty_bounding_box 	== calculate_bounding_box(&empty_vertices)	);
/// ```
pub fn calculate_bounding_box(vertices: &[Vertex]) -> Box {
	
	if vertices.len() > 0 {
	
		let mut bounding_box = Box {
			min: vertices[0],
			max: vertices[0]
		};
		
		for vertex in &vertices[1..] {
		
			if vertex.x < bounding_box.min.x {
				bounding_box.min.x = vertex.x;
			} else if vertex.x > bounding_box.max.x {
				bounding_box.max.x = vertex.x;
			}
			
			if vertex.y < bounding_box.min.y {
				bounding_box.min.y = vertex.y;
			} else if vertex.y > bounding_box.max.y {
				bounding_box.max.y = vertex.y;
			}
			
			if vertex.z < bounding_box.min.z {
				bounding_box.min.z = vertex.z;
			} else if vertex.z > bounding_box.max.z {
				bounding_box.max.z = vertex.z;
			}
			
		}
		
		return bounding_box;
	}
	
	return Box {
		min: Vertex { x: 0.0, y: 0.0, z: 0.0 },
		max: Vertex { x: 0.0, y: 0.0, z: 0.0 }
	};
	
}

/// The test module of the converter
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_calculate_bounding_box() {
	
		let empty_vertices = [];
		let vertices = [
			Vertex { x: 0.0,  y: 0.0, z: 0.0 },
			Vertex { x: -1.1, y: 1.1, z: 1.1 },
			Vertex { x: 2.2,  y: 0.0, z: 1.1 },
			Vertex { x: 1.1,  y: 0.0, z: 2.2 }
		];
	
		let empty_bounding_box = Box {
			min: Vertex { x: 0.0, y: 0.0, z: 0.0 },
			max: Vertex { x: 0.0, y: 0.0, z: 0.0 }
		};
		let bounding_box = Box {
			min: Vertex { x: -1.1, y: 0.0, z: 0.0 },
			max: Vertex { x: 2.2, y: 1.1, z: 2.2 } 
		};
		
		assert!(bounding_box 		== calculate_bounding_box(&vertices)		);
		assert!(empty_bounding_box 	== calculate_bounding_box(&empty_vertices)	);
	}
	
}
