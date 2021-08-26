use nalgebra::{vector, Matrix3, Matrix3x1, Point, SVector};

use super::grid::Cell;

pub fn place_surface_vertex(
    cell: Cell,
    resolution: f32,
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    let cell_min = cell.min_position;
    let cell_max = cell_min + vector![resolution, resolution, resolution];

    let point = place_at_plane_intersection(planes);

    if cell_min.x < point.x
        && cell_min.y < point.y
        && cell_min.z < point.z
        && cell_max.x > point.x
        && cell_max.y > point.y
        && cell_max.z > point.z
    {
        point
    } else {
        place_at_average(planes)
    }
}

#[allow(non_snake_case)]
fn place_at_plane_intersection(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // Based on the approach from https://www.mattkeeter.com/projects/qef/.

    let mut AᵀA = Matrix3::zeros();
    let mut AᵀB = Matrix3x1::zeros();

    for (point, normal) in planes {
        AᵀA += normal * normal.transpose();
        AᵀB += normal * (normal.dot(&point.coords));
    }

    // I don't know under which circumstances this can panic, but so far I
    // haven't seen it do so. Let's just hope for the best, and fix it if this
    // ever turns into a problem.
    let result = AᵀA
        .svd(true, true)
        .solve(&AᵀB, 0.1)
        .expect("Failed to solve QEF. This is a bug.");

    result.into()
}

pub fn place_at_average(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    let mut surface_vertex = Point::origin();
    for (point, _) in planes {
        surface_vertex += point.coords;
    }

    surface_vertex / planes.len() as f32
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use super::place_at_plane_intersection;

    #[test]
    fn test_perpendicular_planes() {
        let a = (point![0.5, 0.0, 0.0], vector![1.0, 0.0, 0.0]);
        let b = (point![0.0, 0.5, 0.0], vector![0.0, 1.0, 0.0]);
        let c = (point![0.0, 0.0, 0.5], vector![0.0, 0.0, 1.0]);

        let point = place_at_plane_intersection(&[a, b, c]);
        assert_eq!(point, point![0.5, 0.5, 0.5]);
    }

    // TASK: Un-ignore test.
    #[test]
    #[ignore]
    fn test_parallel_planes() {
        // TASK: Implement. The parallel planes should result in a vertex that
        //       is located within the cube.
        todo!()
    }
}
