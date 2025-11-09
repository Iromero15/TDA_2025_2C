use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{self, Read};


fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n + 1];
    let mut ban: Vec<HashSet<u64>> = vec![HashSet::new(); n + 1];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: u64   = it.next().unwrap().parse().unwrap();
        adj[a].push((b, c));
        adj[b].push((a, c));
    }

    for i in 1..=n {
        let k: usize = it.next().unwrap().parse().unwrap();
        let mut h = HashSet::with_capacity(k);
        for _ in 0..k {
            h.insert(it.next().unwrap().parse::<u64>().unwrap());
        }
        ban[i] = h;
    }

    let dist = djikstra(&adj, &ban, n);

    // println!("imprimiendo dist");
    // for i in 0..(n+1){
    //     println!("{i} : {}", dist[i]);
    // }
    if dist[n] < u64::MAX {
        println!("{}",dist[n]); 
    }
    else {
        println!("{}",-1);
    }

}
fn dep(mut t: u64, b: &HashSet<u64>) -> u64 {
    while b.contains(&t) { t += 1; }
    t
}

fn djikstra(adj: &Vec<Vec<(usize, u64)>>, ban: &Vec<HashSet<u64>>, n: usize) -> Vec<u64> {
    
    let mut dist: Vec<u64> = vec![u64::MAX; n + 1];
    let mut heap: BinaryHeap<(Reverse<u64>, usize)> = BinaryHeap::new();

    dist[1] = 0;
    heap.push((Reverse(0), 1));

    while let Some((Reverse(cost), u)) = heap.pop() {
        if cost > dist[u] {
            continue;
        }
        if u == n {
            break;
        }
        let departure_time = dep(cost, &ban[u]);

        for &(v, w) in &adj[u] {
            let arrival_at_v = departure_time + w; 

            if arrival_at_v < dist[v] {
                dist[v] = arrival_at_v;
                heap.push((Reverse(arrival_at_v), v));
            }
        }
    }

    dist
}