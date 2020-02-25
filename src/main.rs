use std::{
    collections::VecDeque,
    iter::FromIterator,
};

#[derive(Debug)]
struct LazyBuffer<T> {
    buf: VecDeque<T>,
    // kids: [(left_idx, subtree)]
    // `left_idx` is index of element in *resulting* buffer
    // before which the subtree is inserted
    kids: Vec<(usize, Box<LazyBuffer<T>>)>
}

impl<T: std::fmt::Debug> LazyBuffer<T> {
    fn new<B>(buf: B) -> Self
        where
            B: IntoIterator<Item=T>
    {
        Self {
            buf: VecDeque::from_iter(buf),
            kids: Default::default()
        }
    }
    fn insert(&mut self, idx: usize, data: impl IntoIterator<Item=T>) {
        let elem = (*self.kids).iter_mut().enumerate().rev()
            .find(|(kid_idx, (left_idx, _))| {
                *left_idx <= idx
            });
        let new_buf = Box::new(Self::new(data));
        let (kid_idx, (left_idx, kid)) = match elem {
            None => {
                self.kids.push((idx, new_buf));
                return;
            },
            Some(val) => val
        };
        match dbg!((*self.kids).get_mut(kid_idx + 1..)) {
            None => return,
            Some(val) => val
        }.iter_mut().for_each(|(left_idx, _)| {
            println!("left_idx = {}", left_idx);
            *left_idx += kid_idx;
        });
    }
}

fn main() {
    let mut lb: LazyBuffer<u8> = LazyBuffer::new("Hello world!".to_owned().into_bytes());
    lb.insert(11, " war".bytes());
    lb.insert(4, " n".bytes());
    // lb.apply();
    println!("{:#?}", lb);
}
