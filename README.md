# Cube with a tube in the center

Create cubes with a tube in the center

## Install

```
cargo install --path .
```
## Usage

```
$ cube-with-tube
Usage: cube-with-tube len_side tube_diameter segments cube_count tube_diameter_step
```

## Run

### Create one cube

Create one cube with a tube in the center with the following dimensions:
```
$ cargo run 3 0.561 50 1 0
```

or if installed

```
$ cargo-with-tube 3 0.561 50 1 0
```

Display the spindle in a 3D viewer.
```

$ f3d cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl
```

Create a png image of the spindle.
```
$ f3d cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl --output cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl.png 
```

![cargo-with-tube -- 3 0.561 50 1 0](./cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl.png)

### Print mulitple cubes with tubes in the center

Here we create 5 cubes with a 2.397 side length and a tube diameter of 0.561.
The tube diameter is increased by 0.017 which is the resolution of my printer.

I've chosen the physical dimension numbers 2.397, 0.561 and 0.017 as they are
multiples of 0.017, which is the resolution of the 3D printer.
The range of diameters is 0.561 to 0.629 and one of these should nicely fit
straight pin in the tube. The pin with a diameter of 0.629 was loose at first
but after two days the it is not a little tight. So apparently it has shrunk
a little. These have not yet been cured with UV light so I need to do that
and also do it right after washing to see/feel the difference.

Also, I need to actually measure the pin diameters there are at least two
different sizes. The pinkish pins are slightly smaller in diameter than the
other pins.

```
cargo run 2.397 0.561 50 5 0.017
```

Here are some pics of the cubes taken at 2x (focal len 2.4 mm) optical zoom with my Pixel 7a.

This is a pic via a microscope using the 10x objective and 2x
primary lens, IIRC.
![cube-595-612-620](./cubes-595-612-629.jpg)

5 cubes on 1 pin:
![cubes-5-on-1-pin](./cubes-5-on-1-pin.jpg)

5 cubes on 5 pins:
![cubes-5-on-5-pins](./cubes-5-on-5-pins.jpg)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
