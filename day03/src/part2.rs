use crate::part1::parse_input;

pub fn parse_horizontal(input: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut triangles: Vec<(i32, i32, i32)> = vec![];

    for chunk in input.chunks(3) {
        triangles.push((chunk[0].0, chunk[1].0, chunk[2].0));
        triangles.push((chunk[0].1, chunk[1].1, chunk[2].1));
        triangles.push((chunk[0].2, chunk[1].2, chunk[2].2));
    }

    triangles
}

pub fn solve_part_2(input: &str) -> i32 {
    let mut valids = 0;
    let parsed = parse_input(input);
    let triangles = parse_horizontal(&parsed);

    // I still dont know how triangles work.
    for (side1, side2, side3) in triangles {
        if (side1 + side2) > side3 {
            if (side2 + side3) > side1 {
                if (side3 + side1) > side2 {
                    valids += 1;
                }
            }
        }
    }

    return valids;
}
