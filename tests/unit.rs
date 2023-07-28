use assert_cmd::Command;

// testing with num < 0
#[test]
fn run_with_neg(){
    let mark = "-11";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: Invalid score\n");
}

// testing with num > 100
#[test]
fn run_with_invalid(){
    let mark = "111";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: Invalid score\n");
}

// testing with 0-49
#[test]
fn run_with_f(){
    let mark = "45";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: Failed with F\n");
}

// testing with 50-60
#[test]
fn run_with_d(){
    let mark = "55";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: D\n");
}

// testing with 61-70
#[test]
fn run_with_score_c(){
    let mark = "65";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: C\n");
}

// testing with 71-80
#[test]
fn run_with_b(){
    let mark = "75";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: B\n");
}

// testing with 81-94
#[test]
fn run_with_a(){
    let mark = "90";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: A\n");
}

// testing with 95-100
#[test]
fn run_with_aplus(){
    let mark = "96";
    Command::cargo_bin("scores").unwrap()
        .arg(mark).assert().success().stdout("Grade: Excellent with A+\n");
}