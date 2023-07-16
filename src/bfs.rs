use petgraph::visit::VisitMap;
use petgraph::visit::GraphRef;
use petgraph::visit::Visitable;
use petgraph::visit::IntoNeighbors;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Bfs<N, VM> {
    /// The queue of nodes to visit
    pub stack: VecDeque<VecDeque<N>>,
    /// The map of discovered nodes
    pub discovered: VM,
}

impl<N, VM> Default for Bfs<N, VM>
where
    VM: Default,
{
    fn default() -> Self {
        Bfs {
            stack: VecDeque::new(),
            discovered: VM::default(),
        }
    }
}

impl<N, VM> Bfs<N, VM>
where
    N: Copy + PartialEq,
    VM: VisitMap<N>,
{
    /// Create a new **Bfs**, using the graph's visitor map, and put **start**
    /// in the stack of nodes to visit.
    pub fn new<G>(graph: G, start: N) -> Self
    where
        G: GraphRef + Visitable<NodeId = N, Map = VM>,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut stack = VecDeque::new();
        let mut level_stack = VecDeque::new();
        level_stack.push_front(start);
        stack.push_front(level_stack);
        Bfs { stack, discovered }
    }

    /// Return the next node in the bfs and a boolean value in the tuple indicating if level has ended
    pub fn next<G>(&mut self, graph: G) -> (Option<N>, bool)
    where
        G: IntoNeighbors<NodeId = N>,
    {
        let mut result = None;
        let mut level_end = false;
        // Pop top level stack(we will put it back if it's not exhausted)
        if let Some(mut level_stack) = self.stack.pop_front() {
            // Pop next node from the level stack
            if let Some(node) = level_stack.pop_front() {
                let mut succ_level_stack = VecDeque::new();
                // Make a new level stack from current node
                for succ in graph.neighbors(node) {
                    if self.discovered.visit(succ) {
                        succ_level_stack.push_back(succ);
                    }
                }
                // If there's anything in the new level stack push it back onto main stack
                if succ_level_stack.len() > 0 {
                    self.stack.push_back(succ_level_stack);
                }
                // Assign return node
                result = Some(node);
            }
            // Check if current level stack is not exhausted and put it back onto main stack
            if level_stack.len() > 0 {
                self.stack.push_front(level_stack);    
            } else {
                // Otherwise, level ended, notify caller.
                level_end = true;
            }
        } // else no stacks left
        (result, level_end)
    }
}
