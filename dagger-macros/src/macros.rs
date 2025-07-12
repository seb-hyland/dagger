use std::{
    fmt::Write,
    fs::{create_dir_all, write},
    path::{Path, PathBuf},
};

use layout::{
    backends::svg::SVGWriter,
    gv::{DotParser, GraphBuilder},
};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident, Token, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Colon, Comma, Paren},
};

enum GraphOutput {
    Single(Ident),
    Multi(Punctuated<Ident, Comma>),
}

struct GraphStructure {
    nodes: Vec<Node>,
    output: GraphOutput,
}

struct Node {
    name: Ident,
    process: Ident,
    parents: Punctuated<Ident, Comma>,
}

impl Parse for Node {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![:]>()?;
        let process = input.parse::<Ident>()?;

        let parents;
        parenthesized!(parents in input);
        let parents = parents.parse_terminated(Ident::parse, Token![,])?;

        Ok(Node {
            name,
            process,
            parents,
        })
    }
}

impl Parse for GraphStructure {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while input.peek(Ident) && input.peek2(Colon) {
            let node: Node = input.parse()?;
            nodes.push(node);
            let _: Comma = input.parse()?;
        }
        let output = if input.peek(Paren) {
            let parens_inner;
            parenthesized!(parens_inner in input);
            let output = parens_inner.parse_terminated(Ident::parse, Comma)?;
            GraphOutput::Multi(output)
        } else {
            let output: Ident = input.parse()?;
            GraphOutput::Single(output)
        };
        Ok(GraphStructure { nodes, output })
    }
}

fn dot_to_svg(input: &str) -> String {
    let graph = DotParser::new(input).process().unwrap();
    let mut builder = GraphBuilder::new();
    builder.visit_graph(&graph);
    let mut vg = builder.get();
    let mut svg = SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);
    svg.finalize()
}

#[proc_macro]
pub fn operation(input: TokenStream) -> TokenStream {
    let GraphStructure { nodes, output } = parse_macro_input!(input as GraphStructure);

    let dot = {
        let mut dot = String::from("digraph {");
        writeln!(&mut dot, "graph [rankdir=\"TB\"];").unwrap();
        nodes.iter().for_each(|n| {
            writeln!(
                &mut dot,
                "{} [shape=box label=\"{}: {}\"]",
                n.name, n.name, n.process
            )
            .unwrap();
            n.parents.iter().for_each(|parent| {
                writeln!(&mut dot, "{} -> {}", parent, n.name).unwrap();
            });
        });
        writeln!(&mut dot, "crapper___[label=Output]").unwrap();
        match &output {
            GraphOutput::Single(out_node) => {
                writeln!(&mut dot, "{out_node} -> crapper___").unwrap();
            }
            GraphOutput::Multi(out_nodes) => {
                out_nodes
                    .iter()
                    .for_each(|out_node| writeln!(&mut dot, "{out_node} -> crapper___").unwrap());
            }
        }
        // First } char escapes the second
        writeln!(&mut dot, "}}").unwrap();
        dot
    };

    let svg = dot_to_svg(&dot);

    if cfg!(feature = "visualize") {
        let filename = {
            let span = Span::call_site().start();
            let file_string = span.file();
            let origin_file = Path::new(&file_string);
            let file = origin_file.file_name().unwrap().to_string_lossy();
            let (line, col) = (span.line(), span.column());
            format!("{file}:{line}:{col}.svg")
        };
        let cargo_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let graph_dir = PathBuf::from(cargo_root).join("graphs");
        create_dir_all(&graph_dir).unwrap();
        write(graph_dir.join(filename), &svg).unwrap();
    }

    let (node_name, node_process, node_parents): (Vec<_>, Vec<_>, Vec<Vec<_>>) = nodes
        .into_iter()
        .map(|node| (node.name, node.process, node.parents.into_iter().collect()))
        .collect();
    let node_data: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("{name}_data"), name.span()))
        .collect();
    let node_data_clone: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("{name}_data_c"), name.span()))
        .collect();
    let node_parent_data: Vec<Vec<_>> = node_parents
        .iter()
        .map(|node| {
            node.iter()
                .map(|parent| Ident::new(&format!("{parent}_data"), parent.span()))
                .collect()
        })
        .collect();
    let node_parent_data_clones: Vec<Vec<_>> = node_parents
        .iter()
        .map(|node| {
            node.iter()
                .map(|parent| Ident::new(&format!("{parent}_data_c"), parent.span()))
                .collect()
        })
        .collect();

    let out_tokens = match output {
        GraphOutput::Single(out_ident) => {
            let out_data = Ident::new(&format!("{out_ident}_data"), out_ident.span());
            let out_clone = Ident::new(&format!("{out_ident}_data_c"), out_ident.span());
            quote! {
                let #out_clone = &#out_data;
                s.spawn(|| {
                    #out_clone.wait()
                })
                .join()
                .unwrap()
            }
        }
        GraphOutput::Multi(outputs) => {
            let out_data: Vec<_> = outputs
                .iter()
                .map(|out_ident| Ident::new(&format!("{out_ident}_data"), out_ident.span()))
                .collect();
            let out_clone: Vec<_> = outputs
                .iter()
                .map(|out_ident| Ident::new(&format!("{out_ident}_data_c"), out_ident.span()))
                .collect();
            quote! {
                #(let #out_clone = &#out_data;)*
                s.spawn(|| {
                    (#(#out_clone.wait()),*)
                })
                .join()
                .unwrap()
            }
        }
    };

    quote! {
        ::dagger::Graph::new(|| {
            #(let #node_data = ::dagger::data::ProcessData::new();)*
            ::std::thread::scope(|s| {
                #(
                    #(let #node_parent_data_clones = &#node_parent_data;)*
                    let #node_data_clone = &#node_data;
                    s.spawn(|| {
                        #(
                            let #node_parents = #node_parent_data_clones.wait();
                        )*
                        let result = #node_process(#(#node_parents),*);
                        #node_data_clone.set(result);
                    });
                )*
                #out_tokens
            })
        }, #dot, #svg)
    }
    .into()
}
