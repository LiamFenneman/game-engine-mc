const R: i32 = 3;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let r = args.get(1).map(|s| s.parse::<i32>().unwrap_or(R)).unwrap_or(R);
    let pts = ge_util::points_in_circle(r);
    for y in -r..=r {
        for x in -r..=r {
            match pts.iter().find(|&&p| p == (x, y)) {
                Some(_) => print!("."),
                None => print!(" "),
            }
        }
        println!();
    }
}
