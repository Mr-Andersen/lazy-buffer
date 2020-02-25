use std::{collections::VecDeque, iter::FromIterator};

#[derive(Debug)]
struct LazyBufferEdge<T> {
    // index of left-most element in *resulting* buffer
    left_idx: usize,
    // size of this piece of buffer (including sizes of kids)
    size: usize,
    vertex: Box<LazyBuffer<T>>,
}

#[derive(Debug)]
struct LazyBuffer<T> {
    buf: VecDeque<T>,
    kids: Vec<LazyBufferEdge<T>>,
}

impl<T: std::fmt::Debug> LazyBuffer<T> {
    fn new<B>(buf: B) -> Self
    where
        B: IntoIterator<Item = T>,
    {
        Self {
            buf: VecDeque::from_iter(buf),
            kids: Default::default(),
        }
    }
    fn insert<I>(&mut self, target_idx: usize, data: I)
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        let candidate_idx = self
            .kids
            .binary_search_by(|edge| edge.left_idx.cmp(&target_idx));
        match candidate_idx {
            Ok(exact_idx) => {
                self.kids[exact_idx].size += data.len();
                self.kids.get_mut(exact_idx + 1..).map(|sl| {
                    sl.iter_mut().for_each(|edge| edge.left_idx += data.len())
                });
                self.kids[exact_idx].vertex.insert(0, data);
            }
            Err(0) => {
                self.kids
                    .iter_mut()
                    .for_each(|edge| edge.left_idx += data.len());
                self.kids.insert(
                    0,
                    LazyBufferEdge {
                        left_idx: target_idx,
                        size: data.len(),
                        vertex: Box::new(LazyBuffer::new(data)),
                    },
                );
            }
            // approx_idx > 0
            Err(approx_idx) => {
                self.kids.get_mut(approx_idx..).map(|sl| {
                    sl.iter_mut().for_each(|edge| edge.left_idx += data.len())
                });
                let parent = &mut self.kids[approx_idx - 1];
                if parent.left_idx + parent.size <= target_idx {
                    // parent is too "left"
                    self.kids.insert(
                        approx_idx,
                        LazyBufferEdge {
                            left_idx: target_idx,
                            size: data.len(),
                            vertex: Box::new(LazyBuffer::new(data)),
                        },
                    );
                } else {
                    parent.size += data.len();
                    parent.vertex.insert(target_idx - parent.left_idx, data);
                }
            }
        }
    }
}

fn main() {
    let mut lb: LazyBuffer<u8> =
        LazyBuffer::new("Hello world!".to_owned().into_bytes());
    lb.insert(11, " war".bytes());
    lb.insert(4, " n".bytes());
    // lb.apply();
    println!("{:#?}", lb);
}
