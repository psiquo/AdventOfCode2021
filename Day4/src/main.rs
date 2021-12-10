use std::fs;

fn main() {
    part_1("./resources/input");
    part_2("./resources/input_2");

}

fn part_1(filename : &str) {
    let contents  = fs::read_to_string(filename).expect("Cannot read file");
    let contents : Vec<&str> =  contents.split("\n").collect::<Vec<&str>>();

    let extractions : Vec<&str> = contents[0].trim().split(",").collect();


    let mut tables = extract_tables(&contents[1..].to_vec());

    for extracted in extractions {
        update_extraction(&mut tables, extracted);
        match check_win(&tables) {
            None => (),
            Some((t,_)) => {           
                println!("{}",
                    t.iter()
                        .flatten()
                        .filter(|x| !x.ends_with("x"))
                        .map(|x| x.parse::<u32>().unwrap())
                        .fold(0,|acc,x| acc + x) * extracted.parse::<u32>().unwrap()  
                );
                break;
            }
        }
    }        
}

fn part_2(filename : &str) {
    let contents  = fs::read_to_string(filename).expect("Cannot read file");
    let contents : Vec<&str> =  contents.split("\n").collect::<Vec<&str>>();

    let extractions : Vec<&str> = contents[0].trim().split(",").collect();


    let mut tables = extract_tables(&contents[1..].to_vec());

    let mut last : (Vec<Vec<String>>,&str) = (Vec::new(),extractions[0]);

    for extracted in extractions {
        update_extraction(&mut tables, extracted);
        match check_win(&tables) {
            None => (),
            Some((t,i)) => {
                last = (t.clone(),extracted);
                if tables.iter().all(|x| is_winner(x)){
                    break;
                } else {
                    tables.remove(i);
                }
            }
        }
    } 
      
    println!("{}",
        (last.0).iter()
            .flatten()
            .filter(|x| !x.ends_with("x"))
            .map(|x| x.parse::<u32>().unwrap())
            .fold(0,|acc,x| acc + x) * (last.1).parse::<u32>().unwrap()   
    );
    
}


fn check_win(tables : &Vec<Vec<Vec<String>>>) -> Option<(&Vec<Vec<String>>,usize)> {
    for (i,table) in tables.iter().enumerate() {
        if is_winner(table){
            return Some((table,i));
        }
    }

    return None;
} 

fn is_winner(table : &Vec<Vec<String>>) -> bool {
    for (i,row) in table.iter().enumerate() {
        if row.iter().all(|s| s.ends_with("x")) {
            return true;
        }
    }

    for index in 0..table[0].len() {
        if table.iter().map(|x|  &x[index]).all( |s|  s.ends_with("x")) {
            return true;
        }
    }

    return false;
}

fn update_extraction(tables :  &mut Vec<Vec<Vec<String>>>, extracted : &str) {
    for table in tables {
        for row in table {
            for num in row {
                if *num == String::from(extracted){
                    num.push('x');
                }
            }
        }
    }
}

fn extract_tables<'a>(table : &Vec<&'a str>) -> Vec<Vec<Vec<String>>> {
    let mut tables : Vec<Vec<Vec<String>>> = Vec::new();
    for i in 0 .. table.len(){
        if table[i] == "" && i+1 < table.len() {
            tables.push(extract_table(table,i+1));
        }
    }
    return tables;
}

fn extract_table<'a>(input : &Vec<&'a str>, index : usize) -> Vec<Vec<String>> {
    let mut table : Vec<Vec<String>> = Vec::new();
    let mut index = index;
    loop {
        if input[index] == ""{
            break;
        }

        let line : Vec<String> = input[index].trim().split(" ").filter(|x| *x != "").map(String::from).collect();
        table.push(line);
        index+=1;
    }

    return table;
}

