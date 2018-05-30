extern crate futures;

use super::network;
use std::collections::HashMap;
//use std::sync::mpsc;
use node::futures::channel::mpsc;

pub enum Message<T> {
    New((T, u32)),
    Max((T, u32)),
}

pub struct Node<T> {
    id: T,
    n: u32,
    senders: Vec<mpsc::Sender<Message<T>>>,
    receivers: Vec<mpsc::Receiver<Message<T>>>,
}

impl<T> Node<T>
where
    T: Eq + ::std::hash::Hash + Copy + ::std::fmt::Display,
{
    pub fn new(id: T, n: u32) -> Node<T> {
        Node {
            n,
            id,
            senders: Vec::new(),
            receivers: Vec::new(),
        }
    }

    pub fn set_number(&mut self, new: u32) {
        self.n = new;
    }

    pub fn compare(&self, n: usize) {

    }

    /* pub fn max_number(&self, n: usize) {
        for sender in self.senders.iter() {
            if let Err(_) = sender.send(Message::New((self.id, self.n))) {
                panic!("Could not send on channel");
            }
        }

        let mut max: Option<U> = None;
        let mut known_values: HashMap<T, U> = HashMap::new();
        let mut new_values: Vec<(T, U)> = Vec::new();

        known_values.insert(self.id, self.n);

        while known_values.len() < n {
            // Iterate over receivers.
            for receiver in self.receivers.iter() {
                match receiver.try_recv() {
                    Err(error) => match error {
                        mpsc::TryRecvError::Disconnected => {
                            panic!("Tried to read form a disconnected channel")
                        }
                        _ => (),
                    },
                    Ok(msg) => match msg {
                        Message::New((id, value)) => match known_values.insert(id, value) {
                            None => {
                                new_values.push((id, value));
                            },
                            _ => (),
                        },
                        Message::Max((_, value)) => max = Some(value),
                    },
                };
            }

            // Iterate over senders.
            for sender in self.senders.iter() {
                if let Some(m) = max {
                    if let Err(_) = sender.send(Message::Max((self.id, m))) {
                        panic!("Tried to send to a closed channel");
                    }
                } else {
                    for val in new_values.iter() {
                        if let Err(_) = sender.send(Message::New(*val)) {
                            panic!("Tried to send to a closed channel");
                        }
                    }
                }
            }
            new_values.clear();

            if let Some(m) = max {
                println!("{} decided {}", self.id, m);
            }
        }

        match known_values.values().max() {
            None => panic!("Could not find the maximum of the received values"),
            Some(m) => println!("{} decided {}", self.id, m),
        };

        // Wait for other nodes to finish.
        loop {}
    } */
}

impl<T> network::Connectable for Node<T> {
    type Item = Message<T>;
    fn set_tx(&mut self, tx: mpsc::Sender<Self::Item>) {
        self.senders.push(tx);
    }
    fn set_rx(&mut self, rx: mpsc::Receiver<Self::Item>) {
        self.receivers.push(rx);
    }
}
