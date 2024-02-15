use std::env;

fn main() {
    let mut args = Vec::new();
    for argu in env::args() {
        // println!("{argu}");
        args.push(argu);
    } 

    let init = args[1].parse::<i32>().unwrap();
    let fin = args[2].parse::<i32>().unwrap();
    let mut arr: Vec<i32> = Vec::new();

    range_to_array(init, fin, &mut arr);

    println!("{:?}", arr);
}

fn range_to_array(i: i32, f: i32, arr: &mut Vec<i32>) {
    if i == f+1 {
        return;
    }
    arr.push(i);
    range_to_array(i+1, f, arr);

}