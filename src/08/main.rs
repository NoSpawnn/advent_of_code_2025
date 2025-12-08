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
        let [x, y, z] = value
            .splitn(3, ',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { x, y, z }
    }
}

impl JunctionBox {
    // distance = sqrt( (x1 - x2)^2 + (y1 - y2)^2 + (z1 - z2)^2 )
    // but we store the square distance to save sqrt() computation
    // https://math.stackexchange.com/questions/42640/calculate-distance-in-3d-space
    //   namely https://math.stackexchange.com/a/1069627
    fn euclid_distance_to(&self, other: &JunctionBox) -> usize {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        let z = self.z.abs_diff(other.z).pow(2);
        x + y + z
    }
}

fn connect<'a>(
    from: &'a JunctionBox,
    to: &'a JunctionBox,
    circuits: &mut Vec<Vec<&'a JunctionBox>>,
) {
    let circuit_contains_from = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.contains(&from))
        .map(|(idx, _)| idx);
    let circuit_contains_to = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.contains(&to))
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
            circuits.push(Vec::from([from, to]));
        }
    }
}

/// Returns all possible connections sorted ascending by euclid distance
fn connection_combinations_by_len<'a>(
    points: &'a [JunctionBox],
) -> Vec<(&'a JunctionBox, &'a JunctionBox)> {
    let mut possible_connections: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, j1)| points[i + 1..].iter().map(move |j2| (j1, j2)))
        .collect();
    possible_connections.sort_by(|j1, j2| {
        let d1 = j1.0.euclid_distance_to(j1.1);
        let d2 = j2.0.euclid_distance_to(j2.1);
        d1.cmp(&d2)
    });
    possible_connections
}

pub fn part_1(input: &str) -> Answer {
    const N_CONNECTIONS: usize = 1000;
    let junction_boxes: Vec<_> = input.lines().map(JunctionBox::from).collect();
    let possible_connections = connection_combinations_by_len(&junction_boxes);
    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();

    possible_connections
        .iter()
        .take(N_CONNECTIONS)
        .for_each(|(from, to)| connect(from, to, &mut circuits));

    // sort descending by circuit length
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits.into_iter().take(3).map(|it| it.len()).product()
}

pub fn part_2(input: &str) -> Answer {
    let junction_boxes: Vec<_> = input.lines().map(JunctionBox::from).collect();
    let target_len = junction_boxes.len();
    let possible_connections = connection_combinations_by_len(&junction_boxes);
    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();

    let last = possible_connections.into_iter().cycle().find(|(from, to)| {
        connect(from, to, &mut circuits);
        circuits[0].len() == target_len
    });

    if let Some(last) = last {
        last.0.x * last.1.x
    } else {
        unreachable!()
    }
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
