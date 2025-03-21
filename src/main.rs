use clap::{Parser, value_parser};
use csgrs::csg::CSG;

// Define the command line arguments
#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    len_side: f64,

    #[arg(short, long, default_value = "1", help = "The number of cubes to create")]
    cube_count: usize,

    #[arg(short, long, default_value = "0.0", help = "The minimum diameter of the tube in mm, 0 for no tube")]
    min_tube_diameter: f64,

    #[arg(short, long, default_value = "0.0", help = "The number mm's to increase the tube diameter by when there are multiple cubes")]
    tube_diameter_step: f64,

    #[arg(short, long, default_value = "50", value_parser = value_parser!(u32).range(3..), help = "The number of segments to use when creating the tube, minimum is 3")]
    segments: u32,

    #[arg(short, long, help = "Don't include text for the tube diameter on the object")]
    no_diameter_text: bool,
}

/// Create a cube with an optional tube in the center.
/// The tube is created by removing the material defined by the tube from the cube.
///
/// # Arguments
/// * `len_side` - The length of the sides of the cube
/// * `tube_diameter` - The diameter of the tube to create in the center of the cube, 0.0 for no tube
/// * `segments` - The number of segments to use when creating the tube, minimum is 3
/// * `no_diameter_text` - True if the tube diameter text should NOT be included on the object
fn create_cube(len_side: f64, tube_diameter: f64, segments: u32, no_diameter_text: bool) -> CSG<()> {
    if segments < 3 {
        panic!("segments must be 3 or greater");
    }

    // Create the cube
    let mut cube = CSG::cube(len_side, len_side, len_side, None);

    // Create the tube and translate it to the center of the cube
    if tube_diameter > 0.0 {
        // Create the tube and remove the material it's from the cube
        let tube_radius = tube_diameter / 2.0;
        let tube = CSG::cylinder(tube_radius, len_side, segments as usize, None);
        let tube = tube.translate(len_side / 2.0, len_side / 2.0, 0.0);
        cube = cube.difference(&tube);

        if !no_diameter_text {
            // Create the text for the tube diameter
            let font_data = include_bytes!("../fonts/courier-prime-sans/courier-prime-sans.ttf");
            let text = format!("{:3}", (tube_diameter * 1000.0) as usize);
            let csg_text: CSG<()> = CSG::text(&text, font_data, 4.5, None);
            let csg_text_bb = csg_text.bounding_box();
            let csg_text_extents = csg_text_bb.extents();
            println!("cgs_text_extents: {:?}", csg_text_extents);

            let text_extrude = 0.1;
            let text_3d = csg_text.extrude(text_extrude);

            // Rotate the text to be on the xz plane
            let text_3d = text_3d.rotate(90.0, 0.0, 0.0);

            // Position the text in the center of face on xz plane
            // and sink 10% of the extrude depth into the cube to
            // be sure there are no holes in the print caused by
            // the text not being exactly on the surface.
            let half_len_side = len_side / 2.0;
            let half_extents_y = csg_text_extents.y / 2.0;
            let half_extents_x = csg_text_extents.x / 2.0;
            let text_sink_depth = text_extrude * 0.10;
            let text_3d = text_3d.translate(
                half_len_side - half_extents_x,
                -text_sink_depth,
                half_len_side - half_extents_y,
            );

            // Union the cube with the text
            cube = cube.union(&text_3d);
        }
    }

    // Return the finished cube
    cube
}

fn main() {
    let args = Args::parse();

    for cube_idx in 0..args.cube_count {
        let tube_diameter =
            args.min_tube_diameter + (cube_idx as f64 * args.tube_diameter_step);
        let cube_with_tube = create_cube(args.len_side, tube_diameter, args.segments, args.no_diameter_text,
        );

        let cube_idx_str = if args.cube_count > 1 {
            format!("-{}", cube_idx)
        } else {
            "".to_string()
        };

        // Write the result as an ASCII STL:
        let name = if tube_diameter > 0.0 {
            format!(
                "cube-with-tube{}.len_side-{:0.3}_tube_diameter-{:0.3}_segments-{}",
                cube_idx_str, args.len_side, tube_diameter, args.segments
            )
        } else {
            format!("cube{}.len_side-{:0.3}", cube_idx_str, args.len_side)
        };
        let stl = cube_with_tube.to_stl_ascii(&name);
        std::fs::write(name.to_owned() + ".stl", stl).unwrap();
    }
}
