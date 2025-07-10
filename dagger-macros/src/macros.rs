use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, Token, bracketed,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

struct GraphStructure {
    graph: Vec<Node>,
    output: Ident,
}

struct Node {
    name: Ident,
    process: Ident,
    parents: Punctuated<Ident, Comma>,
    args: Punctuated<Expr, Comma>,
}

impl Parse for Node {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![:]>()?;
        let process = input.parse::<Ident>()?;

        let parents;
        bracketed!(parents in input);
        let parents = parents.parse_terminated(Ident::parse, Token![,])?;

        let args;
        bracketed!(args in input);
        let args = args.parse_terminated(Expr::parse, Token![,])?;

        Ok(Node {
            name,
            process,
            parents,
            args,
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
        let output: Ident = input.parse()?;
        Ok(GraphStructure { graph, output })
    }
}

#[proc_macro]
pub fn graph(input: TokenStream) -> TokenStream {
    let graph = parse_macro_input!(input as GraphStructure);
    quote! {}.into()
}
