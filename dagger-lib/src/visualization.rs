use layout::{
    backends::svg::SVGWriter,
    gv::{DotParser, GraphBuilder},
};

fn dot_to_svg(input: &str) -> String {
    let graph = DotParser::new(input).process().unwrap();
    let mut builder = GraphBuilder::new();
    builder.visit_graph(&graph);
    let mut vg = builder.get();
    let mut svg = SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);
    svg.finalize()
}
