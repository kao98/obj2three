
fn calculate_bounding_box(vertices: &[(f32, f32, f32)]) -> ([f32; 2], [f32; 2], [f32; 2]) {
	
	if vertices.len() > 0 {
		let (mut minx, mut maxx) = (vertices[0].0, vertices[0].0);
		let (mut miny, mut maxy) = (vertices[0].1, vertices[0].1);
		let (mut minz, mut maxz) = (vertices[0].2, vertices[0].2);
		
		for vertex in &vertices[1..] {
		
			if vertex.0 < minx {
				minx = vertex.0;
			} else if vertex.0 > maxx {
				maxx = vertex.0;
			}
			
			if vertex.1 < miny {
				miny = vertex.1;
			} else if vertex.1 > maxy {
				maxy = vertex.1;
			}
			
			if vertex.2 < minz {
				minz = vertex.2;
			} else if vertex.2 > maxz {
				maxz = vertex.2;
			}
			
		}
		
		return ([minx, maxx], [miny, maxy], [minz, maxz]);
	}
	
	return ([0.0, 0.0], [0.0, 0.0], [0.0, 0.0]);
}

#[test]
fn test_calculate_bounding_box() {
	let vertices = [
		(0.0, 0.0, 0.0),
		(1.1, 1.1, 1.1),
		(2.2, 0.0, 1.1),
		(1.1, 0.0, 2.2)
	];
	let empty_vertices = [];
	
	let bounding_box 		= ([0.0, 2.2], [0.0, 1.1], [0.0, 2.2]);
	let empty_bounding_box 	= ([0.0, 0.0], [0.0, 0.0], [0.0, 0.0]);
	
	assert!(bounding_box 		== calculate_bounding_box(&vertices)		);
	assert!(empty_bounding_box 	== calculate_bounding_box(&empty_vertices)	);
}