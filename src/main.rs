use std::env;
use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Check for the correct number of command line arguments
    if env::args().len() != 5 {
        eprintln!("Usage: cube-with-tube len_side tube_diameter tube_wall_thickness tub_segments");
        std::process::exit(1);
    }

    // Parse command line arguments to get the spindle dimensions
    let args: Vec<String> = env::args().collect();
    let len_side= args[1].parse::<f64>().unwrap();
    let tube_diameter= args[2].parse::<f64>().unwrap();
    let tube_wall_thickness = args[3].parse::<f64>().unwrap();
    let segments= args[4].parse::<usize>().unwrap();

    let cube= CSG::cube(len_side, len_side, len_side, None);

    // Create cylinder for the tube
    let outer_radius = (tube_diameter + tube_wall_thickness) / 2.0;
    let inner_radius = tube_diameter / 2.0;
    let cylinder = CSG::cylinder(outer_radius, len_side, segments, None);  // 1 x 20 cylinder

    // Move the cylinder to the center of the cube
    let cylinder = cylinder.translate(Vector3::new(len_side / 2.0, len_side / 2.0, 0.0));

    // Combine the cube and the cylinder
    let cube_and_cylinder = cube.union(&cylinder);

    // This will be the hole in the cube
    let tube_hole= CSG::cylinder(inner_radius, len_side, segments, None);  // 1 x 20 cylinder

    // Move the tube_hole to the center of the cube
    let tube_hole = tube_hole.translate(Vector3::new(len_side / 2.0, len_side / 2.0, 0.0));

    // Remove the material from the cube_and_tube
    let cube_with_hole = cube_and_cylinder.difference(&tube_hole);

    // Write the result as an ASCII STL:
    let name = &format!("cube-with-tube.len_side{:0.3}_tub_diameter{:0.3}_tube_wall_thickness{:0.2}_segments{:}", len_side, tube_diameter, tube_wall_thickness, segments);
    let stl = cube_with_hole.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
