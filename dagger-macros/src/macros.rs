use std::{collections::HashSet, fmt::Write};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, LitStr, parenthesized,
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
    args: Punctuated<Expr, Comma>,
    parents: Vec<Ident>,
}

impl Parse for Node {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Colon = input.parse()?;
        let _: Colon = input.parse()?;
        let process: Ident = input.parse()?;

        let args;
        parenthesized!(args in input);
        let args = args.parse_terminated(Expr::parse, Comma)?;

        Ok(Node {
            name,
            process,
            args,
            parents: Vec::new(),
        })
    }
}

impl Parse for GraphStructure {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while input.peek(Ident) && input.peek2(Colon) && input.peek3(Colon) {
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
    let GraphStructure { mut nodes, output } = parse_macro_input!(input as GraphStructure);

    let mut node_idents = HashSet::with_capacity(nodes.len());
    for node in &nodes {
        if !node_idents.insert(node.name.clone()) {
            return syn::Error::new(
                node.name.span(),
                format!("Duplicate definition of node {}", node.name),
            )
            .into_compile_error()
            .into();
        }
    }
    fn identify_parents(arg: &Expr, node: &mut Node, node_idents: &HashSet<Ident>) {
        match arg {
            Expr::Path(exp) => exp.path.segments.iter().for_each(|seg| {
                if node_idents.contains(&seg.ident) {
                    node.parents.push(seg.ident.clone())
                }
            }),
            Expr::Unary(exp) => {
                identify_parents(&exp.expr, node, node_idents);
            }
            Expr::Binary(exp) => {
                identify_parents(&exp.left, node, node_idents);
                identify_parents(&exp.right, node, node_idents);
            }
            Expr::Call(exp) => exp
                .args
                .iter()
                .for_each(|exp| identify_parents(exp, node, node_idents)),
            Expr::MethodCall(exp) => {
                identify_parents(&exp.receiver, node, node_idents);
                exp.args
                    .iter()
                    .for_each(|arg| identify_parents(arg, node, node_idents));
            }
            Expr::Cast(exp) => {
                identify_parents(&exp.expr, node, node_idents);
            }
            _ => {}
        }
    }
    nodes.iter_mut().for_each(|node| {
        let args = node.args.clone();
        args.iter()
            .for_each(|arg| identify_parents(arg, node, &node_idents))
    });

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

    let (node_name, node_process, node_parents, node_args): (
        Vec<_>,
        Vec<_>,
        Vec<Vec<_>>,
        Vec<Vec<_>>,
    ) = nodes
        .into_iter()
        .map(|node| {
            (
                node.name,
                node.process,
                node.parents.into_iter().collect(),
                node.args.into_iter().collect(),
            )
        })
        .collect();
    let node_data: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("_{name}_data"), name.span()))
        .collect();
    let node_data_tx: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("_{name}_data_tx"), name.span()))
        .collect();
    let node_data_rx: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("_{name}_data_rx"), name.span()))
        .collect();
    let node_name: Vec<_> = node_name
        .into_iter()
        .map(|v| LitStr::new(&v.to_string(), v.span()))
        .collect();
    let node_parent_data: Vec<Vec<_>> = node_parents
        .iter()
        .map(|node| {
            node.iter()
                .map(|parent| Ident::new(&format!("_{parent}_data_rx"), parent.span()))
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
            let out_data = Ident::new(&format!("_{out_ident}_data_rx"), out_ident.span());
            quote! {
                #out_data.wait()
            }
        }
        GraphOutput::Multi(outputs) => {
            let out_data: Vec<_> = outputs
                .iter()
                .map(|out_ident| Ident::new(&format!("_{out_ident}_data_rx"), out_ident.span()))
                .collect();
            quote! {
                (#(#out_data.wait()),*)
            }
        }
    };

    quote! {{
        use dagger::prelude::*;
        Graph::new(|| {
            #(let #node_data = ProcessData::default();)*
            #(let (#node_data_tx, #node_data_rx) = #node_data.channel();)*
            ::std::thread::scope(|s| {
                    #(
                        {
                            let node_name = #node_name;
                            s.spawn(|| {
                                #(
                                    let #node_parents = #node_parent_data.wait();
                                )*
                                if #node_parent_check {
                                    let (#(#node_parents),*) = (#(#node_parents.unwrap()),*);
                                    let process_result = #node_process(#(#node_args),*).into_process_result(node_name);
                                    #node_data_tx.set(process_result);
                                } else {
                                    let mut joined_err = ProcessError::default();
                                    #(
                                        if let Err(e) = #node_parents {
                                            e.push_error(node_name, &mut joined_err);
                                        }
                                    )*
                                    #node_data_tx.set(Err(joined_err));
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
