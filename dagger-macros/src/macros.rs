use std::fmt::Write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
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
        let name: Ident = input.parse()?;
        let _: Colon = input.parse()?;
        let process: Ident = input.parse()?;

        let parents;
        parenthesized!(parents in input);
        let parents = parents.parse_terminated(Ident::parse, Comma)?;

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

#[proc_macro]
pub fn dagger(input: TokenStream) -> TokenStream {
    let GraphStructure { nodes, output } = parse_macro_input!(input as GraphStructure);

    let dot = {
        if cfg!(feature = "visualize") {
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
                    out_nodes.iter().for_each(|out_node| {
                        writeln!(&mut dot, "{out_node} -> crapper___").unwrap()
                    });
                }
            }
            // First } char escapes the second
            writeln!(&mut dot, "}}").unwrap();
            let dot_lit = LitStr::new(&dot, dot.span());

            quote! {
                #dot_lit
            }
        } else {
            quote! {}
        }
    };

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
    let node_name: Vec<_> = node_name
        .into_iter()
        .map(|v| LitStr::new(&v.to_string(), v.span()))
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
    let node_parent_check: Vec<_> = node_parents
        .iter()
        .map(|node_parents| {
            if node_parents.is_empty() {
                quote! { true }
            } else {
                quote! { #(#node_parents.is_ok())&&* }
            }
        })
        .collect();

    let out_tokens = match output {
        GraphOutput::Single(out_ident) => {
            let out_data = Ident::new(&format!("{out_ident}_data"), out_ident.span());
            let out_clone = Ident::new(&format!("{out_ident}_data_c"), out_ident.span());
            quote! {
                let #out_clone = &#out_data;
                #out_clone.wait()
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
                (#(#out_clone.wait()),*)
            }
        }
    };

    quote! {{
        use dagger::prelude::*;
        Graph::new(|| {
            #(let #node_data = ProcessData::new();)*
            ::std::thread::scope(|s| {
                    #(
                        {
                            let node_name = #node_name;
                            #(let #node_parent_data_clones = &#node_parent_data;)*
                            let #node_data_clone = &#node_data;
                            s.spawn(|| {
                                #(
                                    let #node_parents = #node_parent_data_clones.wait();
                                )*
                                if #node_parent_check {
                                    let (#(#node_parents),*) = (#(#node_parents.unwrap()),*);
                                    let process_result = #node_process(#(#node_parents),*).into_process_result(node_name);
                                    #node_data_clone.set(process_result);
                                } else {
                                    let mut joined_err = ProcessError::default();
                                    #(
                                        if let Err(e) = #node_parents {
                                            e.push_error(node_name, &mut joined_err);
                                        }
                                    )*
                                    #node_data_clone.set(Err(joined_err));
                                }
                            });
                        }
                    )*
                #out_tokens
            })
        },
        #dot)
    }}
    .into()
}
