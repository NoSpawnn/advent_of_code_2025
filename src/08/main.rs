// https://adventofcode.com/2025/day/8

type Answer = usize;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let coords: Vec<&str> = value.splitn(3, ',').collect();
        assert!(coords.len() == 3, "input was not of the form 'x,y,z'");

        let x = coords[0].parse().expect("x should be numeric");
        let y = coords[1].parse().expect("y should be numeric");
        let z = coords[2].parse().expect("z should be numeric");

        Self { x, y, z }
    }
}

impl JunctionBox {
    // distance = sqrt( (x1 - x2)^2 + (y1 - y2)^2 + (z1 - z2)^2 )
    // https://math.stackexchange.com/questions/42640/calculate-distance-in-3d-space
    //   namely https://math.stackexchange.com/a/1069627
    fn euclid_distance_to(&self, other: &JunctionBox) -> f32 {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        let z = self.z.abs_diff(other.z).pow(2);
        (x + y + z) as f32
    }

    fn euclid_distance_to_closest<'a>(&self, other: &'a [JunctionBox]) -> f32 {
        let closest = self.find_closest(other);
        self.euclid_distance_to(closest)
    }

    fn find_closest<'a>(&self, other: &'a [JunctionBox]) -> &'a JunctionBox {
        other
            .iter()
            .filter(|other| *other != self)
            .min_by(|cur, next| {
                let cur_dist = self.euclid_distance_to(cur);
                let next_dist = self.euclid_distance_to(next);
                cur_dist.total_cmp(&next_dist)
            })
            .expect("other should not be empty")
    }
}

pub fn part_1(input: &str) -> Answer {
    const N_CONNECTIONS: usize = 1000;
    let junction_boxes: Vec<_> = input.lines().map(JunctionBox::from).collect();
    let mut possible_connections: Vec<_> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, j1)| junction_boxes[i + 1..].iter().map(move |j2| (j1, j2)))
        .collect();
    possible_connections.sort_by(|j1, j2| {
        let d1 = j1.0.euclid_distance_to(j1.1);
        let d2 = j2.0.euclid_distance_to(j2.1);
        d1.total_cmp(&d2)
    });
    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();

    for (from, to) in possible_connections.iter().take(N_CONNECTIONS) {
        let circuit_contains_from = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(from))
            .map(|(idx, _)| idx);
        let circuit_contains_to = circuits
            .iter()
            .enumerate()
            .find(|(_, circuit)| circuit.contains(to))
            .map(|(idx, _)| idx);

        match (circuit_contains_from, circuit_contains_to) {
            (Some(from_idx), Some(to_idx)) if from_idx != to_idx => {
                // connect 2 circuits
                let from = circuits.remove(from_idx.max(to_idx));
                let to = circuits.remove(from_idx.min(to_idx));
                let new = from.into_iter().chain(to.into_iter()).collect();
                circuits.push(new);
            }
            (Some(_), Some(_)) => {
                // already in the same circuit, do nothing
            }
            (Some(from_idx), None) => {
                // extend circuit
                let circuit = circuits.get_mut(from_idx).unwrap();
                circuit.push(to);
            }
            (None, Some(to_idx)) => {
                // extend circuit
                let circuit = circuits.get_mut(to_idx).unwrap();
                circuit.push(from);
            }
            (None, None) => {
                // new circuit
                circuits.push(Vec::from([*from, *to]));
            }
        }
    }

    // sort descending by length
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits.iter().take(3).map(|it| it.len()).product()
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 8 part 2")
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}
