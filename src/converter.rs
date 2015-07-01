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

/// A function to compare floating point numbers using a tolerance value.
#[allow(dead_code)]
pub fn fuzzy_cmp(a: f32, b: f32, tolerance: f32) -> bool {
	a >= b - tolerance && a <= b + tolerance
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

/// This function translate the given vertices by the given translation vector
///
/// # Examples
///
/// ```
/// use converter::{fuzzy_cmp, translate};
/// 
/// let mut vertices = [
/// 	Vertex { x: 0.0,  y: 0.0, z: 0.0 },
/// 	Vertex { x: -1.1, y: 1.1, z: 1.1 },
/// 	Vertex { x: 2.2,  y: 0.0, z: 1.1 },
/// 	Vertex { x: 1.1,  y: 0.0, z: 2.2 }
/// ];
/// 
/// let translated_vertices = [
/// 	Vertex { x: 1.0,  y: -2.0, z: 3.3 },
/// 	Vertex { x: -0.1, y: -0.9, z: 4.4 },
/// 	Vertex { x: 3.2,  y: -2.0, z: 4.4 },
/// 	Vertex { x: 2.1,  y: -2.0, z: 5.5 }
/// ];
/// 
/// let translation_vector = [1.0, -2.0, 3.3];
/// 
/// translate(&mut vertices, &translation_vector);
/// 
/// for it in vertices.iter_mut().zip(translated_vertices.iter()) {
/// 	let (vertex, translated_vertex) = it;
/// 	assert!(
/// 		fuzzy_cmp(vertex.x, translated_vertex.x, 0.01) &&
/// 		fuzzy_cmp(vertex.y, translated_vertex.y, 0.01) &&
/// 		fuzzy_cmp(vertex.z, translated_vertex.z, 0.01)
/// 	);
/// }
/// ```
pub fn translate(vertices: &mut [Vertex], translation_vector: &[f32; 3]) {

	for vertex in vertices {
		vertex.x += translation_vector[0];
		vertex.y += translation_vector[1];
		vertex.z += translation_vector[2];
	}
	
}

/// This function center the given vertices on the middle of the bounding box
/// 
/// # Examples
///
/// ```
/// let mut vertices = [
/// 	Vertex { x: 0.0,  y: 0.0, z: 0.0 },
/// 	Vertex { x: 2.0,  y: 2.0, z: 2.0 },
/// 	Vertex { x: 4.0,  y: 4.0, z: 4.0 },
/// ];
/// 
/// let translated_vertices = [
/// 	Vertex { x: -2.0,  y: -2.0, z: -2.0 },
/// 	Vertex { x:  0.0,  y:  0.0, z:  0.0 },
/// 	Vertex { x:  2.0,  y:  2.0, z:  2.0 },
/// ];
/// 
/// center(&mut vertices);
/// 
/// assert!(vertices == translated_vertices);
/// ```
pub fn center(vertices: &mut [Vertex]) {
	
	let bounding_box = calculate_bounding_box(vertices);
	
	let cx = bounding_box.min.x + (bounding_box.max.x - bounding_box.min.x) / 2.0;
	let cy = bounding_box.min.y + (bounding_box.max.y - bounding_box.min.y) / 2.0;
	let cz = bounding_box.min.z + (bounding_box.max.z - bounding_box.min.z) / 2.0;
	
	translate(vertices, &[-cx, -cy, -cz]);
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
	
	#[test]
	fn test_translate() {
	
		let mut vertices = [
			Vertex { x: 0.0,  y: 0.0, z: 0.0 },
			Vertex { x: -1.1, y: 1.1, z: 1.1 },
			Vertex { x: 2.2,  y: 0.0, z: 1.1 },
			Vertex { x: 1.1,  y: 0.0, z: 2.2 }
		];
	
		let translated_vertices = [
			Vertex { x: 1.0,  y: -2.0, z: 3.3 },
			Vertex { x: -0.1, y: -0.9, z: 4.4 },
			Vertex { x: 3.2,  y: -2.0, z: 4.4 },
			Vertex { x: 2.1,  y: -2.0, z: 5.5 }
		];
		
		let translation_vector = [1.0, -2.0, 3.3];
		
		translate(&mut vertices, &translation_vector);
		
		for it in vertices.iter_mut().zip(translated_vertices.iter()) {
			let (vertex, translated_vertex) = it;
			assert!(
				fuzzy_cmp(vertex.x, translated_vertex.x, 0.01) &&
				fuzzy_cmp(vertex.y, translated_vertex.y, 0.01) &&
				fuzzy_cmp(vertex.z, translated_vertex.z, 0.01)
			);
		}
	}
	
	#[test]
	fn test_center() {
	
		let mut vertices = [
			Vertex { x: 0.0,  y: 0.0, z: 0.0 },
			Vertex { x: 2.0,  y: 2.0, z: 2.0 },
			Vertex { x: 4.0,  y: 4.0, z: 4.0 },
		];
	
		let translated_vertices = [
			Vertex { x: -2.0,  y: -2.0, z: -2.0 },
			Vertex { x:  0.0,  y:  0.0, z:  0.0 },
			Vertex { x:  2.0,  y:  2.0, z:  2.0 },
		];
		
		center(&mut vertices);
		
		assert!(vertices == translated_vertices);
		
	}
	
}
