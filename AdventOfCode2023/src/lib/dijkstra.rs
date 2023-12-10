use itertools::Itertools;

#[derive(Debug)]
struct Node {
    visited: bool,
    goal: bool,
    wall: bool,
}

impl From<char> for Node {
    fn from(char: char) -> Node {
        Node {
            visited: false,
            goal: char == 'G',
            wall: char == 'W',
        }
    }
}
struct Map {
    nodes: Vec<Vec<Node>>,
    x_size: usize,
    y_size: usize,
}

impl From<Vec<&str>> for Map {
    fn from(value: Vec<&str>) -> Self {

        let y_size = value.len();   

        let x_size = value.iter().max_by_key(|x| x.len()).unwrap().len();

        // To account for vectors where the lines all aren't the same size
        let checked_value = value.iter().map(|x: &&str| format!("{:W<width$}", x, width = x_size));

        
        let nodes = checked_value
            .map(|line| {
                line.chars()
                    .map(|chr| Node::from(chr))
                    .collect_vec()
            })
            .collect_vec();

        Map {
            nodes: nodes,
            y_size: y_size,
            x_size: x_size,
        }
    }
}

fn node_path<'map>(map: &'map mut Map, (x, y): (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let node = &mut map.nodes[y][x];

    if node.goal {
        return Some(vec![(x, y)]);
    }

    node.visited = true;

    if (x, y) ==(6,3) {
        println!("aaa {}", map.nodes[3][7].visited);
    }
    let mut results = Vec::new();
    let paths = {
        let paths = [0, 1, -1]
            .iter()
            .map(|x_addition| {
                [0, 1, -1]
                    .iter()
                    .map(|y_addition| {
                        if *x_addition == 0 && *y_addition == 0 {
                            return None;
                        }

                        if (*x_addition < 0 && x == 0) || (*y_addition < 0 && y == 0) {
                            return None;
                        }

                        let new_x = x as i64 + *x_addition;
                        let new_y = y as i64 + *y_addition;

                        if (new_x >= map.x_size as i64) || (new_y >= map.y_size as i64) {
                            return None;
                        }
                        let new_node = &map.nodes[new_y as usize][new_x as usize];

                        if new_node.visited || new_node.wall {
                            return None;
                        }

                        Some((new_x, new_y))
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
            })
            .flatten()
            .collect_vec();
        paths
    };
    for path in paths {
        let result = node_path(map, (path.0 as usize, path.1 as usize));

        if (x,y) == (5,3) {
            println!("{:?}, {:?}", path, result);
        }

        if let Some(val) = result.clone() {
            for loc in val.iter() {

                if loc == &(path.0 as usize, path.1 as usize) { break; }
                
                let node = &mut map.nodes[loc.1][loc.0];
                node.visited = false;

            }
        }
        results.push(result);
    }   

    let paths = results.into_iter().flatten();

    let mut best_path = paths
        .min_by_key(|x| x.len())?
        .clone();
    
    best_path.push((x, y));
    
    Some(best_path)
}
fn dijkstra(mut map: Map, (x, y): (usize, usize)) -> Option<Vec<(usize, usize)>>{
    //-> (usize, Vec<&mut Node>){
    let path = node_path(&mut map, (x, y));

    return match path {
        Some(x) => Some(x.iter().rev().map(|x| x.clone()).collect_vec().clone()),
        None => None,
    };

}
