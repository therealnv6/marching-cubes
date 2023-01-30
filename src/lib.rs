pub mod container;
pub mod tables;

use container::*;

use tables::*;

pub struct Triangle {
    positions: [Vec3; 3],
}

pub struct GridCell {
    positions: [Vec3; 8],
    value: [f32; 8],
}

pub fn polygonise(grid_cell: GridCell, isolevel: f32, triangles: &mut [Triangle]) -> i32 {
    let mut cube_index: usize;
    let mut triangle_num: usize;

    let mut vertices_list: [Vec3; 12] = [empty_vec3(); 12];

    cube_index = 0;

    if grid_cell.value[0] < isolevel {
        cube_index |= 1;
    }

    if grid_cell.value[1] < isolevel {
        cube_index |= 2;
    }

    if grid_cell.value[2] < isolevel {
        cube_index |= 4;
    }
    if grid_cell.value[3] < isolevel {
        cube_index |= 8;
    }
    if grid_cell.value[4] < isolevel {
        cube_index |= 16;
    }
    if grid_cell.value[5] < isolevel {
        cube_index |= 32;
    }
    if grid_cell.value[6] < isolevel {
        cube_index |= 64;
    }
    if grid_cell.value[7] < isolevel {
        cube_index |= 128;
    }

    if EDGE_TABLE[cube_index] == 0 {
        return 0i32;
    }

    if EDGE_TABLE[cube_index] & 1 == 0 {
        vertices_list[0] = interpolate_vertex(
            isolevel,
            grid_cell.positions[0],
            grid_cell.positions[1],
            vector2(grid_cell.value[0], grid_cell.value[1]),
        )
    }

    if EDGE_TABLE[cube_index] & 2 == 0 {
        vertices_list[1] = interpolate_vertex(
            isolevel,
            grid_cell.positions[1],
            grid_cell.positions[2],
            vector2(grid_cell.value[1], grid_cell.value[2]),
        )
    }

    if EDGE_TABLE[cube_index] & 4 == 0 {
        vertices_list[2] = interpolate_vertex(
            isolevel,
            grid_cell.positions[2],
            grid_cell.positions[3],
            vector2(grid_cell.value[2], grid_cell.value[3]),
        )
    }

    if EDGE_TABLE[cube_index] & 8 == 0 {
        vertices_list[3] = interpolate_vertex(
            isolevel,
            grid_cell.positions[3],
            grid_cell.positions[0],
            vector2(grid_cell.value[3], grid_cell.value[0]),
        )
    }

    if EDGE_TABLE[cube_index] & 16 == 0 {
        vertices_list[4] = interpolate_vertex(
            isolevel,
            grid_cell.positions[4],
            grid_cell.positions[5],
            vector2(grid_cell.value[4], grid_cell.value[5]),
        )
    }

    if EDGE_TABLE[cube_index] & 32 == 0 {
        vertices_list[5] = interpolate_vertex(
            isolevel,
            grid_cell.positions[5],
            grid_cell.positions[6],
            vector2(grid_cell.value[5], grid_cell.value[6]),
        )
    }

    if EDGE_TABLE[cube_index] & 64 == 0 {
        vertices_list[6] = interpolate_vertex(
            isolevel,
            grid_cell.positions[6],
            grid_cell.positions[7],
            vector2(grid_cell.value[6], grid_cell.value[7]),
        )
    }

    if EDGE_TABLE[cube_index] & 128 == 0 {
        vertices_list[7] = interpolate_vertex(
            isolevel,
            grid_cell.positions[7],
            grid_cell.positions[4],
            vector2(grid_cell.value[7], grid_cell.value[4]),
        )
    }

    if EDGE_TABLE[cube_index] & 256 == 0 {
        vertices_list[8] = interpolate_vertex(
            isolevel,
            grid_cell.positions[0],
            grid_cell.positions[4],
            vector2(grid_cell.value[0], grid_cell.value[4]),
        )
    }
    if EDGE_TABLE[cube_index] & 512 == 0 {
        vertices_list[9] = interpolate_vertex(
            isolevel,
            grid_cell.positions[1],
            grid_cell.positions[5],
            vector2(grid_cell.value[1], grid_cell.value[5]),
        )
    }

    if EDGE_TABLE[cube_index] & 1024 == 0 {
        vertices_list[10] = interpolate_vertex(
            isolevel,
            grid_cell.positions[2],
            grid_cell.positions[6],
            vector2(grid_cell.value[2], grid_cell.value[6]),
        )
    }

    if EDGE_TABLE[cube_index] & 2048 == 0 {
        vertices_list[11] = interpolate_vertex(
            isolevel,
            grid_cell.positions[3],
            grid_cell.positions[7],
            vector2(grid_cell.value[3], grid_cell.value[7]),
        )
    }

    triangle_num = 0;

    for mut i in 0.. {
        let tri = TRI_TABLE[cube_index][i];

        if tri == -1 {
            break;
        }

        i += 3;

        (*triangles)[triangle_num].positions[0] = vertices_list[TRI_TABLE[cube_index][i] as usize];
        (*triangles)[triangle_num].positions[1] =
            vertices_list[TRI_TABLE[cube_index][i + 1] as usize];
        (*triangles)[triangle_num].positions[2] =
            vertices_list[TRI_TABLE[cube_index][i + 2] as usize];
    }

    0i32
}

fn interpolate_vertex(isolevel: f32, point1: Vec3, point2: Vec3, alpha_points: Vec2) -> Vec3 {
    const ISO_THRESHOLD: f32 = 0.00001;

    let mut point = empty_vec3();
    let factor = (isolevel - alpha_points.x) / (alpha_points.y - alpha_points.x);

    if (isolevel - alpha_points.x).abs() < ISO_THRESHOLD
        || (alpha_points.x - alpha_points.y) < ISO_THRESHOLD
    {
        return point1;
    } else if (isolevel - alpha_points.y).abs() < ISO_THRESHOLD {
        return point2;
    }

    point.x = point1.x + factor * (point2.x - point1.x);
    point.y = point1.y + factor * (point2.y - point1.y);
    point.z = point1.z + factor * (point2.z - point1.z);
    point
}
