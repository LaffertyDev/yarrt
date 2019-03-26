# Yet Another Rust Raytracer (yarrt)

This is built using Peter Shirley's [Raytracing in a Weekend](https://github.com/petershirley/raytracinginoneweekend)

Adapted into Rust. Built to learn about raytracing and build a nontrivial program in Rust. This is a purely academic program with no "real" applications.

## Example Generated File

![Generated Scene](./images/raytraced_random_scene.jpg "Generated Scene")

Scene took ~1 hour to generate using a release build.

## Compiling and Running

Requirements are to have Rust and Cargo installed. Rustup will manage this for you. Download the git project, then...
Test: `cargo test`
Run: `cargo build --release > output.ppm`

## Future Improvements

There's a lot of things I could do to improve raytracer.

Improvements to Rust Code:

1. Ray could take a reference to the Origin
2. Rename "Hitable" to "Geometry"
3. Move main code to a lib function
4. add input arguments to the main for camera settings
  camera position & orientation, aspect ratio, resolution, max_ray_depth, 
5. Maybe add in scene loading to render a blender scene file?

For multithreaded support I'm pretty sure this wouldn't be too difficult. The book recommends to run N copies on the same scene & randomize the results. This would be good to simulate real "light" but isn't going to speed up the compilation times.

Improvements to Performance:

1. multithreaded ray tracing
  Run a thread per X,Y ray. Should decrease execution time linearly
2. simplify random disk/sphere generation to only generate once within a unit-vector
  Currently we generate until we have a unit sphere or disk in these cases. We could generate once to remove the slow loop process.
3. Run 

Improvements to Raytracer:

1. lighting
2. triangle geometry support
3. textures
4. "solid textures"
5. "volumes and media"