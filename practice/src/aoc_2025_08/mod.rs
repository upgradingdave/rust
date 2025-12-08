use ordered_float::NotNan;
use std::io::{BufRead};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point3D {
    x: u64,
    y: u64,
    z: u64,
}

impl Point3D {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Point3D { x, y, z }
    }
    
    fn distance(&self, other: &Point3D) -> f64 {
        let result = ((self.x.max(other.x) - other.x.min(self.x)).pow(2) + (self.y.max(other.y) - other.y.min(self.y)).pow(2) + (self.z.max(other.z) - other.z.min(self.z)).pow(2)) as f64;
        result.sqrt()
    }
}

pub fn solution(file_path: &str, total_distances: usize) -> (u64, u64) {

    let mut points = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let point = Point3D::new(parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap());
        points.push(point);
    }

    let mut distances: BTreeMap<NotNan<f64>, Vec<(&Point3D, &Point3D)>> = BTreeMap::new();
 
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = points[i].distance(&points[j]);
            distances.entry(NotNan::new(distance).unwrap()).or_insert(Vec::new()).push((&points[i], &points[j]));
         }
    }
    
    let mut circuits:Vec<HashSet<&Point3D>> = Vec::new();
    let mut circuits_by_point: HashMap<&Point3D, u64> = HashMap::new();

    //let mut count = 0;
    for count in 0..distances.len(){
        
        let pairs = distances.values().nth(count).unwrap();
        
        for (point1, point2) in pairs {
            // is one of these points part of a circuit?
            let circuit_id:u64 = if circuits_by_point.contains_key(point1) {
                circuits_by_point.get(point1).unwrap().clone()
            } else if circuits_by_point.contains_key(point2) {
                circuits_by_point.get(point2).unwrap().clone()
            } else {
                circuits.push(HashSet::new());
                (circuits.len()-1) as u64
            };
            
            let circuit = circuits.get_mut(circuit_id as usize).unwrap();
            println!("----{}----", count);
            println!("Circuit ID: {}", circuit_id);
            println!("Point1: {:?}", point1);
            println!("Point2: {:?}", point2);
            circuit.insert(point1);
            circuit.insert(point2);
            circuits_by_point.insert(point1, circuit_id);
            circuits_by_point.insert(point2, circuit_id);
            println!("Circuit ID: {} now contains {} points", circuit_id, circuit.len());
        }
        //count += pairs.len();
    }
        
    // order the circuits by size
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut total = 1;
    for (i, circuit) in circuits.iter().enumerate() {
        if i<3 {
            println!("Circuit ID: {} contains {} points", i, circuit.len());
            total *= circuit.len();
        } else {
            break;
        }
    }

    (total as u64, 0)
}
