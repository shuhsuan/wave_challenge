use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs;


///This struct replicates the format of the given data
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    q: Vec<Quadrilateral>,
    p: Vec<Point>,
}

///This struct type defines 3D coordinates 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point([f64; 3]);

///This struct type defines Quadrilaterals
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Quadrilateral([usize; 4]);


impl Point {

    ///This function returns the z-index
    /// 
    /// Example:
    /// 
    /// Set x to be a coordinate of type Point
    ///```
    /// let x: Point = [1, 2, -3];
    /// ```
    /// Find the z-index with the function
    /// ```
    /// let z_value = x.z(); 
    /// ```
    /// 
    fn z(&self) -> f64 {
        self.0[2]
    }

    ///This function checks if the z-index is below 0.0 and returns a boolean
    /// 
    /// Example:
    /// 
    /// Set x to be a coordinate of type Point
    /// ```
    /// let x: Point = [1, 2, -3];
    /// ```
    /// Use the function to check if the z-index is below 0.0
    /// ```
    /// let value = x.is_wet();
    /// ```
    /// The variable, value, will return true
    fn is_wet(&self) -> bool {
        self.z() < 0.0
    }
}

///This function changes the value in the quadrilateral at the index given and returns the new quadrilateral
/// 
/// Example:
/// 
/// Given a quadrilateral 
/// ```
/// let mut q: Quadrilateral = [1,2,3,4];
/// ```
/// Change the 3rd value to 15
/// ```
/// q = change_q(q, 2, 15)
/// ```
/// The value of q is now
/// ```
/// [1,2,15,4]
/// ```
fn change_q(mut q: Quadrilateral, at_index: usize, new_value: usize) -> Quadrilateral {
    q.0[at_index] = new_value;
    q //return q
}


fn main() {
    let data_string =
        fs::read_to_string("./simple_challenge_data.json").expect("Unable to read file");
    let data: Data = serde_json::from_str(&data_string).unwrap();

    let q_data = data.q;
    let p_data = data.p;

    let mut count_neg: usize = 0;
    let mut count_pos: usize = 0;

    let mut map_neg: HashMap<usize, usize> = HashMap::new();
    let mut map_pos: HashMap<usize, usize> = HashMap::new();

    let mut p_neg: Vec<Point> = Vec::new();
    let mut p_pos: Vec<Point> = Vec::new();

    let mut q_negative: Vec<Quadrilateral> = Vec::new();
    let mut q_positive: Vec<Quadrilateral> = Vec::new();

    'loop_quads: for i in q_data {
        //accesses q_data[i]

        let index = i.0;

        for a in index {
            // a is an index intended to map to a point in p_data

            if p_data[a].is_wet() {
                //for negative z

                match map_neg.entry(a) {
                    Occupied(_) => {
                        let position: usize = index.iter().position(|&r| r == a).unwrap(); // index of a in the quadrilteral
                        let new_q_neg = change_q(i.clone(), position, map_neg[&a]); //change the index that maps to p
                        q_negative.push(new_q_neg); //Push to a new array
                    }

                    Vacant(_) => {
                        p_neg.push(p_data[a].clone()); //count and the index that p_data[a] is pushed into are the same
                        let position: usize = index.iter().position(|&r| r == a).unwrap(); //the index value of the point
                        let new_q_neg = change_q(i.clone(), position, count_neg); //change the index that maps to a new p
                        q_negative.push(new_q_neg); //push to a new array
                        count_neg += 1; //Increase count for next negative
                    }
                }

                continue 'loop_quads; //I define the problem such that if just one p coordinate is wet then the quadrilateral is defined as wet and will be placed into the wet_data, thus it should skip to the next quadrilateral after it finds one wet coordinate
            }
            //for positive z
            else {
                let position: usize = index.iter().position(|&r| r == a).unwrap();

                if position == 3 {
                    //only place it in the dry_data if all points in the quadrilateral are found to be dry

                    match map_pos.entry(a) {
                        Occupied(_) => {
                            let position: usize = index.iter().position(|&r| r == a).unwrap();
                            let new_q_pos: Quadrilateral =
                                change_q(i.clone(), position, map_pos[&a]);
                            q_positive.push(new_q_pos);
                        }
                        Vacant(_) => {
                            p_pos.push(p_data[a].clone());
                            let position: usize = index.iter().position(|&r| r == a).unwrap();
                            let new_q_pos: Quadrilateral = change_q(i.clone(), position, count_pos);
                            q_positive.push(new_q_pos);
                            count_pos += 1;
                        }
                    }
                }
            }
        }
    }

    let wet_data = Data {
        q: q_negative,
        p: p_neg,
    };

    let dry_data = Data {
        q: q_positive,
        p: p_pos,
    };

    let wet_data_JSON = serde_json::to_string(&wet_data); //serializes the wet_data struct to a JSON string
    let dry_data_JSON = serde_json::to_string(&dry_data); //serializes the dry_data struct to a JSON string

    let mut f_wet = fs::File::create("wet_data.json").expect("Failed to create JSON file"); //creates a file called wet_data.json
    let mut f_dry = fs::File::create("dry_data.json").expect("Failed to create JSON file"); //creates a file called dry_data.json

    serde_json::to_writer(&mut f_wet, &wet_data); //serializes the wet_data struct to the wet_data.json file
    serde_json::to_writer(&mut f_dry, &dry_data); //serializes the dry_data struct to the dry_data.json file

    //The p points in wet_data and dry_data don't add up to p_data.len(), some values are repeated?
}
