use std::collections::BinaryHeap;

#[derive(PartialEq, Clone, Copy, Eq, Ord, Debug)]
struct Node {
    idx: usize,
    dis: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(other.dis.cmp(&self.dis));
    }
}

pub fn dijkstra(map: &Vec<Vec<(usize, i32)>>, start: usize, n: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut vis = vec![false; n];
    let mut dis = vec![i32::MAX / 2; n];

    heap.push(Node { idx: start, dis: 0 });
    dis[start] = 0;

    while let Some(x) = heap.pop() {
        if vis[x.idx] {
            continue;
        }
        vis[x.idx] = true;
        for (v, d) in &map[x.idx] {
            if dis[*v] > dis[x.idx] + *d {
                dis[*v] = dis[x.idx] + *d;
                heap.push(Node {
                    idx: *v,
                    dis: dis[*v],
                });
            }
        }
    }
    dis
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        heap.push(Node { idx: 0, dis: 10 });
        heap.push(Node { idx: 0, dis: 0 });
        heap.push(Node { idx: 0, dis: 20 });

        assert_eq!(heap.peek(), Some(&Node { idx: 0, dis: 0 }));
    }

    #[test]
    fn test2() {
        let mut map: Vec<Vec<(usize, i32)>> = vec![vec![]; 4];
        map[0].extend_from_slice(&[(1, 10), (2, 3)]);
        map[1].extend_from_slice(&[(3, 1)]);
        map[2].extend_from_slice(&[(3, 30)]);
        let res = dijkstra(&map, 0, 4);
        assert_eq!(res, [0, 10, 3, 11]);
    }
}
