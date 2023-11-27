mod libs;

use std::fs;
use libs::{sweep_line, SegmentTreeSegments, SegmentTree};

use std::path::Path;
use std::fs::{ File};
use std::io::{ BufReader, BufRead};


fn read(file_pat: String) {
    let mut tree :SegmentTree = SegmentTree{ tree: vec![], lazy: vec![] };
    let file = File::open(file_pat).expect("Unable to read the input file");
    let mut reader = BufReader::new(file);

    let mut conta_linea: i32= 0;
    let mut dim_array: usize = 0;
    
    for line in reader.lines() {
        conta_linea = conta_linea + 1;
        let value= line.expect("REASON");
        //println!("Linea letta: {}", value);
        let value_split: Vec<&str> = value.split(' ').collect();
        if conta_linea == 1 {
            dim_array = value_split[0].parse().unwrap();
            tree.new_segment_tree(dim_array);
        }
        else if conta_linea == 2 {
            let array: Vec<usize> = value_split.iter().map(|&s| s.parse().unwrap()).collect();
            tree.build(&array, 1, 0, dim_array-1);
        }
        else if conta_linea > 2 {
            let value_split_int: Vec<usize> = value_split.iter().map(|&s| s.parse().unwrap()).collect();
            if value_split_int[0] == 0 { // Update (i, j, k)
                let i: usize = value_split_int[1] ;
                let j: usize = value_split_int[2] ;
                let k: usize = value_split_int[3] ;
                tree.update(1, 0, dim_array-1, i-1, j-1, k);
                //println!("Query Update!");
            }
            else if value_split_int[0] == 1 { // Max(i, j)
                let i: usize = value_split_int[1];
                let j: usize = value_split_int[2];
                let risp = tree.query(1, 0, dim_array-1, i-1, j-1);
                response_array.push(risp);
                //println!("{}", risp)
                //println!("Response query Max: {}", tree.query(1, 0, dim_array-1, i-1, j-1))

            }
        }
    }
}

fn read2(file_pat: String) -> Vec<usize> {
    let mut tree :SegmentTreeSegments = SegmentTreeSegments{ tree: vec![], size: 0 };
    let file = File::open(file_pat).expect("Unable to read the input file");
    let reader = BufReader::new(file);

    let mut conta_linea: i32 = 0;
    let mut dim_array: usize = 0;
    let mut response_array: Vec<usize> = vec![];
    let mut intervals: Vec<(usize, usize)> = Vec::new();
    let mut queries: Vec<(usize, usize, usize)> = Vec::new();
    for line in reader.lines() {
        conta_linea = conta_linea + 1;
        let value= line.expect("REASON");
        //println!("Linea letta: {}", value);
        let value_split: Vec<&str> = value.split(' ').collect();

        if conta_linea == 1 {
            dim_array = value_split[0].parse().unwrap();
            tree.new_segment_tree_s(dim_array);
        }
        else {
            if value_split.len() == 2 {
                intervals.push((value_split[0].parse().unwrap(), value_split[1].parse().unwrap()));
            } else {
                if value_split.len() == 3 {
                    queries.push((value_split[0].parse().unwrap(), value_split[1].parse().unwrap(), value_split[2].parse().unwrap()));
                }
            }
        }

    }
    let array = sweep_line(&intervals);
    //println!("{:?}", array);
    tree.build(&array, 1, 0, dim_array-1);

    for (i, j, k) in queries{
       response_array.push(tree.is_there(1, 0, dim_array-1, i, j, k));
    }
    //print!("intervals: {:?}", &intervals);
    //println!("Queries{:?}", queries);
    return response_array;
}

fn main() {
    let directory_path = "src/Testset_handson2_2324_p1/";
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                //println!("Sono dentro la directory");
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with("input") {
                    let input_file_path: String = format!("{}{}",directory_path, file_name_str);
                    let exists = Path::new(input_file_path.as_str()).exists();

                    if exists {
                        let risp = read(input_file_path.to_string());
                        let mut output_correct: Vec<usize> = Vec::new();
                        let output_file_path = input_file_path.replace("input", "output");
                        let exits_output_file = Path::new(&output_file_path).exists();

                        if exits_output_file {
                            let file = File::open(&output_file_path).expect("Unable to read the output file");
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                output_correct.push(line.expect("REASON").parse().unwrap());
                            }
                        }

                        if risp == output_correct {
                            println!("Test passed {}", input_file_path);
                        } else {
                            println!("Test NOT passed {}", input_file_path);
                        }
                    }

                    //println!("File trovato: {}", file_name_str);
                }
            } else {
                println!("Error reading directory: {}", directory_path);
            }
        }

    }

    /// SECONDO ESERCIZIO

    println!("SECONDO ASSEGNAMENTO");
    let directory_path = "src/Testset_handson2_2324_p2/";
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                //println!("Sono dentro la directory");
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with("input") {
                    let input_file_path: String = format!("{}{}",directory_path, file_name_str);
                    let exists = Path::new(input_file_path.as_str()).exists();

                    if exists {
                        let risp = read2(input_file_path.to_string());
                        let mut output_correct: Vec<usize> = Vec::new();
                        let output_file_path = input_file_path.replace("input", "output");
                        let exits_output_file = Path::new(&output_file_path).exists();

                        if exits_output_file {
                            let file = File::open(&output_file_path).expect("Unable to read the output file");
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                output_correct.push(line.expect("REASON").parse().unwrap());
                            }
                        }
                       println!("{:?}", risp);
                        if risp == output_correct {
                            println!("Test passed {}", input_file_path);
                        } else {
                            println!("Test NOT passed {}", input_file_path);
                        }
                    }

                    //println!("File trovato: {}", file_name_str);
                }
            } else {
                println!("Error reading directory: {}", directory_path);
            }
        }
    }


}