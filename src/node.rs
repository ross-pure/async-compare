use super::network;
use std::collections::HashMap;
use std::sync::mpsc;

enum Message<T, U> {
    New((T, U)),
    Max((T, U)),
}

pub struct Node<T, U, V> {
    id: T,
    n: U,
    senders: Vec<mpsc::Sender<V>>,
    receivers: Vec<mpsc::Receiver<V>>,
}

impl<T, U> Node<T, U, Message<T, U>>
where
    T: Eq + ::std::hash::Hash + Copy,
    U: Ord + Copy,
{
    pub fn new(id: T, n: U) -> Node<T, U, Message<T, U>> {
        Node {
            n,
            id,
            senders: Vec::new(),
            receivers: Vec::new(),
        }
    }

    pub fn set_number(&mut self, new: U) {
        self.n = new;
    }

    /* pub fn compare(&self) -> Option<T> {
        match (self.sender.as_ref(), self.receiver.as_ref()) {
            (None, _) | (_, None) => panic!("Must set sender and receiver before using node"),
            _ => (),
        };

        if let Err(_) = self.sender.as_ref().unwrap().send(self.n) {
            return None;
        }
        
        match self.receiver.as_ref().unwrap().recv() {
            Err(_) => None,
            Ok(value) => {
                if self.n > value {
                    Some(self.n)
                } else {
                    Some(value)
                }
            }
        }
    } */

    pub fn max_number(&self, n: usize) -> Option<U> {
        for sender in self.senders.iter() {
            if let Err(_) = sender.send(Message::New((self.id, self.n))) {
                panic!("Could not send on channel");
            }
        }

        let max: Option<U>;
        let mut known_values: HashMap<T, U> = HashMap::new();
        let mut new_values: Vec<U> = Vec::new();

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
                            None => new_values.push(value),
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
                }

                for val in new_values.into_iter() {
                    if let Err(_) = sender.send(Message::New((self.id, val))) {
                        panic!("Tried to send to a closed channel");
                    }
                }

                new_values.clear();
            }

            if let Some(m) = max {
                return Some(m);
            }
        }

        match known_values.values().max() {
            None => panic!("Could not find the maximum of the received values"),
            Some(m) => Some(*m),
        }
    }
}

impl<T, U, V> network::Connectable for Node<T, U, V> {
    type Item = V;

    fn set_tx(&mut self, tx: mpsc::Sender<V>) {
        self.senders.push(tx);
    }
    fn set_rx(&mut self, rx: mpsc::Receiver<V>) {
        self.receivers.push(rx);
    }
}
