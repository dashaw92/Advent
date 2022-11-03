use std::error::Error;
use std::fs::read_to_string;

type Triangle = [i32; 3];

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Number of possible triangles for p1: {}", solve_p1(&input));
    println!("Number of possible triangles for p2: {}", solve_p2(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    solve(input)
}

//For p2, since the input is interpreted in a columnar orientation instead of p1's row based,
//we'll first re-arrange the given input to match this:
//given the input:
//101 301 501
//102 302 502
//103 303 503
//...would become:
//101 102 103
//301 302 303
//501 502 503
fn solve_p2(input: impl AsRef<str>) -> usize {
    let mut buf = String::new();

    //Since we're taking 1 triangle from every 3 lines (one column),
    //the total number of triangles from each column is the (number of lines / 3)
    let num = input.as_ref().lines().count() / 3;

    //colN = triangles only from column N
    let mut col1 = Vec::with_capacity(num);
    let mut col2 = Vec::with_capacity(num);
    let mut col3 = Vec::with_capacity(num);

    //build each column's list
    for line in input.as_ref().lines() {
        let cols: Vec<_> = line.split_whitespace().collect();
        //every line holds values from 3 different triangles,
        //store in the correct column
        col1.push(cols[0]);
        col2.push(cols[1]);
        col3.push(cols[2]);
    }

    for _ in 0..num {
        for col in [&mut col1, &mut col2, &mut col3] {
            //transform our built up columns into the expected format
            //every line is again considered as one triangle, as opposed
            //to the format from p2, where triangles are built from every
            //3 values in the same column.
            buf.push_str(&format!(
                "{} {} {}\n",
                col.pop().unwrap(),
                col.pop().unwrap(),
                col.pop().unwrap()
            ));
        }
    }

    //With the transformed input in hand, we're off the solve just like p1.
    solve(&buf)
}

fn solve(input: impl AsRef<str>) -> usize {
    //For every line of the input, one triangle is defined.
    //Split on the whitespace of each line, and collect to [&str; 3]
    //Parse [&str; 3] -> Triangle ([i32; 3])
    //Filter out impossible triangles and count the remaining triangles
    input
        .as_ref()
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut sp| [sp.next().unwrap(), sp.next().unwrap(), sp.next().unwrap()])
        .map(parse_tri)
        .fold(0, |acc, tri| if is_possible(tri) { acc + 1 } else { acc })
}

fn parse_tri(s: [&str; 3]) -> Triangle {
    //The triangle is NOT sorted at this point.
    //It could be done to save a step in is_possible, but
    //sorting is not required to simply construct the triangle.
    [
        s[0].parse().unwrap(),
        s[1].parse().unwrap(),
        s[2].parse().unwrap(),
    ]
}

/// Determines if a triangle is valid by comparing the sum of two sides against the third side.
/// A triangle is valid iff the sum of any two sides is larger than the remaining side.
/// A shortcut exists where the smallest sides are summed and then compared to the largest size.
/// If the smallest sides are larger than the remaining biggest side, then it must hold that
/// the sum of any other combination of the sides is also larger than the remaining side.
fn is_possible(mut tri: Triangle) -> bool {
    tri.sort(); //sort the sides ascending so the smallest sides are at the start
    tri[0] + tri[1] > tri[2]
}
