use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl TryFrom<&str> for Node {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "start" {
            Ok(Self::Start)
        } else if value == "end" {
            Ok(Self::End)
        } else {
            match value.chars().next() {
                Some('A'..='Z') => Ok(Self::Big(value.to_string())),
                Some('a'..='z') => Ok(Self::Small(value.to_string())),
                _ => Err(()),
            }
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Start => "start".hash(state),
            Self::End => "end".hash(state),
            Self::Big(name) => name.hash(state),
            Self::Small(name) => name.hash(state),
        }
    }
}

impl Node {
    #[inline]
    pub fn is_small(&self) -> bool {
        match *self {
            Node::Small(_) => true,
            _ => false,
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Start => f.write_str("start"),
            Node::End => f.write_str("end"),
            Node::Big(name) | Node::Small(name) => f.write_str(&name),
        }
    }
}

pub struct CaveGraph {
    pub nodes: Vec<Node>,
    adj: Vec<Vec<usize>>,
    start_i: usize,
    end_i: usize,
}

impl CaveGraph {
    pub fn from_edges(edges: Vec<(Node, Node)>) -> Self {
        let node_set: HashSet<&Node> =
            edges.iter().map(|(a, b)| [a, b]).flatten().collect();

        let nodes: Vec<Node> = node_set.iter().map(|&n| n.clone()).collect();
        let indexes: HashMap<&Node, usize> =
            HashMap::from_iter(nodes.iter().enumerate().map(|(i, n)| (n, i)));
        let start_i = *indexes.get(&Node::Start).unwrap();
        let end_i = *indexes.get(&Node::End).unwrap();

        let mut adj = vec![Vec::new(); nodes.len()];
        for (a, b) in &edges {
            adj[*indexes.get(a).unwrap()].push(*indexes.get(b).unwrap());
            adj[*indexes.get(b).unwrap()].push(*indexes.get(a).unwrap());
        }

        Self { nodes, adj, start_i, end_i }
    }

    pub fn find_all_paths_with(
        &self,
        skip_node: impl Copy + Fn(&mut Vec<Option<usize>>, bool, usize) -> bool,
    ) -> usize {
        let mut path = Vec::with_capacity(self.nodes.len() * 2);
        let mut visits = self
            .nodes
            .iter()
            .map(|n| if n.is_small() { Some(0_usize) } else { None })
            .collect();
        let mut has_small_double_visit = false;
        let mut count = 0;
        self.find_all_paths_impl(
            &mut path,
            &mut visits,
            &mut has_small_double_visit,
            skip_node,
            &mut count,
            self.start_i,
        );
        count
    }

    fn find_all_paths_impl<'a>(
        &'a self,
        path: &mut Vec<usize>,
        small_visits: &mut Vec<Option<usize>>,
        has_small_double_visit: &mut bool,
        skip_node: impl Copy + Fn(&mut Vec<Option<usize>>, bool, usize) -> bool,
        count: &mut usize,
        node: usize,
    ) {
        path.push(node);
        if let Some(v) = &mut small_visits[node] {
            *v += 1;
            if *v >= 2 {
                *has_small_double_visit = true;
            }
        }

        for a in &self.adj[node] {
            if *a == self.start_i
                || skip_node(small_visits, *has_small_double_visit, *a)
            {
                continue;
            }
            if *a == self.end_i {
                *count += 1;
            } else {
                self.find_all_paths_impl(
                    path,
                    small_visits,
                    has_small_double_visit,
                    skip_node,
                    count,
                    *a,
                );
            }
        }

        path.pop();
        if let Some(v) = &mut small_visits[node] {
            *v -= 1;
            if *v >= 1 {
                *has_small_double_visit = false;
            }
        }
    }
}
