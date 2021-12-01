use std::fs;

fn main() {
    part_1("./resources/input_1.txt");
    part_2("./resources/input_2.txt");
}

fn part_1(filename : &str){
    let contents = fs::read_to_string(filename)
    .expect("Error reading file");

    let mut old : i32 = -1;
    let mut count : u32 = 0;
    let mut cur : i32;

    for num in contents.split("\n") {
    cur = match num.parse() {
    Ok(num) => num,
    Err(_) => break
    };

    count += if old != -1 && cur > old { 1 } else { 0 };
    old = cur;
    }

    println!("{}",count);
}

fn part_2(filename : &str){
    let contents = fs::read_to_string(filename)
                    .expect("Error reading file");
    
    let mut windows : [u32;3] = [0;3];
    let mut num : u32;
    let mut count : u32 = 0;

    for (i,val) in contents.split("\n").enumerate() {

        num = match val.parse() {
            Ok(num) => num,
            Err(msg) => break,
        };

        if i < 3 {
            match i {
                0 => windows[0] += num,
                1 => for i in 0..2 {windows[i] += num},
                2 => for i in 0..3 {windows[i] += num}
                _ => panic!()
            }

            continue;
        }

        for c in 0..3 {
            windows[c] += if c != i%3 {num} else {0};
        }

        print!("{}: ",i);
        for c in 0..3 {
            print!("{} ",windows[c]);
        }
        println!("");

        match i % 3 {
            2 => compare(0, 2, &mut windows, &mut count),
            0 => compare(1, 0, &mut windows, &mut count),
            1 => compare(2, 1, &mut windows, &mut count),
            _ => panic!(),
        }

        windows[i%3] += num;

    }

    println!("{}",count);
}

fn compare(v1 : usize, v2 : usize, arr : &mut [u32], count : &mut u32){
    *count += if arr[v1] > arr[v2] {1} else {0};
    arr[v2] = 0;
}