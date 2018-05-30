extern crate rand;

pub mod network;
pub mod node;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use std::thread;

    const NODES: usize = 10;

    #[test]
    fn comapre() {
        for _ in 0..1 {
            let mut numbers: Vec<u32> = Vec::new();
            let mut nodes = Vec::new();
            let mut handles = Vec::new();

            for i in 0..NODES {
                numbers.push(random());
                nodes.push(node::Node::new(i, numbers.last().unwrap().clone()));
            }

            println!("Max is {:?}", numbers.iter().max().unwrap());

            let links = &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 0)];
            network::connect(&mut nodes[..], links);

            for node in nodes {
                handles.push(thread::spawn(move || node.max_number(NODES)));
            }

            for handle in handles {
                match handle.join() {
                    Err(error) => println!("{:?}", error),
                    _ => (),
                };
            }
        }
    }
}
