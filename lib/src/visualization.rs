use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use crate::process_data::GraphError;
use layout::{
    backends::svg::SVGWriter,
    gv::{DotParser, GraphBuilder},
};

pub fn dot_to_svg(input: &str) -> String {
    let graph = DotParser::new(input).process().unwrap();
    let mut builder = GraphBuilder::new();
    builder.visit_graph(&graph);
    let mut vg = builder.get();
    let mut svg = SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);
    svg.finalize()
}

pub fn render_svg(input: &str, path: &Path) {
    let svg = dot_to_svg(input);
    let _ = fs::write(path, &svg);
}

pub fn visualize_errors(out_path: &Path, results: &[&Result<(), &GraphError>], dot: &'static str) {
    #[derive(Debug)]
    enum LineInstruction<'a> {
        Arrow(&'a str),
        Box(&'a str, usize),
        OutputBox(usize),
        Other,
    }

    let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let dot_parse: Vec<_> = dot
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
    results.iter().for_each(|res| {
        if let Err(e) = res {
            e.0.iter().for_each(|err| {
                failure_origins.insert(err.0);
            });
        }
    });
    if failure_origins.is_empty() {
        render_svg(dot, out_path);
        return;
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

    let dot_final = dot
        .lines()
        .zip(dot_parse)
        .map(|(line, instruction)| {
            let mut line = String::from(line);
            match instruction {
                LineInstruction::Arrow(parent) => {
                    if affected_nodes.contains(parent) {
                        line.push_str(r#" [color="red"]"#);
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
        .join("\n");

    render_svg(&dot_final, out_path);
}
