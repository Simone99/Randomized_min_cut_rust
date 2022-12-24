use rand::Rng;

const EDGE_PERCENTAGE: usize = 3;

#[derive(Clone)]
pub struct Graph {
    n: usize,
    e: usize,
    vertex_cluster: Vec<usize>,
    edge_array: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        let mut tmp = Graph {
            n: n,
            e: 0,
            vertex_cluster: Vec::new(),
            edge_array: Vec::new(),
        };
        for i in 0..n {
            tmp.vertex_cluster.push(i);
        }
        tmp
    }

    pub fn remove_edge(&mut self, index: usize) {
        self.edge_array.swap(index, self.e - 1);
        self.e -= 1;
    }

    pub fn get_n_edges(&self) -> usize {
        self.e
    }

    pub fn get_n_nodes(&self) -> usize {
        self.n
    }

    pub fn generate(&mut self) {
        if self.e == 0 {
            let mut rng = rand::thread_rng();
            for i in 0..self.n {
                for j in 0..self.n {
                    if rng.gen_range(0, 100) < EDGE_PERCENTAGE {
                        self.edge_array.push(vec![i, j]);
                        self.e += 1;
                    }
                }
            }
        }
    }

    pub fn is_edge_in_min_cut(&self, mut cluster_u: usize, mut cluster_v: usize) -> bool {
        loop {
            if cluster_u == self.vertex_cluster[cluster_u] {
                break;
            }
            cluster_u = self.vertex_cluster[cluster_u];
        }
        loop {
            if cluster_v == self.vertex_cluster[cluster_v] {
                break;
            }
            cluster_v = self.vertex_cluster[cluster_v];
        }
        cluster_u != cluster_v
    }

    pub fn min_cut_size(&self) -> usize {
        let mut counter: usize = 0;
        for i in 0..self.e {
            if self.is_edge_in_min_cut(self.edge_array[i][0], self.edge_array[i][1]) {
                counter += 1;
            }
        }
        counter
    }

    pub fn merge_nodes(&mut self, edge_index: usize) {
        let mut cluster_u: usize;
        let mut cluster_v: usize;
        cluster_u = self.edge_array[edge_index][0];
        loop {
            if cluster_u == self.vertex_cluster[cluster_u] {
                break;
            }
            cluster_u = self.vertex_cluster[cluster_u];
        }
        cluster_v = self.edge_array[edge_index][1];
        loop {
            if cluster_v == self.vertex_cluster[cluster_v] {
                break;
            }
            cluster_v = self.vertex_cluster[cluster_v];
        }
        if cluster_u != cluster_v {
            if cluster_u < cluster_v {
                self.vertex_cluster[cluster_u] = cluster_v;
            } else {
                self.vertex_cluster[cluster_v] = cluster_u;
            }
            self.n -= 1;
        }
    }
}
