use hashbrown::HashMap;
use num::Bounded;
use num::Num;
use num::Saturating;
use num::Zero;
use petgraph::dot::Dot;
use petgraph::graph::IndexType;
use petgraph::graph::NodeIndex;
use petgraph::visit::GraphProp;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeIndexable;
use petgraph::EdgeType;
use petgraph::Graph;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn write_graph<G>(graph: G, filename: &str)
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: std::fmt::Display,
    G::NodeWeight: std::fmt::Display,
{
    // Convert the graph into DOT format
    // let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot = Dot::with_config(graph, &[]);
    let dot_string = format!("{}", dot);

    // Write the DOT string to a temporary file
    let dot_filename = "temp.dot";
    let mut file = File::create(dot_filename).expect("Unable to create file");
    file.write_all(dot_string.as_bytes())
        .expect("Unable to write data");

    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    // Use the `dot` command-line tool (part of the Graphviz software package) to convert the DOT file to SVG
    let output = Command::new("dot")
        .arg(format!("-T{}", ext).as_str())
        .arg(dot_filename)
        .arg("-o")
        .arg(filename)
        .output()
        .expect("Failed to execute command");

    // Check the output of the command
    if !output.status.success() {
        eprintln!(
            "dot command failed with output:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Delete the temporary DOT file
    std::fs::remove_file(dot_filename).expect("Failed to remove temporary file");
}

pub fn floyd_warshall<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
) -> HashMap<(NodeIndex<Ix>, NodeIndex<Ix>), E>
where
    E: Num + Bounded + Zero + Saturating + PartialOrd + Copy,
    Ty: EdgeType,
    Ix: IndexType,
{
    let mut distances = HashMap::new();
    let nodes: Vec<_> = graph.node_identifiers().collect();

    // Initialize distances
    for node in &nodes {
        for target in &nodes {
            if node == target {
                distances.insert((*node, *target), E::zero());
            } else if let Some(edge) = graph.find_edge(*node, *target) {
                distances.insert((*node, *target), graph[edge]);
            } else {
                distances.insert((*node, *target), E::max_value());
            }
        }
    }

    // Floyd-Warshall algorithm
    for k in &nodes {
        for i in &nodes {
            for j in &nodes {
                let ikj = distances[&(*i, *k)].saturating_add(distances[&(*k, *j)]);
                if ikj < distances[&(*i, *j)] {
                    distances.insert((*i, *j), ikj);
                }
            }
        }
    }

    distances
}
