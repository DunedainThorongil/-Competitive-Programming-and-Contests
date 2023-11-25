mod libs;
use libs::SegmentTree;

use std::path::Path;
use std::fs::{ File};
use std::io::{ BufReader, BufRead};

fn read(file_pat: String) {
    let mut tree :SegmentTree = SegmentTree{ tree: vec![], lazy: vec![] };
    let file = File::open(file_pat).expect("Impossibile leggere il file");
    let mut reader = BufReader::new(file);

    let mut conta_linea = 0;
    let mut dim_array: i32 = 0;
    let mut n_query: i32 = 0;


    for line in reader.lines() {
        conta_linea = conta_linea + 1;
        let value= line.expect("REASON");
        println!("Linea letta: {}", value);
        let value_split: Vec<&str> = value.split(' ').collect();
        if conta_linea == 1 {
            dim_array = value_split[0].parse().unwrap();
            n_query = value_split[1].parse().unwrap();
        }
        else if conta_linea == 2 {
            let array: Vec<i32> = value_split.iter().map(|&s| s.parse().unwrap()).collect();
            tree.build_segment_tree(array,dim_array, 1, 0, dim_array-1);
        }
        else if conta_linea > 2 {
            let value_split_int: Vec<i32> = value_split.iter().map(|&s| s.parse().unwrap()).collect();
            if value_split_int[0] == 0 { // Update (i, j, k)
                let i: i32 = value_split_int[1] ;
                let j: i32 = value_split_int[2] ;
                let k: i32 = value_split_int[3] ;
            }
            else if value_split_int[0] == 1 { // Max(i, j)
                let i: i32 = value_split_int[1];
                let j: i32 = value_split_int[2];
                println!("Risulato query Max: {}", tree.query(1, 0, dim_array, i, j))

            }
        }
    }
}

fn main() {
    let file_path = "02-hands-on-Segment-Trees/src/Testset_handson2_2324_p1/input0.txt";
    let exists = Path::new(file_path).exists();
    if exists {
        read(file_path.to_string());
    }

}