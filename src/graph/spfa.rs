use std::collections::VecDeque;

pub fn spfa(maps: &Vec<Vec<(usize, i32)>>, center: usize) -> Vec<i32> {
    let n = maps.len();
    let mut dis = vec![i32::MAX; n];
    let mut vis = vec![false; n];
    dis[center] = 0;

    let mut que = VecDeque::new();
    que.push_back(center);
    vis[center] = true;

    while let Some(pos) = que.pop_front() {
        vis[pos] = false;
        for (v, v_dis) in &maps[pos] {
            if dis[*v] > dis[pos] + v_dis {
                dis[*v] = dis[pos] + v_dis;
                if !vis[*v] {
                    vis[*v] = true;
                    que.push_back(*v);
                }
            }
        }
    }

    dis
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        /*
        0----1
        |    |
        |    |
        2----3
         */
        let maps: [[(usize, i32); 2]; 4] = [
            [(1, 1), (2, 3)],
            [(3, 10), (0, 1)],
            [(3, 1), (0, 3)],
            [(3, 0), (3, 0)],
        ];
        let vmaps = maps
            .iter()
            .map(|x| x.iter().map(|x| *x).collect())
            .collect();
        let res = super::spfa(&vmaps, 0);
        assert_eq!(res, [0, 1, 3, 4]);
    }
}
