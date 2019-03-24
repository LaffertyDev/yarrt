# Yet Another Rust Raytracer (yarrt)

This is built using Peter Shirley's [Raytracing in a Weekend](https://github.com/petershirley/raytracinginoneweekend)

Adapted into Rust.

## Data Structure

One significant divergance that all Rust Raytracers have is in the data structure. The guide has a recursive structure:

`World` owns `Hitables` which owns `Materials`. When a ray creates a `HitRecord` (when it collides with an object) that `HitRecord` must know about the `Material` that it hit.

In Rust this doesn't work because of very strict ownership rules. There's a few ways to design the structure:

1. ECS

Build a loosely connected structure of entities related by some `Id`. The `EntityStore` is implemented in HashMaps to make lookups quick and painless. The `EntityStore` owns all references and data structures simply contain the Ids to the Entities.

We load all of the Geometry and Materials into their related entities, and then we have a single "System" responsible for casting rays. When we go to create a "HitRecord" we simply store the Ids to the entities necessary.

2. Lifetimes

Unique to Rust (and to help with Ownership) we can specify non-mutable references. Since none of our created entities (`Hitables` and `Materials`) actually mutate after creation, we could use read-only references in the `HitRecord` structure. To do *that*, we would need to specify a `HitRecord` has the same lifetime as a `Ray`.

## Todo

1. Ray could take a reference to the Origin
2. Rename "Hitable" to "Geometry"
3. Move main code to a lib function
4. add input arguments to the main for camera settings
5. Maybe add in scene loading to render a blender scene file?
6. multithreaded to speed processing

Book-recommended improvements
1. lighting
2. triangle geometry support
3. textures
4. "solid textures"
5. "volumes and media"