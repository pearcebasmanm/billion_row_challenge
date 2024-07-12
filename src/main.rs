use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut map = BTreeMap::new();
    let lines =
        read_lines("./weather_stations.csv").expect("weather station lines couldn't be read");
    for line in lines.flatten() {
        let (city, temp) = line.split_once(';').unwrap();
        let temp: f64 = temp.parse().unwrap();
        map.entry(city.to_string())
            .and_modify(|(min, max, sum, count): &mut (f64, f64, f64, u32)| {
                *min = min.min(temp);
                *max = max.max(temp);
                *sum += temp;
                *count += 1;
            })
            .or_insert((temp, temp, temp, 1));
    }
    print!("{{");
    for (city, (min, max, sum, count)) in map.into_iter() {
        let mean = sum / count as f64;
        print!("{city}={min}/{mean}/{max}, ");
    }
    print!("}}")
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
