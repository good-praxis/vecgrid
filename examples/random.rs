use rand::prelude::*;
use vecgrid::Vecgrid;

fn main() {
    let mut rng = rand::thread_rng();
    let board = Vecgrid::filled_by_row_major(|| rng.gen_range(0..10), 3, 2);
    println!("{:?}", board);

    let mut counter = 1;
    let f = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let board2 = Vecgrid::filled_by_column_major(f, 2, 3);
    println!("{:?}", board2);
}
