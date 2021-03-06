//! obj-2-three converter
//!
//! # Examples
//!
//! ```
//! //
//! ```

use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;

/// A macro to determine the file name in a string representing an absolute path.
/// 
/// # Panics
///
/// Panics if no file name may be extract from the given path, or if
/// the resulting file name contains non-unicode characters.
/// 
/// # Examples
/// 
/// ```
/// use std::path::PathBuf;
/// 
/// assert!(file_name!("/home/user/file.txt") == "file.txt");
/// assert!(file_name!("c:\\users\\user\\file.txt") == "file.txt");
/// ```
macro_rules! file_name {
	($path:expr) => {{
		PathBuf::from($path.replace("\\", "/"))	
			.file_name().unwrap()
			.to_str().unwrap()
	}};
}

/// A structure to manipulate one vertex composed of its `x`, `y` and `z` components.
#[derive(PartialEq, Copy, Clone)]
pub struct Vertex {
	pub x: f64,
	pub y: f64,
	pub z: f64
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
/// This may not be used outside some tests, so we allow dead_code for that one.
#[allow(dead_code)]
pub fn fuzzy_cmp(a: f64, b: f64, tolerance: f64) -> bool {
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
pub fn translate(vertices: &mut [Vertex], translation_vector: &[f64; 3]) {

	for vertex in vertices {
		vertex.x += translation_vector[0];
		vertex.y += translation_vector[1];
		vertex.z += translation_vector[2];
	}
	
}

/// Alignment option for the align function
pub enum align_option {
	/// Align the model on the center of the bounding box
	center,
	/// Align the top of the model with the floor of the bounding box
	top,
	/// Align the bottom of the model with the floor of the bounding box
	bottom,
	/// Center the model on the x and z axis
	xz
}

/// This function align the model on the given direction
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
/// 	Vertex { x: -2.0,  y: -4.0, z: -2.0 },
/// 	Vertex { x:  0.0,  y: -2.0, z:  0.0 },
/// 	Vertex { x:  2.0,  y:  0.0, z:  2.0 },
/// ];
/// 
/// align(&mut vertices, align_option::top);
/// 
/// assert!(vertices == translated_vertices);
/// ```
pub fn align(vertices: &mut [Vertex], direction: align_option) {
	
	let bounding_box = calculate_bounding_box(vertices);
	
	let cx = bounding_box.min.x + (bounding_box.max.x - bounding_box.min.x) / 2.0;
	let cz = bounding_box.min.z + (bounding_box.max.z - bounding_box.min.z) / 2.0;
	
	let cy = match direction {
		align_option::center 	=> bounding_box.min.y + (bounding_box.max.y - bounding_box.min.y) / 2.0,
		align_option::top 		=> bounding_box.max.y,
		align_option::bottom 	=> bounding_box.min.y,
		align_option::xz 		=> 0.0
	};
	
	translate(vertices, &[-cx, -cy, -cz]);
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
	
	align(vertices, align_option::center);
	
}

/// This function align the top of the model with the floor (y-axis) of the bounding box
/// and center it around x and z
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
/// 	Vertex { x: -2.0,  y: -4.0, z: -2.0 },
/// 	Vertex { x:  0.0,  y: -2.0, z:  0.0 },
/// 	Vertex { x:  2.0,  y:  0.0, z:  2.0 },
/// ];
/// 
/// align_top(&mut vertices);
/// 
/// assert!(vertices == translated_vertices);
/// ```
pub fn align_top(vertices: &mut [Vertex]) {
	
	align(vertices, align_option::top);
	
}

/// This function align the bottom of the model with the floor (y-axis) of the bounding box
/// and center it around x and z
/// 
/// # Examples
///
/// ```
/// let mut vertices = [
/// 	Vertex { x: 0.0,  y: -2.0, z: 0.0 },
/// 	Vertex { x: 2.0,  y:  2.0, z: 2.0 },
/// 	Vertex { x: 4.0,  y:  4.0, z: 4.0 },
/// ];
/// 
/// let translated_vertices = [
/// 	Vertex { x: -2.0,  y:  0.0, z: -2.0 },
/// 	Vertex { x:  0.0,  y:  4.0, z:  0.0 },
/// 	Vertex { x:  2.0,  y:  6.0, z:  2.0 },
/// ];
/// 
/// align_bottom(&mut vertices);
/// 
/// assert!(vertices == translated_vertices);
/// ```
pub fn align_bottom(vertices: &mut [Vertex]) {
	
	align(vertices, align_option::bottom);
	
}

/// This function center the model around x and z
/// 
/// # Examples
///
/// ```
/// let mut vertices = [
/// 	Vertex { x: 0.0,  y: -2.0, z: 0.0 },
/// 	Vertex { x: 2.0,  y:  2.0, z: 2.0 },
/// 	Vertex { x: 4.0,  y:  4.0, z: 4.0 },
/// ];
/// 
/// let translated_vertices = [
/// 	Vertex { x: -2.0,  y:  -2.0, z: -2.0 },
/// 	Vertex { x:  0.0,  y:   2.0, z:  0.0 },
/// 	Vertex { x:  2.0,  y:   4.0, z:  2.0 },
/// ];
/// 
/// center_xz(&mut vertices);
/// 
/// assert!(vertices == translated_vertices);
/// ```
pub fn center_xz(vertices: &mut [Vertex]) {
	
	align(vertices, align_option::xz);
	
}

/// This function normalize the given vertex
/// 
/// # Examples
/// 
/// ```
/// use converter::{fuzzy_comp, normalize};
/// 
/// let mut v = Vertex { x: 1.0, y: 1.0, z: 1.0 };
/// 
/// let vn = Vertex { x: 0.57735, y: 0.57735, z: 0.57735 };
/// 
/// normalize(&mut v1);
/// assert!(
/// 	fuzzy_cmp(v1.x, v1n.x, 0.000001) &&
/// 	fuzzy_cmp(v1.y, v1n.y, 0.000001) &&
/// 	fuzzy_cmp(v1.z, v1n.z, 0.000001)
/// );
/// ```
pub fn normalize(vertex: &mut Vertex) {
	
	let lenght = (vertex.x.powi(2) + vertex.y.powi(2) + vertex.z.powi(2)).sqrt();
	
	if lenght.is_normal() {
		vertex.x /= lenght;
		vertex.y /= lenght;
		vertex.z /= lenght;
	}
	 
}

pub fn parse_mtl(file_name: & str) {

	let file = match File::open(file_name) {
		Ok(file) => file,
		Err(e) => panic!("Couldn't open {}", file_name)
	};

	let file = BufReader::new(&file);

	let mut previous_line	:String = String::new();	
	let mut line			:String;

	for current_line in file.lines() {
	
		line = match current_line {
			Ok(current_line) => format!(
					"{}{}",
					previous_line,
					current_line
				),
			Err(_) => previous_line
		};
		
		previous_line = String::new();
		
		let mut iter = line.rsplitn(2, "\\\\");
		
		if iter.next() == Some("") {
			previous_line = match iter.last() {
				Some(line) => String::from(line),
				None => String::new()
			};
		} else {
			let mut chunks = line.splitn(2, ' ');
			//if chunks.count() > 0 {
			let first = chunks.next();
			
			if first != None {
				let first = first.unwrap().trim();
				println!("chunk[0]: {}", first);
				
				match first {
					"newmtl"=> {
						println!("Let's start a new material!\n");
					}
					"map_Kd" => {
						println!("This is a diffuse map.\n");
					}
					_ => println!("Something else:/\n")
				};
				
			}

		}

	}
	
}

fn parse_mtl_line(line: & String) {
	
	let mut chunks = line.splitn(2, ' ');
	
	//if chunks.count() > 0 {
		let first = chunks.next().unwrap();
		println!("chunk[0]: {}", first);
	//}
	/*
	let chunk_count = chunks.clone().count();
	
	println!("chunk count: {}", chunk_count);
	
	for chunk in chunks {
		let mut chunk = chunk;
		/*if chunks_count > 1 {
			chunk = chunk.trim();
		}*/
		
		println!("'{}' ", chunk);
	}
	println!("-----\n");*/
}

/// The test module of the converter
#[cfg(test)]
mod tests {

	use super::*;
	use std::path::PathBuf;

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
	
	#[test]
	fn test_align_top() {
	
		let mut vertices = [
			Vertex { x: 0.0,  y: 0.0, z: 0.0 },
			Vertex { x: 2.0,  y: 2.0, z: 2.0 },
			Vertex { x: 4.0,  y: 4.0, z: 4.0 },
		];
	
		let translated_vertices = [
			Vertex { x: -2.0,  y: -4.0, z: -2.0 },
			Vertex { x:  0.0,  y: -2.0, z:  0.0 },
			Vertex { x:  2.0,  y:  0.0, z:  2.0 },
		];
		
		align_top(&mut vertices);
		
		assert!(vertices == translated_vertices);
		
	}
	
	#[test]
	fn test_align_bottom() {
	
		let mut vertices = [
			Vertex { x: 0.0,  y: -2.0, z: 0.0 },
			Vertex { x: 2.0,  y:  2.0, z: 2.0 },
			Vertex { x: 4.0,  y:  4.0, z: 4.0 },
		];
	
		let translated_vertices = [
			Vertex { x: -2.0,  y: 0.0, z: -2.0 },
			Vertex { x:  0.0,  y: 4.0, z:  0.0 },
			Vertex { x:  2.0,  y: 6.0, z:  2.0 },
		];
		
		align_bottom(&mut vertices);
		
		assert!(vertices == translated_vertices);
		
	}
	
	#[test]
	fn test_center_xz() {
	
		let mut vertices = [
			Vertex { x: 0.0,  y: -2.0, z: 0.0 },
			Vertex { x: 2.0,  y:  2.0, z: 2.0 },
			Vertex { x: 4.0,  y:  4.0, z: 4.0 },
		];
	
		let translated_vertices = [
			Vertex { x: -2.0,  y: -2.0, z: -2.0 },
			Vertex { x:  0.0,  y:  2.0, z:  0.0 },
			Vertex { x:  2.0,  y:  4.0, z:  2.0 },
		];
		
		center_xz(&mut vertices);
		
		assert!(vertices == translated_vertices);
		
	}
	
	#[test]
	fn test_normalize() {
		
		let mut v1 = Vertex { x: 1.0, y: 1.0, z: 1.0 };
		let mut v2 = Vertex { x: 0.0, y: 0.0, z: 0.0 };
		let mut v3 = Vertex { x: 2.0, y: 2.0, z: 3.0 };
		let mut v4 = Vertex { x: -2.0, y: 2.0, z: -3.0 };
		
		let v1n = Vertex { x: 0.57735, y: 0.57735, z: 0.57735 };
		let v2n = Vertex { x: 0.0, y: 0.0, z: 0.0 };
		let v3n = Vertex { x:  0.485071, y: 0.485071, z:  0.727607 };
		let v4n = Vertex { x: -0.485071, y: 0.485071, z: -0.727607 };
		
		normalize(&mut v1);
		normalize(&mut v2);
		normalize(&mut v3);
		normalize(&mut v4);
		
		println!("{} {} {}", v1.x, v1.y, v1.z);
		println!("{} {} {}", v2.x, v2.y, v2.z);
		println!("{} {} {}", v3.x, v3.y, v3.z);
		println!("{} {} {}", v4.x, v4.y, v4.z);		
		
		assert!(
			fuzzy_cmp(v1.x, v1n.x, 0.000001) &&
			fuzzy_cmp(v1.y, v1n.y, 0.000001) &&
			fuzzy_cmp(v1.z, v1n.z, 0.000001)
		);
		assert!(v2n == v2);
		assert!(
			fuzzy_cmp(v3.x, v3n.x, 0.000001) &&
			fuzzy_cmp(v3.y, v3n.y, 0.000001) &&
			fuzzy_cmp(v3.z, v3n.z, 0.000001)
		);
		assert!(
			fuzzy_cmp(v4.x, v4n.x, 0.000001) &&
			fuzzy_cmp(v4.y, v4n.y, 0.000001) &&
			fuzzy_cmp(v4.z, v4n.z, 0.000001)
		);
		
	}
	
	#[test]
	fn test_file_name_macro() {
		assert!(file_name!("/home/user/file.txt") == "file.txt");
		assert!(file_name!("c:\\users\\user\\file.txt") == "file.txt");
	}
	
	#[test]
	#[should_panic]
	fn test_file_name_macro_panic() {
		file_name!("/home/user/..");
	}
}
