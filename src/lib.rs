pub fn grader(score : &i32) -> String{

    match score{
        0..=49 => String::from("Failed with F"),
        50..=60 => String::from("D"),
        61..=70 => String::from("C"),
        71..=80 => String::from("B"),
        81..=94 => String::from("A"),
        95..=100 => String::from("Excellent with A+"),
        _ => String::from("Invalid score")

    }
}