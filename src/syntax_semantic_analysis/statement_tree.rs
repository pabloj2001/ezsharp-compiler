use crate::{lexical_analysis::Token, logger::Loggable};

use super::symbol_declaration::{BasicType, SymbolDecl};

#[derive(Debug, Clone)]
pub enum StatementSymbol {
    Decl(SymbolDecl),
    Literal(Token),
    Operator(Token),
    SingleChildOperator(Token),
    FunctionCall(SymbolDecl, Vec<StatementTree>),
}

impl Loggable for StatementSymbol {
    fn to_log_message(&self) -> String {
        match self {
            StatementSymbol::Decl(decl) => decl.name.clone(),
            StatementSymbol::Literal(token) => {
                match token {
                    Token::Tint(value) => value.to_string(),
                    Token::Tdouble(value) => value.to_string(),
                    _ => token.to_log_message(),
                }
            },
            StatementSymbol::Operator(token) | Self::SingleChildOperator(token) => {
                match token {
                    Token::Soparen => String::from("()"),
                    Token::Oplus => String::from("+"),
                    Token::Ominus => String::from("-"),
                    Token::Omultiply => String::from("*"),
                    Token::Odivide => String::from("/"),
                    Token::Omod => String::from("%"),
                    Token::Kand => String::from("and"),
                    Token::Kor => String::from("or"),
                    Token::Knot => String::from("not"),
                    Token::Olt => String::from("<"),
                    Token::Ogt => String::from(">"),
                    Token::Olte => String::from("<="),
                    Token::Ogte => String::from(">="),
                    _ => token.to_log_message(),
                }
            },
            StatementSymbol::FunctionCall(decl, params) => {
                let mut message = decl.name.clone();
                message.push_str("(\n");
                for param in params {
                    message.push_str(&param.to_log_message());
                    message.pop();
                    message.push_str(",\n");
                }
                message.push_str(")");
                message
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub symbol: StatementSymbol,
    pub node_type: Option<BasicType>,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl StatementNode {
    pub fn has_both_children(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

impl Loggable for StatementNode {
    fn to_log_message(&self) -> String {
        format!(
            "{}: {}, {}\n",
            self.symbol.to_log_message(),
            self.left.unwrap_or(0),
            self.right.unwrap_or(0)
        )
    }
}

#[derive(Debug, Clone)]
pub struct StatementTree {
    pub nodes: Vec<StatementNode>,
    pub start: Option<usize>,
}

impl StatementTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            start: None,
        }
    }

    pub fn add_node(&mut self, symbol: StatementSymbol, parent: Option<usize>) -> usize {
        self.nodes.push(StatementNode {
            symbol,
            node_type: None,
            parent,
            left: None,
            right: None,
        });
        let node_index = self.nodes.len() - 1;

        if let Some(parent_index) = parent {
            // Place node in free branch
            if let Some(node) = self.nodes.get_mut(parent_index) {
                if node.left.is_none() {
                    node.left = Some(node_index);
                } else {
                    node.right = Some(node_index);
                }
            }
        } else {
            self.start = Some(node_index);
        }

        node_index
    }

    pub fn split_tree(&mut self, symbol: StatementSymbol, node: usize) -> usize {
        // Create new node with current node as left child
        let parent_index = self.nodes[node].parent;
        self.nodes.push(StatementNode {
            symbol,
            node_type: None,
            parent: parent_index,
            left: Some(node),
            right: None,
        });
        let new_node = self.nodes.len() - 1;

        if let Some(parent) = parent_index {
            // Update parent node
            if let Some(parent_node) = self.nodes.get_mut(parent) {
                if parent_node.left == Some(node) {
                    parent_node.left = Some(new_node);
                } else {
                    parent_node.right = Some(new_node);
                }
            }
        } else {
            // Update start node
            self.start = Some(new_node);
        }

        // Update current node
        self.nodes[node].parent = Some(new_node);

        new_node
    }
}

impl Loggable for StatementTree {
    fn to_log_message(&self) -> String {
        let mut message = format!("Start: {}\n", self.start.unwrap_or(0));

        let mut node_stack = vec![self.start];
        while !node_stack.is_empty() {
            if let Some(curr_node) = node_stack.pop().unwrap() {
                let node = &self.nodes[curr_node];
                message.push_str(&format!("({}) {}", curr_node, node.to_log_message()));
                
                if node.right.is_some() {
                    node_stack.push(node.right);
                }
                if node.left.is_some() {
                    node_stack.push(node.left);
                }
            }
        }

        message.pop();
        message
    }
}

pub struct StatementTreeInfo {
    pub tree: StatementTree,
    pub curr_node: Option<usize>,
}

impl StatementTreeInfo {
    pub fn new() -> Self {
        Self {
            tree: StatementTree::new(),
            curr_node: None,
        }
    }
}