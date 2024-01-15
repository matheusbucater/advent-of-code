#[derive(Copy, Clone)]
struct Node<'a> {
    main: &'a str,
    left: &'a str,
    rigth: &'a str
}

fn create_vec_of_nodes(nodes: Vec<&str>) -> Vec<Node> {
    let mut nodes_vec: Vec<Node> = vec![];
    for node in nodes {
        let main = &node[0..=2];
        let left = &node[7..=9];
        let rigth = &node[12..=14];
        let node_item = Node { main, left, rigth };
        nodes_vec.push(node_item);
    }
    nodes_vec
}

pub fn main(input: &str) -> String  { 
    
    let directions: &str = input.split("\n\n").collect::<Vec<&str>>()[0];
    let nodes: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>();
    let nodes_vec = create_vec_of_nodes(nodes);

    let directions_length = directions.len();
    let mut count: u32 = 0;

    let mut node: &Node = &nodes_vec.into_iter().filter(|x| x.main == "AAA".to_string()).collect::<Vec<Node>>()[0];

    loop {
        let index = (count as usize) % directions_length;
        let direction = directions.chars().nth(index).unwrap();
        node = match direction {
            'R' => nodes_vec.into_iter().filter(|x| x.main == node.rigth).collect::<Vec<Node>>()[0],
            'L' => nodes_vec.into_iter().filter(|x| x.main == node.left).collect::<Vec<Node>>()[0],
            _ => continue,
        };

        if node.main == "ZZZ".to_string() {
            break;
        }

        count += 1;
    }

    count.to_string()


}
