use std::io::{self};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..=n).collect::<Vec<_>>();
        let rank = vec![0; n + 1];
        Self { parent, rank }
    }

    fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb { return false; }
        if self.rank[ra] < self.rank[rb] { std::mem::swap(&mut ra, &mut rb); }
        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] { self.rank[ra] += 1; }
        true
    }
}

fn calcular_ops_agm_unico(n: usize, edges: &Vec<(usize, usize, u64)>) -> usize {
    let m = edges.len();

    let mut indices: Vec<usize> = (0..m).collect();
    indices.sort_by_key(|&i| edges[i].2);

    let mut dsu = UnionFind::new(n);
    let mut in_mst = vec![false; m];
    let mut mst_graph: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n + 1];
    let mut aristas_en_agm = 0;

    for &i in &indices {
        let (u, v, w) = edges[i];
        if dsu.unite(u, v) {
            in_mst[i] = true;
            mst_graph[u].push((v, w));
            mst_graph[v].push((u, w));
            aristas_en_agm += 1;
            if aristas_en_agm == n - 1 { break; }
        }
    }

    const LOG_N: usize = 20;
    let mut ancestro = vec![vec![0usize; n + 1]; LOG_N];
    let mut max_arista = vec![vec![0u64;   n + 1]; LOG_N];
    let mut depth = vec![0usize; n + 1];

    let mut visited = vec![false; n + 1];
    let mut stack = vec![1usize];
    visited[1] = true;
    ancestro[0][1] = 0;
    max_arista[0][1] = 0;

    while let Some(u) = stack.pop() {
        for &(v, w) in &mst_graph[u] {
            if !visited[v] {
                visited[v] = true;
                depth[v] = depth[u] + 1;
                ancestro[0][v] = u;
                max_arista[0][v] = w;
                stack.push(v);
            }
        }
    }

    for k in 1..LOG_N {
        for v in 1..=n {
            let mid = ancestro[k - 1][v];
            ancestro[k][v] = ancestro[k - 1][mid];
            max_arista[k][v] = max_arista[k - 1][v].max(max_arista[k - 1][mid]);
        }
    }

    let max_en_camino = |mut a: usize, mut b: usize| -> u64 {
        let mut ans = 0u64;
        if depth[a] < depth[b] { std::mem::swap(&mut a, &mut b); }

        let mut diff = depth[a] - depth[b];
        let mut k = 0;
        while diff > 0 {
            if (diff & 1) == 1 {
                ans = ans.max(max_arista[k][a]);
                a = ancestro[k][a];
            }
            diff >>= 1;
            k += 1;
        }
        if a == b { return ans; }

        for k in (0..LOG_N).rev() {
            if ancestro[k][a] != ancestro[k][b] {
                ans = ans.max(max_arista[k][a]); a = ancestro[k][a];
                ans = ans.max(max_arista[k][b]); b = ancestro[k][b];
            }
        }
        ans = ans.max(max_arista[0][a]);
        ans = ans.max(max_arista[0][b]);
        ans
    };

    let mut operaciones = 0usize;
    for i in 0..m {
        if !in_mst[i] {
            let (u, v, w) = edges[i];
            if max_en_camino(u, v) == w {
                operaciones += 1;
            }
        }
    }
    operaciones
}

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Error al leer la primera línea");

    let mut numbers = input_line.split_whitespace();

    let n: usize = numbers
        .next()
        .expect("No se encontró el número de vértices (n)")
        .parse()
        .expect("No se pudo parsear 'n'");

    let m: usize = numbers
        .next()
        .expect("No se encontró el número de aristas (m)")
        .parse()
        .expect("No se pudo parsear 'm'");

    let mut edges: Vec<(usize, usize, u64)> = Vec::with_capacity(m);

    for _ in 0..m {
        input_line.clear();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Error al leer una línea de arista");

        let mut parts = input_line.split_whitespace();

        let u: usize = parts.next().expect("No se encontró u").parse().expect("parse u");
        let v: usize = parts.next().expect("No se encontró v").parse().expect("parse v");
        let w: u64   = parts.next().expect("No se encontró w").parse().expect("parse w");

        edges.push((u, v, w));
    }

    println!("{}", calcular_ops_agm_unico(n, &edges));
}
