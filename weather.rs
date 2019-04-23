/* 
In weather.dat you’ll find daily weather data for Morristown, NJ for June 2002. 

Download this text file, then write a program to output the day number 
(column one) with the smallest temperature spread 
(the maximum temperature is the second column, the minimum the third column).
Spend 30 - 45 minutes. If we have time, there are more slides!
http://codekata.com/data/04/weather.dat
*/

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
// use std::path::Path;

struct Day {
    dayNum: i32,
    minTemp: i32,
    maxTemp: i32,
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}


fn main() -> std::io::Result<()> {
    // let mut file = File::open("weather.dat")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    //let mut stringVec = Vec::new();

    let f = try!(File::open("weather.dat"));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let mut linewords_iter = l.split_whitespace(); 
        if linewords_iter.next() == Some("Dy")  {
            println!("line found that we don't want, skip");
            continue; 
        }
        if linewor
        // want first 3 of each line
        //stringVec.push(l);
        //println!("{}", l); 
        
    }           

    

    // take max - min = spread
    // day with smallest spread... print dy number
    Ok(())
}

