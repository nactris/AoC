use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("BANKS.txt") {
        let mut total_jolt = 0;
        for line in lines.map_while(Result::ok) {
            total_jolt += find_set_jolt(line,12);

        }
        print!("{}",total_jolt);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_max_jolt(bank:String) -> u32{
        let ported:Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut first = 0;
    for i in 0..bank.len()-1 {
        if ported[first] < ported[i] {
            first = i;
        }
    }
    let mut second = first+1;
    for i in first+1..bank.len() {
        if ported[second] < ported[i] {
            second = i;
        }
    }
    println!("{}{}",bank.chars().nth(first).unwrap(),bank.chars().nth(second).unwrap());
    ported[first]*10+ported[second]

}

fn find_set_jolt(bank:String, batnum:usize) -> u64 {

    let ported:Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut sel:Vec<usize> = vec![0; batnum];
    let mut left_bracket = 0;
    for bati in 0..batnum{
        sel[bati] = left_bracket;
        for i in left_bracket..=bank.len()-batnum+bati {
           // println!("   {}v{}:{}",ported[sel[bati]],ported[i],ported[sel[bati]] < ported[i]);
            if ported[sel[bati]] < ported[i] {
                sel[bati] = i;
            }
        }
     //   print!("{}..{}: {} at {}\n",left_bracket,bank.len()-batnum+bati,ported[sel[bati]], sel[bati]);
        left_bracket = sel[bati]+1;
    }
    let mut sum:u64 = 0;
    for i in sel{
        sum= sum*10 + ported[i] as u64;
    }
    print!("{}",sum);
    println!("");
    sum

}