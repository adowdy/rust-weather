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

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

fn main() -> std::io::Result<()> {

    // LEARN -- .to_string is doing explicit allocation
    // we need this because later we will assign into this
    let mut smallest_day = "".to_string();
    let mut smallest_range = 1000.0;

    let f = try!(File::open("weather.dat"));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let mut unwrapline = line.unwrap();
        let mut linewords: Vec<_> = unwrapline.split_whitespace().collect(); 
        if linewords.len() == 0 { print!("skip line\n"); continue; }
        if linewords.contains(&"Dy") { print!("skip line\n"); continue; }

        let day = linewords[0].to_string();
        let s_high_temp = strip_characters(linewords[1], "*").to_string();
        let f_high_temp = s_high_temp.parse::<f64>().unwrap();
        let s_low_temp = strip_characters(linewords[2], "*").to_string();
        let f_low_temp = s_low_temp.parse::<f64>().unwrap();

        if (f_high_temp - f_low_temp) < smallest_range {
            smallest_day = day;
            smallest_range = f_high_temp - f_low_temp;
        }  
    }
    println!("day number w/ smallest spread is {}", smallest_day);           
    Ok(())
}

