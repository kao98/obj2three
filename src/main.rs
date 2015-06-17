/* 
Convert Wavefront OBJ / MTL files into Three.js (JSON model version, to be used with ascii / binary loader)
-------------------------
How to use this converter
-------------------------
obj2three -i infile.obj -o outfile.js [-m "morphfiles*.obj"] [-c "morphcolors*.obj"] [-a center|centerxz|top|bottom|none] [-s smooth|flat] [-t ascii|binary] [-d invert|normal] [-b] [-e]
Notes:
    - flags
        -i infile.obj			input OBJ file
        -o outfile.js			output JS file
        -m "morphfiles*.obj"	morph OBJ files (can use wildcards, enclosed in quotes multiple patterns separate by space)
        -c "morphcolors*.obj"	morph colors OBJ files (can use wildcards, enclosed in quotes multiple patterns separate by space)
        -a center|centerxz|top|bottom|none model alignment
        -s smooth|flat			smooth = export vertex normals, flat = no normals (face normals computed in loader)
        -t ascii|binary			export ascii or binary format (ascii has more features, binary just supports vertices, faces, normals, uvs and materials)
        -d invert|normal		invert transparency
        -b						bake material colors into face colors
        -x 10.0                 scale and truncate
        -f 2                    morph frame sampling step
    - by default:
        use smooth shading (if there were vertex normals in the original model)
        will be in ASCII format
        original model is assumed to use non-inverted transparency / dissolve (0.0 fully transparent, 1.0 fully opaque)
        no face colors baking
        no scale and truncate
        morph frame step = 1 (all files will be processed)
    - binary conversion will create two files:
        outfile.js  (materials)
        outfile.bin (binary buffers)
--------------------------------------------------
How to use generated JS file in your HTML document
--------------------------------------------------
    <script type="text/javascript" src="Three.js"></script>
    ...
    <script type="text/javascript">
        ...
        // load ascii model
        var jsonLoader = new THREE.JSONLoader();
        jsonLoader.load( "Model_ascii.js", createScene );
        // load binary model
        var binLoader = new THREE.BinaryLoader();
        binLoader.load( "Model_bin.js", createScene );
        function createScene( geometry, materials ) {
            var mesh = new THREE.Mesh( geometry, new THREE.MeshFaceMaterial( materials ) );
        }
        ...
    </script>
-------------------------------------
Parsers based on formats descriptions
-------------------------------------
    http://en.wikipedia.org/wiki/Obj
    http://en.wikipedia.org/wiki/Material_Template_Library
-------------------
Current limitations
-------------------
    - for the moment, only diffuse color and texture are used
      (will need to extend shaders / renderers / materials in Three)
    - texture coordinates can be wrong in canvas renderer
      (there is crude normalization, but it doesn't
       work for all cases)
    - smoothing can be turned on/off only for the whole mesh
----------------------------------------------
How to get proper OBJ + MTL files with Blender
----------------------------------------------
    0. Remove default cube (press DEL and ENTER)
    1. Import / create model
    2. Select all meshes (Select -> Select All by Type -> Mesh)
    3. Export to OBJ (File -> Export -> Wavefront .obj)
        - enable following options in exporter
            Material Groups
            Rotate X90
            Apply Modifiers
            High Quality Normals
            Copy Images
            Selection Only
            Objects as OBJ Objects
            UVs
            Normals
            Materials
        - select empty folder
        - give your exported file name with "obj" extension
        - click on "Export OBJ" button
    4. Your model is now all files in this folder (OBJ, MTL, number of images)
        - this converter assumes all files staying in the same folder,
          (OBJ / MTL files use relative paths)
        - for WebGL, textures must be power of 2 sized
------
Author
------
Original author: AlteredQualia http://alteredqualia.com
Rust port: Kao ..98 https://github.com/kao98/obj2three
*/

extern crate argparse;

use std::str::FromStr;
use std::fmt;

use argparse::{ArgumentParser, Store};

enum Alignment {
	Center,
	Centerxz,
	Top,
	Bottom,
	None
}

impl FromStr for Alignment {
	type Err = ();
	fn from_str(src: &str) -> Result<Alignment, ()> {
		return match src {
			"center" => Ok(Alignment::Center),
			"centerxz" => Ok(Alignment::Centerxz),
			"top" => Ok(Alignment::Top),
			"bottom" => Ok(Alignment::Bottom),
			"none" => Ok(Alignment::None),
			_ => Err(())
		}
	}
}

impl fmt::Display for Alignment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Alignment::Center => write!(f, "center"),
			&Alignment::Centerxz => write!(f, "centerxz"),
			&Alignment::Top => write!(f, "top"),
			&Alignment::Bottom => write!(f, "bottom"),
			&Alignment::None => write!(f, "none")
		}
	}
}

struct Options {
	input: 			String,
	output: 		String,
	morph_files: 	String,
	morph_colors: 	String,
	alignment: 		Alignment
}

fn main() {

	let mut options = Options {
		input: 			"".to_string(),
		output: 		"".to_string(),
		morph_files: 	"".to_string(),
		morph_colors: 	"".to_string(),
		alignment: 		Alignment::None
	};
	
	{ // this block limits scope of borrows by ap.refer() method
		let mut ap = ArgumentParser::new();

		ap.set_description("\
			Convert Wavefront OBJ / MTL files into Three.js \
			(JSON model version, to be used with ascii / binary loader). \
			For more help please visit https://github.com/kao98/obj2three\
			or read the README file.\
		");
		
		ap
			.refer(&mut options.input)
			.required()
			.add_option(
				&["-i", "--infile"],
				Store,
				"Input OBJ file"
			)
		;
		
		ap
			.refer(&mut options.output)
			.required()
			.add_option(
				&["-o", "--outfile"],
				Store,
				"Output JS file"
			)
		;
		
		ap
			.refer(&mut options.morph_files)
			.add_option(
				&["-m"],
				Store,
				"morph OBJ files (can use wildcards, enclosed in quotes multiple patterns separate by space)"
			)
		;
		
		ap
			.refer(&mut options.morph_colors)
			.add_option(
				&["-c"],
				Store,
				"morph colors OBJ files (can use wildcards, enclosed in quotes multiple patterns separate by space)"
			)
		;
		
		ap
			.refer(&mut options.alignment)
			.add_option(
				&["-a"],
				Store,
				"center|centerxz|top|bottom|none model alignment"
			)
		;
				
		ap.parse_args_or_exit();
	}
	
	println!("{}", options.input);
	println!("{}", options.output);
	println!("{}", options.morph_files);
	println!("{}", options.morph_colors);
	println!("{}", options.alignment);
}
