extern crate rand;

pub mod network;
pub mod node;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use std::thread;

    #[test]
    fn comapre() {
        for _ in 0..1000 {
            let (x, y): (u32, u32) = (random(), random());
            let mut nodes = vec![node::Node::new(x), node::Node::new(y)];
            let mut handles = Vec::new();

            let links = &[(0, 1)];
            network::connect(&mut nodes[..], links);

            for node in nodes {
                handles.push(thread::spawn(move || node.compare()));
            }

            for handle in handles {
                match handle.join() {
                    Ok(result) => {
                        if x < y {
                            assert_eq!(result.unwrap(), y);
                        } else {
                            assert_eq!(result.unwrap(), x);
                        }
                    }
                    Err(error) => println!("{:?}", error),
                };
            }
        }
    }
}
