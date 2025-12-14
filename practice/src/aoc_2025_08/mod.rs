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
        //result.sqrt()
        result
    }
}

pub fn solution(file_path: &str, total_distances: usize) -> u64 {

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
    
    let mut circuits:HashMap<u64, HashSet<&Point3D>> = HashMap::new();
    let mut circuits_by_point: HashMap<&Point3D, u64> = HashMap::new();

    let mut circuit_id = 0;
    let mut processed = 0;
    for count in 0..distances.len(){
        
        let pairs = distances.values().nth(count).unwrap();
        
        for (point1, point2) in pairs {
            // is one of these points part of a circuit?
            circuit_id = if circuits_by_point.contains_key(point1) {

                let circuit_id1 = circuits_by_point.get(point1).unwrap().clone();
                
                if circuits_by_point.contains_key(point2) {
                    let circuit_id2 = circuits_by_point.get(point2).unwrap().clone();
                    
                    if circuit_id1 != circuit_id2 {
                        println!("Merging circuits!!");

                        let mut circuit1 = circuits.get(&circuit_id1).unwrap().clone();
                        let circuit2 = circuits.remove(&circuit_id2).unwrap();
    
                        println!("Circuit Id 1: {:?}", circuit_id1);
                        println!("{:?}", circuit1);
                        println!("Circuit Id 2: {:?}", circuit_id2);
                        println!("{:?}", circuit2);
                        
                        for point in circuit2.iter() {
                            circuit1.insert(point);
                        }
                        
                        println!("MERGED {:?}", circuit1);
    
                        circuits.insert(circuit_id1, circuit1);
                        
                        // reindex the circuit IDs
                        circuits_by_point = HashMap::new();
                        for (i, circuit) in circuits.iter() {
                            for point in circuit.iter() {
                                //println!("Insert Point: {:?} to circuit {}", point, i);
                                circuits_by_point.insert(point, *i);
                            }
                        }
                    }
                }
                
                circuit_id1
                
            } else if circuits_by_point.contains_key(point2) {
                circuits_by_point.get(point2).unwrap().clone()
            } else {                
                circuits.insert(circuit_id+1, HashSet::new());
                circuit_id+1
            };
            
            let circuit = circuits.get_mut(&circuit_id).unwrap();
            println!("----{}----", processed);
            println!("Circuit ID: {}", circuit_id);
            println!("Point1: {:?}", point1);
            println!("Point2: {:?}", point2);
            circuit.insert(point1);
            circuit.insert(point2);
            circuits_by_point.insert(point1, circuit_id);
            circuits_by_point.insert(point2, circuit_id);
            println!("Circuit ID: {} now contains {} points", circuit_id, circuit.len());
            println!("{:?}", circuit);
            processed += 1;
            if processed > total_distances {
                break;
            }
        }
        if processed > total_distances {
            break;
        }
    }
        
    // order the circuits by size
    let mut sorted_circuits: Vec<(u64, HashSet<&Point3D>)> = circuits.into_iter().collect();
    sorted_circuits.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    let total = sorted_circuits[0].1.len() * sorted_circuits[1].1.len() * sorted_circuits[2].1.len();

    total as u64
}
