use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

fn main() -> std::io::Result<()>{
    let path = Path::new("MAP.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }
    let mut total_rolls = 0;
    let mut next_stage = s.clone();
    let mut layer_rolls = 0;
    loop {
        
    (layer_rolls, next_stage) = remove_layer_of_valid_rolls(&next_stage);
    total_rolls += layer_rolls;

    println!("number of valid rolls: {}",layer_rolls);
   // println!("{} \n",next_stage);
    if layer_rolls == 0 {
        break;
    }

    }
    println!("total valid rolls: {}",total_rolls);

    Ok(())
}

fn count_valid_rolls(data:&String) -> i32{
    let roll_table:Vec<Vec<bool>> = data.split('\n').map(|c| { c.trim_end().chars().map(|c| c == '@').collect()}).collect();
    let neighbors: Vec<(i32, i32)> = (-1..=1)
    .flat_map(|x| (-1..=1).map(move |y| (x, y)))
    .filter(|&(x, y)| x != 0 || y != 0)
    .collect();

    let mut valid_rolls = 0;
    for y in 0..roll_table.len() {
        for x in 0..roll_table[y].len() {
            if roll_table[y][x] {
                let mut sum = 0;
                for (xi,yi) in &neighbors {
                   if get_at(&roll_table,xi +x as i32,yi+ y as i32) {
                    sum += 1;
                   };
                }
                if sum <4 {
                    valid_rolls+=1;
                }
            }    
        }          
    }
    valid_rolls
}

fn remove_layer_of_valid_rolls(data:&String) -> (i32, String){
    let roll_table:Vec<Vec<bool>> = data.split('\n').map(|c| { c.trim_end().chars().map(|c| c == '@').collect()}).collect();
    let mut fixed_up_data:Vec<Vec<char>> = data.split('\n').map(|c| { c.trim_end().chars().collect()}).collect();

    let neighbors: Vec<(i32, i32)> = (-1..=1)
    .flat_map(|x| (-1..=1).map(move |y| (x, y)))
    .filter(|&(x, y)| x != 0 || y != 0)
    .collect();

    let mut valid_rolls = 0;
    for y in 0..roll_table.len() {
        for x in 0..roll_table[y].len() {
            if roll_table[y][x] {
                let mut sum = 0;
                for (xi,yi) in &neighbors {
                   if get_at(&roll_table,xi +x as i32,yi+ y as i32) {
                    sum += 1;
                   };
                }
                

                if sum <4 {
                    valid_rolls+=1;
                    fixed_up_data[y][x] = '.'
                }
            }    
        }          
    }

    let mut out: String = fixed_up_data.iter().map(|line| { 
        let mut s: String = line.iter().collect();
        s.push('\n');
        s
        }).collect();
        out = out.trim_end().to_string();
     
   (valid_rolls,out)
   
}


fn get_at(data:&Vec<Vec<bool>>,x:i32,y:i32) -> bool {
    if x < 0 || y < 0 {
        return false
    }
    data.get(y as usize)       
        .and_then(|row| row.get(x as usize))  
        .copied()                    
        .unwrap_or(false)
}
      

