// fn main() {
//     println!("Hello, world!");
// }

use clap::{App,Arg};
use clap::AppSettings::AllowNegativeNumbers;
use scores::grader;

fn main(){
    let _matches = App::new("scores")
        .arg(
            
            Arg::with_name("score")
            .value_name("SCORE")
            .index(1)
            .required(true)
        )
        .setting(AllowNegativeNumbers)
        .get_matches();

    let score = _matches.value_of("score").unwrap_or("0");
    let score = score.parse::<i32>().unwrap();
    println!("Grade: {}", grader(&score));

}