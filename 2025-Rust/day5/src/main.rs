use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;
use regex::Regex;
use core::cmp::max;

fn main() {
    let data: String = open_to_str("INGREDIENTS.txt");
    let readdata = parse(data);
    println!("total fresh: {}", check_fresh(readdata.clone())) ;
    println!("-------------------------");
    println!("total possible fresh: {}", check_fresh_total(readdata)) ;
 
}


fn check_fresh(data:Option<(Vec<(u64,u64)>,Vec<u64>)>) -> u64{
    let mut total = 0;
    if let Some((ranges,ids)) = data {
        for id in &ids {
            let mut success = false;
            for (lb,rb) in &ranges {
                if id >= lb && id <= rb {
                    success = true;
                   // println!("id {} within range {}-{}", id,lb,rb);
                }
            } 
            if success {
                total +=1;
            }
            
        }
        
        println!("total number checked: {}", &ids.len());
        println!("total number ranges: {}", &ranges.len());
    }
    total
}

fn check_fresh_total(data:Option<(Vec<(u64,u64)>,Vec<u64>)>) -> u64{
    let mut total = 0;
    let mut refined_ranges = vec![];
    if let Some((mut ranges,_)) = data{

        ranges.sort_by(|(a,_),(b,_)|a.cmp(&b) );
         for(i,(lm,rm)) in ranges.iter().enumerate(){
                println!( "{}: {}-{}, {}",i, lm,rm,rm-lm+1);

            }
        println!("-------------------------");
        

        let (mut lm, mut rm) = ranges[0];
        for range in &ranges[1..]{
          
               // println!( "refining {}-{}",lm,rm);
                let (nrl,nrr) = range;
                if rm >= *nrl {
                    println!( "overlap at {}-{} with {}-{}",lm,rm,nrl,nrr);
                    rm = max(*nrr,rm);
                }
                else{
                    refined_ranges.push((lm,rm));
                    (lm,rm) = *range;
                }

            }
            refined_ranges.push((lm,rm));


            for(i,(lm,rm)) in refined_ranges.iter().enumerate(){
                println!( "{}: {}-{}, {}",i, lm,rm,rm-lm+1);

                total += rm-lm+1;
            }
        } 
    total
}






fn parse (data:String) -> Option<(Vec<(u64,u64)>, Vec<u64>)>{

if let Some((ranges, ids)) = data.split_once("\n\r\n") {

    let re = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
    let mut results = vec![];
    for (_, [ lid,rid]) in re.captures_iter(&data).map(|c| c.extract()) {
        results.push( (lid.parse().unwrap(),rid.parse().unwrap()));
    }

   return Some((results,ids.lines().map(|line| line.parse().unwrap_or(0)).collect()))

}

  None  

    
}

fn open_to_str(name:&str) -> String{
    let path = Path::new(name);
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
    s
}