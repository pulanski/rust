use nalgebra::dmatrix;

// making structs
#[allow(dead_code)]
struct Data {
    num1: i32,
    num2: i32,
    str: String,
    optional_num: Option<i32>,
}

// making tuples

struct TwoNums(i32, i32);

// progress bar
fn main() {
    structs_playground();
    progress_bar();
}

fn do_hard_work() {
    for i in 0..10000 {
        let _d = dmatrix![1, 2, 3;
                 4, 5, 6;
                 7, 8, 9];
    }
}

fn progress_bar() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn structs_playground() {
    let _a = Data {
        num1: 2,
        num2: 4,
        str: String::from("hello"),
        optional_num: Some(4),
    };
    let d = TwoNums(3, 5);
    println!("{:10}{:10}", d.0, d.1);

    let mut v = vec![3, 4, 5];
    let p = v.pop();

    dbg!(Some(p));

    eprintln!("error");
}
