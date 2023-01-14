use crate::base_types::Position;


pub static DIRECTION_OFFSETS: [i8; 8] = [8, -8, -1, 1, 7, -7, 9, -9];
pub static NUM_SQUARES_TO_EDGE: [[u8; 8]; 64] = create_num_square_to_edge();


pub fn get_direction_index(start_pos : Position, end_pos : Position) -> usize {

    let row_dif = end_pos.get_row() as i8 - start_pos.get_row() as i8;
    let col_dif = end_pos.get_col() as i8 - start_pos.get_col() as i8;

    if row_dif == 0 {
        if col_dif > 0 { // East
            3
        } else { // West
            2
        }
    } else if col_dif == 0 {
        if row_dif > 0 {
            0
        } else {
            1
        }
    } else if row_dif == col_dif {
        if row_dif > 0 {
            6
        } else {
            7
        }
    } else if row_dif == -col_dif {
        if row_dif > 0 {
            4
        } else {
            5
        }
    } else {
        println!("No direction found for start_pos: {}, end_pos: {}", start_pos.to_string(), end_pos.to_string());
        0
    }
}


const fn min(a: u8, b: u8) -> u8
{
    if a < b {
        a
    } else {
        b
    }
}

const fn create_num_square_to_edge() -> [[u8; 8]; 64] {
    let mut num_squares_to_edge : [[u8; 8];64] = [[0; 8]; 64];
    let mut iter: usize = 0;
    loop {
        if iter == 64 {
            break;
        }
        let row = (iter / 8) as u8;
        let col = (iter % 8) as u8;

        let num_north = 7 - row;
        let num_south = row;
        let num_west = col;
        let num_east = 7 - col;

        num_squares_to_edge[iter][0] = num_north;
        num_squares_to_edge[iter][1] = num_south;
        num_squares_to_edge[iter][2] = num_west;
        num_squares_to_edge[iter][3] = num_east;
        num_squares_to_edge[iter][4] = min(num_north, num_west);
        num_squares_to_edge[iter][5] = min(num_south, num_east);
        num_squares_to_edge[iter][6] = min(num_north, num_east);
        num_squares_to_edge[iter][7] = min(num_south, num_west);

        iter += 1;
    }
    num_squares_to_edge
}