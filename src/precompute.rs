
pub static DIRECTION_OFFSETS: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
pub static NUM_SQUARES_TO_EDGE: [[u8; 8]; 64] = create_num_square_to_edge();


const fn create_num_square_to_edge() -> [[u8; 8]; 64] {
    let mut num_squares_to_edge : [[u8; 8];64] = [[0; 8]; 64];
    let mut iter = 0;
    loop {
        if iter == 64 {
            break;
        }
        let row = (iter / 8) as u8;
        let col = (iter % 8) as u8;
        num_squares_to_edge[iter][0] = row;
        num_squares_to_edge[iter][1] = 7 - row;
        num_squares_to_edge[iter][2] = col;
        num_squares_to_edge[iter][3] = 7 - col;
        num_squares_to_edge[iter][4] = if row < col { row } else { col };
        num_squares_to_edge[iter][5] = if row < col { 7 - col } else { 7 - row };
        num_squares_to_edge[iter][6] = if row < col { col } else { row };
        num_squares_to_edge[iter][7] = if row < col { 7 - row } else { 7 - col };
        iter += 1;
    }
    num_squares_to_edge
}