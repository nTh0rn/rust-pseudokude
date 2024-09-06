

fn main() {

    //let mut board = Vec::new();
    let mut b_stack = Vec::new();
    let mut aoe_stack = Vec::new();
    let mut p_stack = Vec::new();
    let mut p_limits_stack = Vec::new();
    let mut last_cell_stack = Vec::new();
    let mut board: Vec<Vec<i32>> =vec![
        vec![0,0,7,6,0,5,9,4,0],
        vec![0,0,0,0,0,0,0,0,6],
        vec![8,0,0,1,0,0,0,0,0],
        vec![0,0,0,0,0,0,2,0,0],
        vec![0,7,0,0,9,0,0,0,0],
        vec![0,0,9,0,0,4,5,3,0],
        vec![0,1,0,5,0,0,3,6,0],
        vec![0,0,0,0,0,6,0,0,7],
        vec![0,0,3,0,0,0,0,0,2]];
    
    let data_template: Vec<Vec<i32>> = vec![];

    let mut aoe = Vec::new();


    let mut p = Vec::new();
    let mut p_limits = Vec::new();
    let mut last_cell: [i32; 2];
    

    let mut i = 0;
    let mut j = 0;
    let mut k = 1;

    while i < 9 {
        aoe.push(data_template.clone());
        p.push(data_template.clone());
        p_limits.push(data_template.clone());
        while j < 9 {
            aoe[i].push(vec![]);
            p[i].push(vec![]);
            p_limits[i].push(vec![]);
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }
    
    i = 0;
    j = 0;
    while i < 9 {
        while j < 9 {
            update_aoe([i as i32,j as i32], &mut aoe, board.clone());
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }

    

    b_stack.push(board.clone());
    print_board(&b_stack[0]);

    for e in &aoe[0][0] {
        println!("{}", e);
    }

        
    update_all_p(&mut p, &mut p_limits, &mut aoe, board.clone());
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Yuppers");
    for e in &p[8][1] {
        println!("{}", e);
    }

    last_cell = [0,0];

    b_stack.push(board.clone());
    p_stack.push(p.clone());
    p_limits_stack.push(p_limits.clone());
    aoe_stack.push(aoe.clone());
    last_cell_stack.push(last_cell);
    

    for e in &p_stack[0][8][1] {
        println!("{}", e);
    }

    b_stack.push(board.clone());
    print_board(&b_stack[0]);

    
    let mut reset: bool = false;
    let mut ly: i32 = 0;
    let mut lx: i32 = 0;
    let mut ld: i32 = 0;

    let mut b_len: usize = 0;
    let mut p_len: usize = 0;
    let mut p_limits_len: usize = 0;
    let mut aoe_len: usize = 0;

    while true {
        board = b_stack[b_stack.len()-1].clone();
        p = p_stack[p_stack.len()-1].clone();
        p_limits = p_limits_stack[p_limits_stack.len()-1].clone();
        aoe = aoe_stack[aoe_stack.len()-1].clone();
        last_cell = last_cell_stack[last_cell_stack.len()-1].clone();

        i = 0;
        j = 0;
        while i < 9 {
            while j < 9 {
                if b_stack[b_stack.len()-1][i][j] == 0 {
                    if p_stack[p_stack.len()-1][i][j].len() > 0 {
                        board[i][j]=p_stack[p_stack.len()-1][i][j][0];
                        last_cell = [i as i32, j as i32];
                        update_all_p(&mut p, &mut p_limits, &mut aoe, board.clone());
                        
                        b_stack.push(board.clone());
                        p_stack.push(p.clone());
                        p_limits_stack.push(p_limits.clone());
                        aoe_stack.push(aoe.clone());
                        last_cell_stack.push(last_cell);
                        
                        reset = true;
                    } else {
                        ly = last_cell_stack[last_cell_stack.len()-1][0];
                        lx = last_cell_stack[last_cell_stack.len()-1][1];
                        ld = b_stack[b_stack.len()-1][ly as usize][lx as usize];

                        b_stack.remove(b_stack.len()-1);
                        p_stack.remove(p_stack.len()-1);
                        p_limits_stack.remove(p_limits_stack.len()-1);
                        aoe_stack.remove(aoe_stack.len()-1);
                        last_cell_stack.remove(last_cell_stack.len()-1);
                        
                        b_len = b_stack.len()-1;
                        p_limits_len = p_limits_stack.len()-1;
                        p_len = p_stack.len()-1;
                        aoe_len = aoe_stack.len()-1;

                        p_limits_stack[p_limits_len][ly as usize][lx as usize].push(ld);


                        b_stack[b_len][ly as usize][lx as usize] = 0;

                        update_all_p(&mut p_stack[p_len], &mut p_limits_stack[p_limits_len], &mut aoe_stack[aoe_len], b_stack[b_len].clone());
                        reset = true;
                    }
                    if reset == true {
                        break;
                    }
                }
                j = j + 1;
            }
            if reset == true {
                break;
            }
            j = 0;
            i = i + 1;
        }
        if reset == true {
            reset = false;
        } else {
            break;
        }
        
    }
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print_board(&b_stack[b_stack.len()-1]);


}

fn update_all_p(p: &mut Vec<Vec<Vec<i32>>>, p_limits: &mut Vec<Vec<Vec<i32>>>, aoe: &mut Vec<Vec<Vec<i32>>>, board: Vec<Vec<i32>>) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;

    while i < 9 {
        while j < 9 {
            update_aoe([i as i32, j as i32], aoe, board.clone());
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }

    i = 0;
    j = 0;
    k = 1;
    
    //Possibilities
    while i < 9 {
        while j < 9 {
            p[i][j].clear();
            if board[i][j]==0 {
                while k < 10 {
                    if !p_limits[i][j].contains(&(k as i32)) {
                        if !aoe[i][j].contains(&(k as i32)) {
                            p[i][j].push(k as i32);
                        }
                    }
                    k = k + 1
                }
            }
            k = 0;
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }
}

fn update_aoe(c: [i32; 2], aoe: &mut Vec<Vec<Vec<i32>>>, board: Vec<Vec<i32>>) -> Vec<i32> {
    
    aoe[c[0] as usize][c[1] as usize].clear();

    //Add house cells
    aoe[c[0] as usize][c[1] as usize] = (get_house_cells([c[0],c[1]], board.clone()));

    //Right after house
    let mut i = 3-(c[1]%3)+c[1];
    let mut ilim = 9;
    while i < ilim {
        aoe[c[0] as usize][c[1] as usize].push(board[c[0] as usize][i as usize]);
        i = i + 1;
    }

    //Left before house
    i = 0;
    ilim = c[1]+(3-(c[1]%3))-3;
    while i < ilim {
        aoe[c[0] as usize][c[1] as usize].push(board[c[0] as usize][i as usize]);
        i = i + 1;
    }

    //Below after house
    i = 3-(c[0]%3)+c[0];
    ilim = 9;
    while i < ilim {
        aoe[c[0] as usize][c[1] as usize].push(board[i as usize][c[1] as usize]);
        i = i + 1;
    }

    //Top before house
    i = 0;
    ilim = c[0]+(3-(c[0]%3))-3;
    while i < ilim {
        aoe[c[0] as usize][c[1] as usize].push(board[i as usize][c[1] as usize]);
        i = i + 1;
    }

    return vec![0];
}


fn print_board(board: &Vec<Vec<i32>>) {
    for x in board {
        for y in x {
            if *y == 0 {
                print!(". ");
            } else {
                print!("{y} ");
            }
        }
        println!("");
    }
}


//Returns all numbers within an x and y's house.
fn get_house_cells(yx: [i32; 2], board: Vec<Vec<i32>>) -> Vec<i32> {
    
    //Fancy math to turn an x and y into the
    //top-left coordinate of its house
    let hx: f32 = (yx[1] as f32)/3.0;
    let hy: f32 = (yx[0] as f32)/3.0;
    let cx = (((hx).floor() % 3.0)*3.0) as i32;
    let cy = (((hy).floor() % 3.0)*3.0) as i32;

    let mut output: Vec<i32> = vec![];

    let mut i = cy;
    let mut j = cx;

    //Iterate through the house and add digits to output
    while i < cy+3 {
        while j < cx+3 {
            output.push(board[i as usize][j as usize]);
            j = j + 1;
        }
        j = cx;
        i = i + 1;
    }

    return output;
}


/*
fn id_to_coord(mut id: i32) -> Vec<i32> {
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [9, 10, 11, 12, 13, 14, 15, 16, 17]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
}
    */