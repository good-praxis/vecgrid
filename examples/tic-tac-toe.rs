use vecgrid::Vecgrid;

fn format_board(board: &Vecgrid<String>) -> String {
    board
        .rows_iter()
        .map(|row_iter| row_iter.cloned().collect::<Vec<_>>().join("|"))
        .collect::<Vec<_>>()
        .join("\n-----\n")
}

fn main() {
    let mut board = Vecgrid::filled_with(" ".to_string(), 3, 3);
    println!("{}\n", format_board(&board));
    board[(0, 2)] = "X".to_string();
    println!("{}\n", format_board(&board));
}
