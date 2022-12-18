use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();

    let scan = read_to_string("data/day18_personal.txt")?;

    let mut drop: HashSet<Face> = HashSet::new();
    for cube in scan.lines() {
        let mut coordinates = cube.split(",");
        let x: isize = coordinates.next().unwrap().parse().unwrap();
        let y: isize = coordinates.next().unwrap().parse().unwrap();
        let z: isize = coordinates.next().unwrap().parse().unwrap();

        insert_face(x, y, z, FaceDirection::LEFT, &mut drop);
        insert_face(x, y, z, FaceDirection::RIGHT, &mut drop);
        insert_face(x, y, z, FaceDirection::DOWN, &mut drop);
        insert_face(x, y, z, FaceDirection::UP, &mut drop);
        insert_face(x, y, z, FaceDirection::FRONT, &mut drop);
        insert_face(x, y, z, FaceDirection::BACK, &mut drop);
    }
    println!(
        "part 1: {} ; time elapsed: {:?}",
        drop.len(),
        start.elapsed()
    );
    Ok(())
}

fn insert_face(x: isize, y: isize, z: isize, direction: FaceDirection, drop: &mut HashSet<Face>) {
    let face_to_check = match direction {
        FaceDirection::LEFT => Face {
            x: x - 1,
            y,
            z,
            direction: FaceDirection::RIGHT,
        },
        FaceDirection::RIGHT => Face {
            x: x + 1,
            y,
            z,
            direction: FaceDirection::LEFT,
        },
        FaceDirection::DOWN => Face {
            x,
            y: y + 1,
            z,
            direction: FaceDirection::UP,
        },
        FaceDirection::UP => Face {
            x,
            y: y - 1,
            z,
            direction: FaceDirection::DOWN,
        },
        FaceDirection::FRONT => Face {
            x,
            y,
            z: z + 1,
            direction: FaceDirection::BACK,
        },
        FaceDirection::BACK => Face {
            x,
            y,
            z: z - 1,
            direction: FaceDirection::FRONT,
        },
    };

    if drop.contains(&face_to_check) {
        drop.remove(&face_to_check);
    } else {
        drop.insert(Face { x, y, z, direction });
    }
}

fn insert_face_part2(
    x: isize,
    y: isize,
    z: isize,
    direction: FaceDirection,
    drop: &mut HashSet<Face>,
) {
    let matches: Vec<&Face> = drop
        .iter()
        .filter(|face| face.x > x && face.y == y && face.z == z)
        .collect();
    let mut size = matches.len();
    // delete only if value of axis of interest of face is smaller than the highest we deleted
    let mut current_highest = 0;
    drop.retain(|face| {
        if size == 1 {
            return true;
        }
        if face.x > x && face.y == y && face.z == z {
            size -= 1;
            return false;
        }
        return true;
    });
}

fn massacre_interior_faces(x: isize, y: isize, z: isize, drop: &mut HashSet<Face>) {

}

#[derive(Eq, Hash, PartialEq)]
struct Face {
    x: isize,
    y: isize,
    z: isize,
    direction: FaceDirection,
}

#[derive(Eq, Hash, PartialEq)]
enum FaceDirection {
    LEFT,
    RIGHT,
    DOWN,
    UP,
    FRONT,
    BACK,
}
