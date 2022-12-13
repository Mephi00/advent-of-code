use self::node::Node;

mod node;

pub fn main(input_str: &String) {
    let grid: Vec<Vec<&str>> = input_str
        .split("\n")
        .map(|x| x.split("").filter(|f| !f.is_empty()).collect())
        .collect();

    let mut unexplored: Vec<Vec<Node>> = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y_axis in 0..grid.len() {
        let mut vec_buf = Vec::new();
        for x_axis in 0..grid[y_axis].len() {
            if first_char(grid[y_axis][x_axis]) == 'S' {
                vec_buf.push(Node::new_start((y_axis, x_axis)));
                start = (y_axis, x_axis);
                continue;
            } else if first_char(grid[y_axis][x_axis]) == 'E' {
                vec_buf.push(Node::new((y_axis, x_axis), 'z' as u8));
                end = (y_axis, x_axis);
                continue;
            }
            vec_buf.push(Node::new(
                (y_axis, x_axis),
                first_char(grid[y_axis][x_axis]) as u8,
            ));
        }
        if !vec_buf.is_empty() {
            unexplored.push(vec_buf);
        }
    }

    println!(
        "found the end after {} stept",
        perform_dijkstra_search(
            start,
            Box::new(move |check| { check.position.0 == end.0 && check.position.1 == end.1 }),
            unexplored.clone(),
            Box::new(move |x, curr_node| x.height <= curr_node.height + 1)
        )
    );

    unexplored[start.0][start.1].shortest_path = usize::MAX;
    unexplored[end.0][end.1].shortest_path = 0;

    println!(
        "steps to an end point with height of 'a': {}",
        perform_dijkstra_search(
            end,
            Box::new(move |check| { check.height == ('a' as u8) }),
            unexplored,
            Box::new(move |x, curr_node| x.height >= curr_node.height - 1)
        )
    )
}

fn first_char(str: &str) -> char {
    str.chars().collect::<Vec<char>>()[0]
}

fn get_neighbour_nodes(node: &Node, size_y: usize, size_x: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    if !(node.position.0 + 1 >= size_y) {
        ret.push((node.position.0 + 1, node.position.1));
    }

    if !(node.position.1 + 1 >= size_x) {
        ret.push((node.position.0, node.position.1 + 1))
    }

    if !(node.position.1 == 0) {
        ret.push((node.position.0, node.position.1 - 1))
    }

    if !(node.position.0 == 0) {
        ret.push((node.position.0 - 1, node.position.1));
    }

    ret
}

fn get_next_node_pos(nodes: &Vec<Vec<Node>>) -> (usize, usize) {
    let mut shortest = (0, 0);
    let mut shortest_len = usize::MAX;
    for (idx_y, node_row) in nodes.iter().enumerate() {
        for (idx_x, node) in node_row.iter().enumerate() {
            if node.visited() {
                continue;
            } else if shortest_len > node.shortest_path {
                shortest_len = node.shortest_path;
                shortest = (idx_y, idx_x);
            }
        }
    }

    shortest
}

fn perform_dijkstra_search(
    start: (usize, usize),
    check_if_end: Box<dyn Fn(Node) -> bool>,
    mut unexplored: Vec<Vec<Node>>,
    height_filter: Box<dyn Fn(&Node, &Node) -> bool>,
) -> usize {
    let mut curr_node = unexplored[start.0][start.1];

    loop {
        let theo_neighbours =
            get_neighbour_nodes(&curr_node, unexplored.len(), unexplored[0].len());

        let neighbours: Vec<Node> = theo_neighbours
            .iter()
            .filter(|x| !unexplored[x.0][x.1].visited())
            .map(|x| unexplored[x.0][x.1])
            .filter(|x| height_filter(x, &curr_node))
            .collect();

        for mut neighbour in neighbours {
            if neighbour.shortest_path > curr_node.shortest_path + 1 {
                neighbour.shortest_path = curr_node.shortest_path + 1;

                unexplored[neighbour.position.0][neighbour.position.1] = neighbour;
            }
        }

        unexplored[curr_node.position.0][curr_node.position.1].visit();

        if check_if_end(curr_node) {
            return curr_node.shortest_path;
        } else {
            let next_pos = get_next_node_pos(&unexplored);
            curr_node = unexplored[next_pos.0][next_pos.1];
        }
    }
}
