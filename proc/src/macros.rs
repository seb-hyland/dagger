use std::{collections::HashSet, fmt::Write as _};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, LitStr, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
    spanned::Spanned as _,
    token::{Colon, Comma, Paren, Semi},
    visit::Visit,
};

struct GraphStructure {
    nodes: Vec<Node>,
    output: Vec<Ident>,
}

struct Node {
    name: Ident,
    process: Expr,
    parents: Vec<Ident>,
}

impl Parse for Node {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Colon = input.parse()?;
        let _: Colon = input.parse()?;
        let process: Expr = input.parse()?;

        Ok(Node {
            name,
            process,
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
            let _: Semi = input.parse()?;
        }
        let output = if input.peek(Paren) {
            let parens_inner;
            parenthesized!(parens_inner in input);
            let output = parens_inner.parse_terminated(Ident::parse, Comma)?;
            output.into_iter().collect()
        } else {
            let output: Ident = input.parse()?;
            vec![output]
        };
        Ok(GraphStructure { nodes, output })
    }
}

#[proc_macro]
pub fn dagger(input: TokenStream) -> TokenStream {
    let GraphStructure { mut nodes, output } = parse_macro_input!(input as GraphStructure);

    let mut node_idents = HashSet::with_capacity(nodes.len());
    if let Err(e) = nodes.iter().try_for_each(|node| -> Result<(), syn::Error> {
        if !node_idents.insert(node.name.clone()) {
            Err(syn::Error::new(
                node.name.span(),
                format!("Duplicate definition of node {}", node.name),
            ))
        } else {
            Ok(())
        }
    }) {
        return e.to_compile_error().into();
    }

    struct ParentCollector<'a> {
        node: &'a mut Node,
        node_idents: &'a HashSet<Ident>,
    }

    impl<'a> Visit<'a> for ParentCollector<'a> {
        fn visit_path(&mut self, path: &'a syn::Path) {
            path.segments.iter().for_each(|seg| {
                if self.node_idents.contains(&seg.ident) && self.node.name != seg.ident {
                    self.node.parents.push(seg.ident.clone())
                }
            });
            syn::visit::visit_path(self, path);
        }
    }

    nodes.iter_mut().for_each(|node| {
        let node_idents = &node_idents;
        let process = node.process.clone();
        let mut visitor = ParentCollector { node, node_idents };
        visitor.visit_expr(&process);
    });

    let (dot, visualize_tokens) = {
        if cfg!(feature = "visualize") {
            let mut dot = String::from("digraph {\n");
            writeln!(&mut dot, r#"graph [rankdir="TB"]"#).unwrap();
            nodes.iter().for_each(|n| {
                writeln!(&mut dot, r#"{} [shape=box label="{}"]"#, n.name, n.name).unwrap();
                n.parents.iter().for_each(|parent| {
                    writeln!(&mut dot, "{} -> {}", parent, n.name).unwrap();
                });
            });
            writeln!(&mut dot, r#""___process_output" [label=Output]"#).unwrap();
            output.iter().for_each(|out_node| {
                writeln!(&mut dot, r#"{out_node} -> "___process_output""#).unwrap()
            });
            // First } char escapes the second
            writeln!(&mut dot, "}}").unwrap();
            let dot_lit = LitStr::new(&dot, dot.span());

            (
                quote! { #dot_lit },
                quote! {
                    if let Some(path) = write_path {
                        visualize_errors(path, &[#(&#output.as_ref().map(|_| ())),*], dot);
                    }
                },
            )
        } else {
            (quote! {}, quote! {})
        }
    };

    let (node_name, node_process, node_parents): (Vec<_>, Vec<_>, Vec<Vec<_>>) = nodes
        .into_iter()
        .map(|node| (node.name, node.process, node.parents.into_iter().collect()))
        .collect();
    let node_data: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("__private_{name}"), name.span()))
        .collect();
    let node_data_tx: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("__private_{name}_tx"), name.span()))
        .collect();
    let node_data_rx: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("__private_{name}_rx"), name.span()))
        .collect();
    let node_name: Vec<_> = node_name
        .into_iter()
        .map(|v| LitStr::new(&v.to_string(), v.span()))
        .collect();
    let node_parent_data: Vec<Vec<_>> = node_parents
        .iter()
        .map(|node| {
            node.iter()
                .map(|parent| Ident::new(&format!("__private_{parent}_rx"), parent.span()))
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

    let out_idents: Vec<_> = output
        .iter()
        .map(|out_ident| Ident::new(&format!("__private_{out_ident}_rx"), out_ident.span()))
        .collect();

    quote! {{
        use dagger::__private::*;
        let execution_function = |write_path: Option<&::std::path::Path>, dot: &'static str| {
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
                                    let process_value: NodeResult<_> = #node_process;
                                    let process_result = process_value.into_graph_result(node_name);
                                    #node_data_tx.set(process_result);
                                } else {
                                    let mut joined_err = GraphError::default();
                                    #(
                                        if let Err(e) = #node_parents {
                                            e.push_error(&mut joined_err);
                                        }
                                    )*
                                    #node_data_tx.set(Err(joined_err));
                                }
                            });
                        }
                    )*
                    let (#(#output),*) = (#(#out_idents.wait()),*);
                    #visualize_tokens
                    (#(#output),*)
            })
        };
        Graph::new(execution_function, #dot)
    }}
    .into()
}
