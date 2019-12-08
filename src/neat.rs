use sdl2::render::WindowCanvas;

#[derive(Debug)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
}

#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub nodeType: NodeType,
}

impl Node {
    pub fn new(id: i32, nodeType: NodeType) -> Node {
        Node {
            id: id,
            nodeType: nodeType,
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub inNode: i32,
    pub outNode: i32,
    pub weight: f32,
    pub enabled: bool,
    pub inno: i32,
}

pub struct Network {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub fitness: f32,
}

impl Network {
    pub fn new(numInputs: i32, numOutputs: i32) -> Network {
        let mut nodes: Vec<Node> = Vec::new();

        // Input Nodes
        for i in 0..numInputs {
            nodes.push(Node::new(i + 1, NodeType::Input));
        }

        // Output Nodes
        for i in numInputs..(numInputs + numOutputs) {
            nodes.push(Node::new(i + 1, NodeType::Output));
        }


        Network {
            nodes: nodes,
            connections: Vec::new(),
            fitness: 0.0,
        }
    }

}