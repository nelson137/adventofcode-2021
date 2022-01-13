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
    pub fn is_start(&self) -> bool {
        match *self {
            Node::Start => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_end(&self) -> bool {
        match *self {
            Node::End => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_small(&self) -> bool {
        match self {
            Self::Small(_) => true,
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
    nodes: Vec<Node>,
    adj: HashMap<Node, Vec<Node>>,
}

impl CaveGraph {
    pub fn from_edges(edges: Vec<(Node, Node)>) -> Self {
        let node_set: HashSet<&Node> =
            edges.iter().map(|(a, b)| [a, b]).flatten().collect();

        let nodes: Vec<Node> = node_set.iter().map(|&n| n.clone()).collect();

        let mut adj =
            HashMap::from_iter(nodes.iter().map(|n| (n.clone(), Vec::new())));
        for (a, b) in &edges {
            adj.get_mut(a).unwrap().push(b.clone());
            adj.get_mut(b).unwrap().push(a.clone());
        }

        Self { nodes, adj }
    }

    #[inline]
    pub fn get_neighbors(&self, node: &Node) -> &Vec<Node> {
        self.adj.get(node).unwrap()
    }

    pub fn find_all_paths(
        &self,
        one_small_extra_visits: Option<usize>,
    ) -> usize {
        let mut path = Vec::with_capacity(self.nodes.len());
        let mut visited = HashMap::from_iter(self.nodes.iter().map(|n| (n, 0)));
        let mut count = 0;
        self.find_all_paths_impl(
            &mut path,
            &mut visited,
            one_small_extra_visits,
            &mut count,
            &Node::Start,
        );
        count
    }

    fn find_all_paths_impl<'a>(
        &'a self,
        path: &mut Vec<&'a Node>,
        small_visits: &mut HashMap<&'a Node, usize>,
        one_small_extra_visits: Option<usize>,
        count: &mut usize,
        node: &'a Node,
    ) {
        path.push(node);
        if node.is_small() {
            *small_visits.get_mut(node).unwrap() += 1;
        }

        let did_visit_extra = if let Some(extra_visits) = one_small_extra_visits
        {
            small_visits.values().find(|v| **v >= extra_visits).is_some()
        } else {
            false
        };

        if path.last().unwrap().is_end() {
            *count += 1;
        } else {
            for neighbor in self.get_neighbors(node) {
                if neighbor.is_start() {
                    continue;
                }
                if neighbor.is_small() {
                    let visits = *small_visits.get(neighbor).unwrap();
                    if let Some(extra_visits) = one_small_extra_visits {
                        if visits >= extra_visits - 1 && did_visit_extra {
                            continue;
                        }
                    } else {
                        if visits >= 1 {
                            continue;
                        }
                    }
                }
                self.find_all_paths_impl(
                    path,
                    small_visits,
                    one_small_extra_visits,
                    count,
                    neighbor,
                );
            }
        }

        path.pop();
        if node.is_small() {
            *small_visits.get_mut(node).unwrap() -= 1;
        }
    }
}
