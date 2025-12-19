use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("IDS.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_ranges(contents);
    Ok(())
}

fn parse_ranges(data:String)  { //-> Vec<(u32,u32)>

    let re = Regex::new(r"([0-9]+)-([0-9]+),?").unwrap();
    let mut invalid: Vec<u64> = vec![];
    for (_,[lo,hi]) in re.captures_iter(&data).map(|c| c.extract()){
        println!("{} - {}",lo,hi);

        for id in lo.parse::<u64>().unwrap()..=hi.parse::<u64>().unwrap()
        {
            let strid = id.to_string();
            for i in 2..=strid.len(){
                let (piece,_) = strid.split_at(strid.len()/i);
                
                if strid == piece.repeat(i){
                    //println!("{} matches!",&strid);
                    invalid.push(strid.parse::<u64>().unwrap());
                    break;
                }
            }

            //let (first_half, second_half) = strid.split_at(strid.len() / 2);
            //if first_half == second_half {
            //    invalid.push(strid.parse::<u64>().unwrap())
            //}
        }
    }
    let mut sum:u64 = 0;
    for inv in invalid {
        sum += inv;
    }
    print!("Answer: {}", sum);
}