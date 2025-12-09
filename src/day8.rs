const TAKE_FIRST_N_DISTANCES: usize = 1000;
const TAKE_FIRST_N_CIRCUITS: usize = 3;

pub fn solve(input: Vec<(i64, i64, i64)>, part2: bool) -> usize {
    let mut distances: Vec<(f64, usize, usize)> = vec![];
    for (i, from) in input.iter().rev().skip(1).rev().enumerate() {
        for (j, to) in input.iter().enumerate().skip(i + 1) {
            let part = (from.0 - to.0).pow(2) as f64
                + (from.1 - to.1).pow(2) as f64
                + (from.2 - to.2).pow(2) as f64;
            distances.push((part.sqrt(), i, j));
        }
    }
    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut connections: Vec<Vec<usize>> = vec![];
    let mut last = (0, 0);
    let iterate_for = if part2 {
        distances.len()
    } else {
        TAKE_FIRST_N_DISTANCES
    };
    for distance in distances.iter().take(iterate_for) {
        let mut inserted = false;
        for (i, connection) in connections.iter().enumerate() {
            if connection.contains(&distance.1) && connection.contains(&distance.2) {
                inserted = true;
                break;
            } else if connection.contains(&distance.1) {
                let mut inserted_inner = false;
                for (j, connection) in connections.iter().enumerate() {
                    if connection.contains(&distance.2) {
                        let popped = connections.remove(j);
                        connections[i].extend(popped.iter().cloned());
                        inserted_inner = true;
                        break;
                    }
                }
                if !inserted_inner {
                    connections[i].push(distance.2);
                }
                inserted = true;
                break;
            } else if connection.contains(&distance.2) {
                let mut inserted_inner = false;
                for (j, connection) in connections.iter().enumerate() {
                    if connection.contains(&distance.1) {
                        let popped = connections.remove(j);
                        connections[i].extend(popped.iter().cloned());
                        inserted_inner = true;
                        break;
                    }
                }
                if !inserted_inner {
                    connections[i].push(distance.1);
                }
                inserted = true;
                break;
            }
        }
        if !inserted {
            connections.push(vec![distance.1, distance.2]);
        }
        if connections[0].len() == input.len() {
            last = (distance.1, distance.2);
            break;
        }
    }
    connections.sort_by_key(|a| a.len());
    connections.reverse();

    if part2 {
        input[last.0].0 as usize * input[last.1].0 as usize
    } else {
        connections
            .iter()
            .take(TAKE_FIRST_N_CIRCUITS)
            .map(|v| v.len())
            .product::<usize>()
    }
}
