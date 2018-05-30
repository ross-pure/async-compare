extern crate futures;

use network::futures::channel::mpsc;

pub trait Connectable {
    fn set_tx(&mut self, mpsc::Sender<u32>);
    fn set_rx(&mut self, mpsc::Receiver<u32>);
}

pub fn connect<T, U>(nodes: &mut [T], links: &[(usize, usize)])
where
    T: Connectable,
{
    let len = nodes.len();
    for (i, j) in links {
        if i > &len || j > &len {
            panic!("Link out of bounds");
        } else if i == j {
            panic!("Cannot connect node to itself");
        }

        let (first_tx, first_rx) = mpsc::channel::<u32>(1000);
        let (second_tx, second_rx) = mpsc::channel::<u32>(1000);

        nodes[*i].set_tx(first_tx);
        nodes[*i].set_rx(second_rx);
        nodes[*j].set_tx(second_tx);
        nodes[*j].set_rx(first_rx);
    }

}
