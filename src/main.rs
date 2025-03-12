use std::env;
use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Check for the correct number of command line arguments
    if env::args().len() != 5 {
        eprintln!("Usage: cube-with-tube len_side tube_diameter tube_segments");
        std::process::exit(1);
    }

    // Parse command line arguments to get the spindle dimensions
    let args: Vec<String> = env::args().collect();
    let len_side= args[1].parse::<f64>().unwrap();
    let tube_diameter= args[2].parse::<f64>().unwrap();
    let segments= args[4].parse::<usize>().unwrap();

    let cube= CSG::cube(len_side, len_side, len_side, None);

    // Create the tube and translate it to the center of the cube
    let tube_radius = tube_diameter / 2.0;
    let tube = CSG::cylinder(tube_radius, len_side, segments, None);  // 1 x 20 cylinder
    let tube = tube.translate(Vector3::new(len_side / 2.0, len_side / 2.0, 0.0));

    // Remove the material from the cube to create the tube
    let cube_with_tube = cube.difference(&tube);

    // Write the result as an ASCII STL:
    let name = &format!("cube-with-tube.len_side-{:0.3}_tub_diameter-{:0.3}_segments-{:}", len_side, tube_diameter, segments);
    let stl = cube_with_tube.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
