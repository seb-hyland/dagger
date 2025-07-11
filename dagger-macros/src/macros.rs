use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, Paren},
};

enum GraphOutput {
    Single(Ident),
    Multi(Punctuated<Ident, Comma>),
}

struct GraphStructure {
    graph: Vec<Node>,
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
        let mut graph = Vec::new();
        while input.peek(Ident) && input.peek2(Token![:]) {
            let node: Node = input.parse()?;
            graph.push(node);
            input.parse::<Token![,]>()?;
        }
        let output = if input.peek(Paren) {
            let parens_inner;
            parenthesized!(parens_inner in input);
            let output = parens_inner.parse_terminated(Ident::parse, Comma)?;
            GraphOutput::Multi(output)
        } else {
            let output = input.parse()?;
            GraphOutput::Single(output)
        };
        Ok(GraphStructure { graph, output })
    }
}

#[proc_macro]
pub fn graph(input: TokenStream) -> TokenStream {
    let GraphStructure { graph, output } = parse_macro_input!(input as GraphStructure);

    let (node_name, node_process, node_parents): (Vec<_>, Vec<_>, Vec<Vec<_>>) = graph
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
                let #out_clone = #out_data.clone();
                s.spawn(move || {
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
                #(let #out_clone = #out_data.clone();)*
                s.spawn(move || {
                    (#(#out_clone.wait()),*)
                })
                .join()
                .unwrap()
            }
        }
    };

    quote! {
        ::dagger::Graph::new(|| {
            #(let #node_data = ::dagger::ProcessData::new();)*
            ::std::thread::scope(move |s| {
                #(
                    #(let #node_parent_data_clones = #node_parent_data.clone();)*
                    let #node_data_clone = #node_data.clone();
                    s.spawn(move || {
                        #(
                            let #node_parents = #node_parent_data_clones.wait();
                        )*
                        let result = #node_process(#(#node_parents),*);
                        #node_data_clone.set(result);
                    });
                )*
                #out_tokens
            })
        })
    }
    .into()
}
