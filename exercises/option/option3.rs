// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        // Fun note here! I tried to write a basic benchmark on here, because if you know `ref`,
        // you can do this differently with Some(ref p) instead. It turns out this is ever SO
        // slightly slower! So match &y is the way to go.
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
