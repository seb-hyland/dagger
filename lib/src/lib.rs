use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

#[doc(hidden)]
pub mod __private;
pub mod prelude;
pub mod process_data;
pub mod result;
#[cfg(feature = "visualize")]
mod visualization;

pub use dagger_macros::dagger;

pub struct Graph<T, F: Fn() -> T> {
    func: F,
    #[cfg(feature = "visualize")]
    dot: &'static str,
}

impl<T, F: Fn() -> T> Graph<T, F> {
    #[cfg(not(feature = "visualize"))]
    pub fn new(func: F) -> Graph<T, F> {
        Graph { func }
    }

    #[cfg(feature = "visualize")]
    pub fn new(func: F, dot: &'static str) -> Graph<T, F> {
        Graph { func, dot }
    }

    pub fn exe(&self) -> T {
        (self.func)()
    }

    #[cfg(feature = "visualize")]
    pub fn dot(&self) -> &'static str {
        self.dot
    }

    pub fn visualize_errors<'a, I>(&'a self, results: I) -> String
    where
        I: IntoIterator<Item = &'a Result<(), process_data::GraphError>>,
    {
        #[derive(Debug)]
        enum LineInstruction<'a> {
            Arrow(&'a str),
            Box(&'a str, usize),
            OutputBox(usize),
            Other,
        }

        let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();
        let dot_parse: Vec<_> = self
            .dot
            .lines()
            .enumerate()
            .map(|(idx, l)| {
                if idx == 0 {
                    return LineInstruction::Other;
                }
                let mut parts = l.split(" -> ");
                if let (Some(parent), Some(child)) = (parts.next(), parts.next()) {
                    let entry = node_map.entry(parent).or_default();
                    entry.push(child);
                    LineInstruction::Arrow(parent)
                } else if let Some(idx) = l.rfind(']') {
                    if l.contains("___process_output") {
                        LineInstruction::OutputBox(idx)
                    } else {
                        let mut parts = l.split_whitespace();
                        if let Some(parent) = parts.next() {
                            LineInstruction::Box(parent, idx)
                        } else {
                            LineInstruction::Other
                        }
                    }
                } else {
                    LineInstruction::Other
                }
            })
            .collect();

        let mut failure_origins = HashSet::new();
        results.into_iter().for_each(|res| {
            if let Err(e) = res {
                e.0.iter().for_each(|err| {
                    failure_origins.insert(err.0);
                });
            }
        });
        if failure_origins.is_empty() {
            return self.dot.to_string();
        }

        let mut affected_nodes = HashSet::new();
        fn add_children<'a>(
            node: &str,
            node_map: &HashMap<&str, Vec<&'a str>>,
            affected_nodes: &mut HashSet<&'a str>,
        ) {
            if let Some(v) = node_map.get(node) {
                v.iter().for_each(|node| {
                    affected_nodes.insert(node);
                    add_children(node, node_map, affected_nodes)
                });
            }
        }
        failure_origins.into_iter().for_each(|origin_node| {
            affected_nodes.insert(origin_node);
            add_children(origin_node, &node_map, &mut affected_nodes)
        });

        self.dot
            .lines()
            .zip(dot_parse)
            .map(|(line, instruction)| {
                let mut line = String::from(line);
                match instruction {
                    LineInstruction::Arrow(parent) => {
                        if affected_nodes.contains(parent) {
                            line.push_str(r#"[color="red"]"#);
                        }
                    }
                    LineInstruction::Box(parent, idx) => {
                        if affected_nodes.contains(parent) {
                            let (start, end) = line.split_at(idx);
                            line = format!(r#"{start} color="red"{end}"#);
                        }
                    }
                    LineInstruction::OutputBox(idx) => {
                        let (start, end) = line.split_at(idx);
                        line = format!(r#"{start} color="orange"{end}"#);
                    }
                    LineInstruction::Other => {}
                }
                line
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub trait CloneInner<T> {
    fn clone_inner(self) -> T;
}

impl<T: Clone> CloneInner<T> for Arc<T> {
    fn clone_inner(self) -> T {
        self.deref().clone()
    }
}
