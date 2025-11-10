use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub content: String,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub is_expanded: bool,
}

impl Node {
    pub fn new(id: NodeId, content: String) -> Self {
        Self {
            id,
            content,
            parent: None,
            children: Vec::new(),
            is_expanded: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    nodes: Vec<Node>,
    root_nodes: Vec<NodeId>,
    next_id: usize,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_nodes: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_node(&mut self, content: String, parent: Option<NodeId>) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;

        let mut node = Node::new(id, content);
        node.parent = parent;

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.get_node_mut(parent_id) {
                parent_node.children.push(id);
            }
        } else {
            self.root_nodes.push(id);
        }

        self.nodes.push(node);
        id
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn get_root_nodes(&self) -> &[NodeId] {
        &self.root_nodes
    }

    pub fn get_all_nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn toggle_expanded(&mut self, id: NodeId) {
        if let Some(node) = self.get_node_mut(id) {
            node.is_expanded = !node.is_expanded;
        }
    }

    pub fn update_content(&mut self, id: NodeId, content: String) {
        if let Some(node) = self.get_node_mut(id) {
            node.content = content;
        }
    }

    pub fn delete_node(&mut self, id: NodeId) {
        if let Some(node) = self.get_node(id).cloned() {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.get_node_mut(parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            } else {
                self.root_nodes.retain(|&root_id| root_id != id);
            }

            let children = node.children.clone();
            for child_id in children {
                self.delete_node(child_id);
            }

            self.nodes.retain(|n| n.id != id);
        }
    }

    pub fn add_sibling(&mut self, sibling_id: NodeId, content: String) -> NodeId {
        let parent = self.get_node(sibling_id).and_then(|n| n.parent);
        self.add_node(content, parent)
    }

    pub fn indent_node(&mut self, id: NodeId) {
        if let Some(node) = self.get_node(id).cloned() {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.get_node(parent_id) {
                    let siblings = &parent.children;
                    if let Some(pos) = siblings.iter().position(|&nid| nid == id) {
                        if pos > 0 {
                            let new_parent_id = siblings[pos - 1];
                            
                            if let Some(parent) = self.get_node_mut(parent_id) {
                                parent.children.retain(|&child_id| child_id != id);
                            }
                            
                            if let Some(new_parent) = self.get_node_mut(new_parent_id) {
                                new_parent.children.push(id);
                            }
                            
                            if let Some(node) = self.get_node_mut(id) {
                                node.parent = Some(new_parent_id);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn outdent_node(&mut self, id: NodeId) {
        if let Some(node) = self.get_node(id).cloned() {
            if let Some(parent_id) = node.parent {
                if let Some(grandparent_id) = self.get_node(parent_id).and_then(|p| p.parent) {
                    if let Some(parent) = self.get_node_mut(parent_id) {
                        parent.children.retain(|&child_id| child_id != id);
                    }
                    
                    if let Some(grandparent) = self.get_node_mut(grandparent_id) {
                        grandparent.children.push(id);
                    }
                    
                    if let Some(node) = self.get_node_mut(id) {
                        node.parent = Some(grandparent_id);
                    }
                } else {
                    if let Some(parent) = self.get_node_mut(parent_id) {
                        parent.children.retain(|&child_id| child_id != id);
                    }
                    
                    if let Some(node) = self.get_node_mut(id) {
                        node.parent = None;
                    }
                    
                    self.root_nodes.push(id);
                }
            }
        }
    }

    pub fn reorder_children(&mut self, parent_id: Option<NodeId>, new_order: Vec<NodeId>) {
        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.get_node_mut(parent_id) {
                parent.children = new_order;
            }
        } else {
            self.root_nodes = new_order;
        }
    }

    pub fn move_node(&mut self, node_id: NodeId, new_parent_id: Option<NodeId>, position: usize) {
        if let Some(node) = self.get_node(node_id).cloned() {
            // Remove from old parent
            if let Some(old_parent_id) = node.parent {
                if let Some(old_parent) = self.get_node_mut(old_parent_id) {
                    old_parent.children.retain(|&id| id != node_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != node_id);
            }
            
            // Update node's parent
            if let Some(node) = self.get_node_mut(node_id) {
                node.parent = new_parent_id;
            }
            
            // Add to new parent
            if let Some(new_parent_id) = new_parent_id {
                if let Some(new_parent) = self.get_node_mut(new_parent_id) {
                    let pos = position.min(new_parent.children.len());
                    new_parent.children.insert(pos, node_id);
                }
            } else {
                let pos = position.min(self.root_nodes.len());
                self.root_nodes.insert(pos, node_id);
            }
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
