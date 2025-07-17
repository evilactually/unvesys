/*
 Wire list ordered traverse

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

use std::collections::HashMap;
use crate::wirelist::*;
use crate::bfs::*;

use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::{Undirected};
use petgraph::data::{FromElements, Element};
use petgraph::EdgeType;

use petgraph::visit::EdgeRef;

//use petgraph::algo::{min_spanning_tree, MinSpanningTree};

use petgraph::algo::{min_spanning_tree};

use petgraph::algo::min_spanning_tree::MinSpanningTree;


fn find_node_or_add<T, S, U>(graph: &mut Graph<S, T, U>, node_weight:S) -> NodeIndex where U: EdgeType, S: std::cmp::PartialEq<S> + Clone {
    let found_node = graph
    .node_indices()
    .find(|&node_index| graph[node_index] == node_weight.clone());

    match found_node {
        Some(node) => {
            node
        }
        None => {
            graph.add_node(node_weight.clone())
        }
    }
}

/// Recursive helper function used by find_weakly_connected_components to find connected nodes in a component. 
/// Variables graph, visited and component are shared along recursive calls. Variable graph is constant, while visited and component are mutable
/// and updated during each call. 
fn depth_first_search_for_node_connections<T, S, U>(node: &NodeIndex, graph: &Graph<S, T, U>, visited: &mut HashMap<NodeIndex, bool>, component: &mut Vec<NodeIndex>) where U: EdgeType {
    visited.insert(*node, true); // mark current node as visited
    component.push(*node);       // add current node to current component
    for neighbor in graph.neighbors(*node) {   // iterate current node neighbors
        if !visited.get(&neighbor).unwrap() {  // avoid previously visited nodes
            depth_first_search_for_node_connections(&neighbor, graph, visited, component); // call recursively on unvisited neighbors
        }
    }
} 

/// Returns list of components, where each component is represented as a list of connected nodes in original graph
pub fn find_weakly_connected_components<N, E: Copy>(graph: &Graph<N, E, Undirected>) -> std::vec::Vec<std::vec::Vec<NodeIndex>> {
    let mut visited: HashMap<NodeIndex, bool> = HashMap::new();
    let mut components: Vec<Vec<NodeIndex>> = Vec::new();
    // for node in graph:
    for node in graph.node_indices() {
        visited.insert(node, false);
    }

    for node in graph.node_indices() {
        if !visited.get(&node).unwrap_or(&false) {
            let mut component: Vec<NodeIndex> = Vec::new();
            depth_first_search_for_node_connections(&node, &graph, &mut visited, &mut component);
            components.push(component);
        }
    }

    return components;
}

/// Build Graphs from lists of nodes produced by find_weakly_connected_components
pub fn build_graphs_from_components<N: Clone, E: Copy>(graph: &Graph<N, E, Undirected>, components: std::vec::Vec<std::vec::Vec<NodeIndex>>) -> std::vec::Vec<Graph<N, E, Undirected>> {
    let mut component_graphs: Vec<Graph<N, E, Undirected>> = Vec::new();

    // iterator over each component
    for component in components.iter() {
        let mut component_graph: Graph<N, E, Undirected> = Graph::new_undirected();
        let mut nodeid_map: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        // copy over coponent nodes from source graph
        for node in component.iter() {
            let newid = component_graph.add_node(graph[*node].clone());
            nodeid_map.insert(*node, newid);
        }
        // copy edges if both nodes of an edge are part of the component
        for edge in graph.edge_references() {
            if component.contains(&edge.source()) && 
               component.contains(&edge.target()) {
                component_graph.add_edge(
                    *nodeid_map.get(&edge.source()).unwrap(), // lookup new ids from old ids
                    *nodeid_map.get(&edge.target()).unwrap(),
                    *edge.weight()                            // copy edge weight
                    );
            }
        }

        component_graphs.push(component_graph); // push graph to results
    }
    return component_graphs;
}

pub fn build_graph_from_wirelist(wirelist: &WireList) -> Graph<Box<str>, (), Undirected> {
    // Graph
    let mut graph: Graph<Box<str>, (), Undirected> = Graph::new_undirected();

    for wire in &wirelist.wires {
        // Build a graph of devices and connectors
        // Match if both connections exist
        match (&wire.left, &wire.right) {
            (Some(ref left), Some(ref right)) => {
                let left_node = find_node_or_add(&mut graph, left.device.clone().into());
                let right_node = find_node_or_add(&mut graph, right.device.clone().into());
                // Check if edge exists
                match graph.find_edge(left_node, right_node) {
                    Some(_) => {}
                    None => {
                        // Add edges only once
                        graph.add_edge(left_node, right_node, ());
                    }
                }
            }
            _ => {

            }
        }
    }

    return graph;
 
}

// // Depth-First Search (DFS) traversal
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

// fn build_directed_graph(graph: &Graph<NodeIndex, (), Undirected>, root_node: NodeIndex) -> Graph<NodeIndex, ()> {
//     fn dfs(node: NodeIndex, graph: &Graph<NodeIndex, (), Undirected>, visited: &mut HashMap<NodeIndex, bool>, directed_tree: &mut Graph<NodeIndex, ()>) {
//         visited.insert(node, true);                // mark current node as visited
//         for neighbor in graph.neighbors(node) {    // iterate current node neighbors
//             if !visited.get(&neighbor).unwrap() {  // avoid previously visited nodes
//                 //let node_a_idx = node;
//                 //let node_b_idx = directed_tree.add_node(neighbor);  // add current node to current component
//                 //println!("{:?} {:?}", node_a_idx, node_b_idx);
//                 directed_tree.add_edge(node, neighbor, ());
//                 dfs(neighbor, graph, visited, directed_tree);       // call recursively on unvisited neighbors
//             }
//         }
//     }

//     let mut visited: HashMap<NodeIndex, bool> = HashMap::new();
//     let mut directed_graph = Graph::new();
    
//     // for node in graph:
//     for node in graph.node_indices() {
//         visited.insert(node, false);
//         directed_graph.add_node(node);
//     }
//     let root_node_idx = directed_graph.add_node(root_node); // add first node, later nodes will be added by dfs function
//     dfs(root_node_idx, graph, &mut visited, &mut directed_graph);

//     return directed_graph;
// }

/// Find node with maximum neighbor nodes
fn find_max_neighbor_node<E,N>(graph:&Graph<N,E,Undirected>) -> Option<NodeIndex> {
    // Pick the root of MST
    let mut max_neighbors = -1;
    let mut max_node = graph.node_indices().next();
    for node in graph.node_indices() {
        let neighbors = graph.neighbors(node).count() as i32;
        if neighbors >= max_neighbors {
            max_neighbors = neighbors;
            max_node = Some(node);
        }
    }
    max_node
}

/// Builds a minimum spanning tree for a graph(tree that visits each node of the graph).
/// Each node is a reference to original graph.
/// # Arguments
///
/// * `graph` - Original undirected graph.
/// * `root_node` - Root node of the MST.
///
/// # Returns
///
/// Unidirected graph of MST
fn build_graph_minimum_spanning_tree<N, E>(graph: &Graph<N, E, Undirected>) -> Graph<NodeIndex, (), Undirected> 
where
    N: Clone + std::fmt::Debug,
    E: PartialOrd + Clone, // In Kruskal's MST algorithm edge weight influences which edges become part of the tree
{
    let mut mst_edges:Vec<EdgeIndex> = Vec::new();
    let mut mst_unidirected_graph: Graph<NodeIndex, (), Undirected> = Graph::new_undirected();
    {
        let mut mst = min_spanning_tree(&graph); // Returns an iterator that traverses graph in MST order
        let mut index = 0;
        for element in mst {        // This loop builds an actual MST from iterator
            match element {
                Element::Node{weight} => {
                    mst_unidirected_graph.add_node(NodeIndex::new(index));
                }
                Element::Edge{source, target, ..} => {
                    let out_source = find_node_or_add(&mut mst_unidirected_graph, NodeIndex::new(source));
                    let out_target = find_node_or_add(&mut mst_unidirected_graph, NodeIndex::new(target));
                    let edge = mst_unidirected_graph.add_edge(out_source, out_target, ());
                    mst_edges.push(edge);
                }
            }
            index = index + 1;
        }
    }

    return mst_unidirected_graph;

}


// fn unidirected_to_directed_in_dfs_order<N, E>(unidirected_graph: Graph<NodeIndex, E, Undirected>, root_node: NodeIndex) -> Graph<NodeIndex, E> {
//     let mut mst_directed_graph: Graph<NodeIndex, E> = Graph::new();
//     // Add nodes to the directed tree
//     for node in mst_unidirected_graph.node_indices() {
//         mst_directed_graph.add_node(node);
//     }
//     // Build graph in mst_directed_graph starting from root_node
//     dfs_traversal(&mst_unidirected_graph, &mut mst_directed_graph, root_node, None, mst_edges.as_slice());

//     return mst_directed_graph;
// }

//     let mut mst_directed_graph: Graph<NodeIndex, ()> = Graph::new();
//     // Add nodes to the directed tree
//     for node in mst_unidirected_graph.node_indices() {
//         mst_directed_graph.add_node(node);
//     }
//     // Build graph in mst_directed_graph starting from root_node
//     dfs_traversal(&mst_unidirected_graph, &mut mst_directed_graph, root_node, None, mst_edges.as_slice());

//     return mst_directed_graph;
// }



// produce groups
pub fn traverse(wirelist: &WireList) -> Vec<Vec<WireEntry>> {

    let mut wireGroups: Vec<Vec<WireEntry>> = Vec::new();
    wireGroups.push(Vec::new()); // Create initial wire group

    // Create a copy of wire list so we can take out wires as we go
    let mut wirelist_copy: WireList = wirelist.clone();

    // Graph
    let mut graph: Graph<Box<str>, (), Undirected> = build_graph_from_wirelist(wirelist);

    let components = find_weakly_connected_components(&graph);
    let component_graphs = build_graphs_from_components(&graph,components);

    for graph in component_graphs {
        // Build a Minimum Spaning Tree from connectivity graph. Each node is a reference to original graph.

        let mst_unidirected_graph: Graph<NodeIndex, (), Undirected> = build_graph_minimum_spanning_tree(&graph);
        let root_node = find_max_neighbor_node(&graph);

        // Perform BST traversal of mst_directed_graph
        let mut bfs = Bfs::new(&mst_unidirected_graph, root_node.unwrap());
        let current_root = root_node;

        // iterator give next item and tells if level ended(for visual separators)
        while let (Some(node), level_end) = bfs.next(&mst_unidirected_graph) { 
            let current_group = wireGroups.last_mut().unwrap();
            for neighbor in graph.neighbors(node) { // this will intentionally pick up non-mst edges of the node too
                let device_wire_entries = wirelist.get_wires_between_devices(&graph[node], &graph[neighbor]);
                for wire_entry in device_wire_entries {
                    let extracted_entries = wirelist_copy.wires.take(&wire_entry);
                    for entry in extracted_entries.iter() {
                        //println!("{:?}: ", entry.name);
                        let mut entry_reconciled = entry.clone();
                        // Reconcile wire ends to be on the same side
                        if entry.left.as_ref().unwrap().device != graph[node] {
                            entry_reconciled.swap();
                        }
                        current_group.push(entry_reconciled.clone());
                    }
                }
            }

            if level_end {
                // Create new group at the end of minimum spanning tree level during breadth-first traversal
                wireGroups.push(Vec::new());
            }
        }

        // for group in &wireGroups {
        //     println!("{}", "***************");
        //     for wire in group.wires.iter() {
        //         println!("{}: {} -> {}", wire.name, wire.left.as_ref().unwrap().device, wire.right.as_ref().unwrap().device);
        //     }
        // }



        {
        let dot = Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &move|_, edge| {
            //let is_mst_edge = mst_directed_graph.find_edge(edge.source(), edge.target()).is_some();
            if  true {
                format!("color=\"{}\"", "red")
            } else {
                "".to_string()
            }
        },
        &|_, (id,name)| {
            format!("label=\"{}\"", name)
        });

        // Print the DOT representation
        println!("{:?}", dot);
        }


        

        // let dot2 = Dot::with_attr_getters(&mst_directed_graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &|_, edge| {
        //     "".to_string()
        // },
        // &|_, (id,name)| {
        //     format!("label=\"{:?}\"", name)
        // });

        // println!("{:?}", dot2);
    }

    // println!("Remaining wires");
    // for wire in &wirelist_copy.wires {
    //     println!("{:?}", wire.name);
    // }

    // All wires must be sorted into groups by now!
    //assert_eq!(wirelist_copy.wires.len(), 0);

    // Dump remaining wires into last group
    if (wirelist_copy.wires.len() > 0) {
        for wire in wirelist_copy.wires {
            let mut struggler_group = Vec::new();
            struggler_group.push(wire);
            wireGroups.push(struggler_group);
        }
    }
    

    // Clean empty groups
    wireGroups.retain(|group| group.len() > 0);

    return wireGroups;
}

// pub fn sort_wirelist_by_left_device_pin(wirelist: &mut WireList) {
//     wirelist.wires.sort_by(|a,b| {
//         if let Some(left_end) = a.left {

//         }
//     }
// }