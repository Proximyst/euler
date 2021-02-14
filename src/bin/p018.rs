use itertools::Itertools as _;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;

static TREE: Lazy<RwLock<Node>> = Lazy::new(|| {
    RwLock::new(parse_tree(
        "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
    ))
});

fn main() {
    println!("{}", TREE.write().best_value());
}

#[derive(Clone)]
enum Node {
    Branch {
        value: u64,
        memoized: Option<u64>,
        left: Arc<RwLock<Node>>,
        right: Arc<RwLock<Node>>,
    },
    Leaf(u64),
}

impl Node {
    fn best_value(&mut self) -> u64 {
        match self {
            Self::Leaf(v) => *v,
            Self::Branch {
                value,
                memoized,
                left,
                right,
            } => {
                if let Some(v) = memoized {
                    return *v;
                }

                let left = left.write().best_value();
                let right = right.write().best_value();
                let val = *value + left.max(right);
                *memoized = Some(val);
                val
            }
        }
    }
}

fn parse_tree(s: &str) -> Node {
    let mut lines = s
        .lines()
        .map(|l| l.split(' ').flat_map(str::parse::<u64>).collect_vec())
        .collect_vec();

    let mut nodes = lines
        .pop()
        .unwrap()
        .into_iter()
        .map(|n| Arc::new(RwLock::new(Node::Leaf(n))))
        .collect_vec();

    for line in lines.into_iter().rev() {
        nodes = line
            .into_iter()
            .enumerate()
            .map(|(i, val)| {
                Arc::new(RwLock::new(Node::Branch {
                    value: val,
                    memoized: None,
                    left: Arc::clone(&nodes[i]),
                    right: Arc::clone(&nodes[i + 1]),
                }))
            })
            .collect_vec();
    }

    Arc::try_unwrap(nodes.pop().unwrap())
        .map_err(|_| panic!("cannot unwrap arc?"))
        .unwrap()
        .into_inner()
}

#[test]
fn test_parse() {
    let mut tree = parse_tree(
        "50
30 70
5 3 1",
    );
    assert_eq!(123, tree.best_value());
}
