

// use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
// use petgraph::dot::{Dot, Config};
// use petgraph::{Undirected};
// use petgraph::data::{FromElements, Element};
// use petgraph::EdgeType;

// use petgraph::visit::EdgeRef;

// fn split_connected_components<N, E: Copy>(graph: &Graph<N, E, Undirected>) {
// 	// dfs traverse, add nodes
// 	// copy edges to new graph
// 	//  iterate over all edges
// 	//  add edge if, source & target are in the graph
// }

// fn dfs_traversal<N, E: Copy>(graph: &Graph<N, E, Undirected>, directed_tree: &mut Graph<N, E>, current_node: NodeIndex, parent_edge: Option<EdgeIndex>, mst_edges: &[EdgeIndex])
// {
//     for edge in graph.edges(current_node) {
//         let edge_index = edge.id();
//         let source = edge.source();
//         let target = edge.target();

//         // Skip the parent edge in the DFS traversal(don't want to step backwards)
//         if Some(edge_index) == parent_edge {
//             continue;
//         }

//         // Assign a direction to the edge based on MST membership
//         if mst_edges.contains(&edge_index) {
//             directed_tree.add_edge(source, target, graph[edge_index]);
//         } else {
//             directed_tree.add_edge(target, source, graph[edge_index]);
//         }

//         // Recursively traverse the child node connected to the edge
//         let child_node = if current_node == source { target } else { source };
//         dfs_traversal(graph, directed_tree, child_node, Some(edge_index), mst_edges);
//     }
// }