# iso
A Rust implementation of the marching cubes algorithm for extracting isosurfaces from 3D volumetric data.

## Features
- Fast and efficient
- Commonly used in computer graphics, medical imaging, and scientific visualization

## Examples
```rs
markdown

# iso
A Rust implementation of the marching cubes algorithm for extracting isosurfaces from 3D volumetric data.

## Features
- Fast and efficient
- Commonly used in computer graphics, medical imaging, and scientific visualization

## Examples

use iso::{MarchingCubes, GridCell, Triangle};

let grid = GridCell {
    positions: [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0]
    ],
    value: [0.0, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0],
};

let mut triangles = vec![];

let isolevel = 0.5;
let mc = MarchingCubes::new(isolevel, grid);
let triangle_count = mc.polygonise(&mut triangles);

assert_eq!(triangle_count, 4);
```

## Usage
The `MarchingCubes` struct is the main entry point to the library.
- The `new` method creates a new instance of the algorithm and takes two arguments: the 3D volumetric data as a nested array of scalar values and the isovalue used to extract the isosurface.
- The `polygonise` method executes the algorithm and returns the resulting vertices as a vector of 3D points.

Note: The example code uses a 4x4x4 volume for simplicity, but in practice the volume size can be much larger and the algorithm will scale accordingly.