use std::{
    collections::{HashMap, HashSet},
    fmt::Write as _,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, LitStr, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
    spanned::Spanned as _,
    token::{Colon, Comma, Paren, Return, Semi},
    visit::Visit,
};

struct GraphStructure {
    nodes: Vec<Node>,
    output: Option<Vec<Ident>>,
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
        let return_ident = input.peek(Return);
        if return_ident {
            let _: Return = input.parse()?;
        }
        let output = if input.peek(Paren) {
            let parens_inner;
            parenthesized!(parens_inner in input);
            let output = parens_inner.parse_terminated(Ident::parse, Comma)?;
            Some(output.into_iter().collect())
        } else if input.peek(Ident) {
            let output: Ident = input.parse()?;
            Some(vec![output])
        } else {
            None
        };
        if return_ident && input.peek(Semi) {
            let _: Semi = input.parse()?;
        }
        Ok(GraphStructure { nodes, output })
    }
}

#[proc_macro]
pub fn dagger(input: TokenStream) -> TokenStream {
    let GraphStructure { mut nodes, output } = parse_macro_input!(input as GraphStructure);

    // Collect node idents and check duplicates
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

    // Visitor to identify node parents
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

    for node in nodes.iter_mut() {
        let node_idents = &node_idents;
        let process = node.process.clone();
        let mut visitor = ParentCollector { node, node_idents };
        visitor.visit_expr(&process);
    }

    let mut children_map = HashMap::with_capacity(nodes.len());
    for node in node_idents {
        children_map.insert(node, Vec::new());
    }
    for (node_idx, node) in nodes.iter().enumerate() {
        for parent in node.parents.iter() {
            children_map
                .get_mut(parent)
                .expect("Parent should exist in ident map")
                .push(node_idx);
        }
    }
    let mut node_children = Vec::with_capacity(nodes.len());
    for node in nodes.iter() {
        let children = children_map
            .get(&node.name)
            .expect("Node should exist in children map");
        node_children.push(children);
    }

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
            if let Some(o) = output.as_ref() {
                writeln!(&mut dot, r#""___process_output" [label=Output]"#).unwrap();
                o.iter().for_each(|out_node| {
                    writeln!(&mut dot, r#"{out_node} -> "___process_output""#).unwrap()
                });
            }
            // First } char escapes the second
            writeln!(&mut dot, "}}").unwrap();
            let dot_lit = LitStr::new(&dot, dot.span());

            let empty_vec = Vec::new();
            let visualize_output = output.as_ref().unwrap_or(&empty_vec);
            (
                quote! { #dot_lit },
                quote! {
                    if let Some(path) = write_path {
                        visualize_errors(path, &[#(&#visualize_output.as_ref().map(|_| ())),*], dot);
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
    let node_data_fn: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("__private_{name}_fn"), name.span()))
        .collect();
    let node_task: Vec<_> = node_name
        .iter()
        .map(|name| Ident::new(&format!("__private_{name}_task"), name.span()))
        .collect();
    let node_parent_len: Vec<_> = node_parents.iter().map(|vec| vec.len() as u32).collect();
    let node_name: Vec<_> = node_name
        .into_iter()
        .map(|v| LitStr::new(&v.to_string(), v.span()))
        .collect();
    let node_parent_data: Vec<Vec<_>> = node_parents
        .iter()
        .map(|node| {
            node.iter()
                .map(|parent| Ident::new(&format!("__private_{parent}"), parent.span()))
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

    let output = output.unwrap_or(Vec::new());
    let out_idents: Vec<_> = output
        .iter()
        .map(|out_ident| Ident::new(&format!("__private_{out_ident}"), out_ident.span()))
        .collect();

    quote! {
        {
            use dagger::__private::*;
            #[allow(path_statements)]
            let execution_function = |write_path: Option<&::std::path::Path>, dot: &'static str| {
                #(
                    let #node_data = ProcessData::default();
                )*
                #(
                    let #node_data_fn = || {
                        #(
                            let #node_parents = unsafe { #node_parent_data.get() };
                        )*
                        if #node_parent_check {
                            let (#(#node_parents),*) = (#(#node_parents.unwrap()),*);
                            let process_value: NodeResult<_> = #node_process;
                            let process_result = process_value.into_graph_result(#node_name);
                            #node_data.set(process_result);
                        } else {
                            let mut joined_err = GraphError::default();
                            #(
                                if let Err(e) = #node_parents {
                                    e.push_error(&mut joined_err);
                                }
                            )*
                            #node_data.set(Err(joined_err));
                        }
                    };
                    let #node_task = Task::new(#node_parent_len, &[#(#node_children),*], &#node_data_fn);
                )*;
                Scheduler::execute([#(#node_task),*]);
                let (#(#output),*) = (#(unsafe { #out_idents.get_owned() }),*);
                #visualize_tokens
                (#(#output),*)
            };
            Graph::new(execution_function, #dot)
    }}.into()
}
