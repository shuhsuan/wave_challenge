use std::fs;
use std::fs::File;
use serde_json::Value;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {

    //reading a file
    let file = fs::File::open("simple_challenge_data.json")
    .expect("file should open read only");

    //parsing contents as a JSON
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be JSON");

    //extracting the list of 3D coordinates 
    let _p = json.get("p")
    .expect("file should have p key");

    //extracting the list of quadrilaterals
    let _q = json.get("q")
    .expect("file should have q key");

    //find the length of the array of q
    let size_q = _q.as_array().unwrap().len();

    let mut vec_pos: Vec<Value> = Vec::new();
    let mut vec_neg: Vec<Value> = Vec::new();

    //writing quadrilaterals which have a negative z-index directly to the file
    let file_neg=File::create("wet_geometry.json")?;
    let mut writer_neg = BufWriter::new(file_neg);
    writer_neg.flush()?;

    //writing quadrilaterals which have a positive z-index directly to the file
    let file_pos=File::create("dry_geometry.json")?;
    let mut writer_pos = BufWriter::new(file_pos);
    writer_pos.flush()?;

    let mut count_neg = 0;
    let mut count_pos = 0;


    for i in 0..size_q{  //for every quadrilateral of 4 points

        for x in 0..4{ //check each point for a negative z-index
            
            let coord_index = _q[i][x].as_u64().unwrap() as usize; //why does this not need a mut?

            if _p[coord_index][2].as_f64() < Some(0.0){ //if the z-index of the p coordinate is negative

                vec_neg.push(_q[i].clone()); 
                count_neg+=1;
            }
            else if _p[coord_index][2].as_f64() > Some(0.0){ //if the z-index of the p coordinate is positive

                vec_pos.push(_q[i].clone());
                count_pos+=1;
            }
        }
    }

    serde_json::to_writer(&mut writer_neg, &vec_neg)?;
    serde_json::to_writer(&mut writer_pos, &vec_pos)?;


    println!("{}", count_neg);
    println!("{}", count_pos);
    println!("{}", size_q);

    Ok(())
}


