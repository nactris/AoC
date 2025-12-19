use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("CODES.txt");
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
    let (passwd_old,passwd_new,_) = parse_steps(50,parse_line(s));
    print!("The Password is: {}\n",passwd_old);
    print!("The New Password is: {}\n",passwd_new);
    
    
}

fn parse_line(command: String) -> Vec<i32> {
   
let re = Regex::new(r"(L|R)([0-9]+)").unwrap();
let mut steps = vec![];

for (_, [dir, step]) in re.captures_iter(&command).map(|c| c.extract()) {
    steps.push(
    match dir {
        "L"=>  -step.parse::<i32>().unwrap(),
        &_ => step.parse::<i32>().unwrap(),
    });
    }
steps


}

fn parse_steps(start: u8, steps:Vec<i32>) -> (i32,i32, Vec<u8>){
let mut stops:Vec<u8> = vec![start];
let mut tick = start;
let mut passes = 0;
for s in steps{
    passes += (s/100).abs() as i32;
    let fix_s = s%100;
    if fix_s > (99- (tick as i32)) {
       // println!("right pass");
        passes+=1;
    }
    
    if -fix_s >= (tick as i32) && tick != 0{
     //   println!("left pass");
        passes+=1;
    }
   // print!("{}, s:{}, fs:{}, {}\n",tick, s,fix_s, passes);
    tick = ((fix_s+100 +tick as i32)%100) as u8;
    stops.push(tick);   
}
(stops.iter().filter(|&&n| n == 0).count() as i32,passes, stops)

}