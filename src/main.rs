use std::fs::OpenOptions;
use std::path::PathBuf;

// use ncollide3d::math::{Isometry, Point, Vector};
// use ncollide3d::nalgebra::Point3;
// use ncollide3d::query::{Ray, RayCast};
// use ncollide3d::shape::TriMesh;

use parry3d::math::{Point, Vector};
use parry3d::query::{Ray, RayCast};
use parry3d::shape::TriMesh;

use stl_io::read_stl;

type Precision = f32;

struct NamedMesh {
    pub name: &'static str,
    pub mesh: TriMesh,
}

impl NamedMesh {
    fn new(name: &'static str) -> Self {
        let mut stl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .expect("couldn't resolve");
        stl_path.push("data");
        stl_path.push(name);

        println!("Parsing {:?}", stl_path);
        let mut f = OpenOptions::new()
            .read(true)
            .open(stl_path)
            .expect("Couldn't open file");
        let io_obj = read_stl(&mut f).expect("Couldn't parse STL");
        io_obj.validate().expect("Mesh is invalid");

        let mesh = TriMesh::new(
            io_obj
                .vertices
                .iter()
                .map(|v| Point::new(v[0], v[1], v[2]))
                .collect(),
            io_obj
                .faces
                .iter()
                .map(|t| {
                    [
                        t.vertices[0] as u32,
                        t.vertices[1] as u32,
                        t.vertices[2] as u32,
                    ]
                })
                .collect(),
        );
        Self { name, mesh }
    }
}

fn print_result(named_mesh: &NamedMesh, point: &[Precision; 3], vector: &[Precision; 3]) {
    let p = Point::new(point[0], point[1], point[2]);
    let v = Vector::new(vector[0], vector[1], vector[2]);

    let prefix = format!(
        "Ray from {:?} in direction {:?} vs {:?}: ",
        point, vector, named_mesh.name
    );

    match named_mesh.mesh.cast_local_ray_and_get_normal(
        &Ray::new(p, v),
        Precision::INFINITY,
        false, // unused
    ) {
        Some(intersection) => {
            if named_mesh.mesh.is_backface(intersection.feature) {
                println!("{}INTERSECTION WITH BACKFACE", prefix)
            } else {
                println!("{}INTERSECTION WITH SOMETHING ELSE", prefix)
            }
        }
        None => println!("{}NO INTERSECTION", prefix),
    }
}

fn main() {
    // let sez_right = NamedMesh::new("SEZ_right.stl");
    // println!("Motivating example");
    // print_result(&sez_right, &[28355.6, 51807.3, 47050.0], &[1.0, 0.0, 0.0]);

    let unit_cube = NamedMesh::new("unit_cube.stl");

    println!("\nAll of these rays originate outside of the unit cube, so none should intersect with a backface.\n");

    println!("\nTransverse ray-face intersection");
    print_result(&unit_cube, &[-1.0, 0.0, 0.0], &[1.0, 0.0, 0.0]);
    println!("\nRay-edge touch");
    print_result(&unit_cube, &[-0.5, 0.5, 0.5], &[1.0, 1.0, 0.0]);
    println!("\nRay-corner touch");
    print_result(&unit_cube, &[-0.5, 0.5, 0.5], &[1.0, 1.0, 1.0]);
    println!("\nRay-edge intersection");
    print_result(&unit_cube, &[-1.0, -1.0, 0.5], &[1.0, 1.0, 0.0]);
    println!("\nRay-corner intersection");
    print_result(&unit_cube, &[-1.0, -1.0, -1.0], &[1.0, 1.0, 1.0]);
}
