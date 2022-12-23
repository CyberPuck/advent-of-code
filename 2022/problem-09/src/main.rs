mod tail_simulator {

    /// NOTE: Assuming origin (starting point) is (0,0)
    struct Simulator {
        head: Point,
        tail: Point,
    }

    /// NOTE: Assuming there are no negative coordinates
    struct Point {
        x: u32,
        y: u32,
    }
}

fn main() {
    println!("Hello, world!");
}
