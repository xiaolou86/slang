use std::rc::Rc;

use crate::{types, yaml::cst};

use super::{
    node::Node,
    parser::{Parser, ParserDefinition, ParserRef},
    production::ConcreteAbstractPair,
};

pub type PrecedenceParserRef = Rc<PrecedenceParser>;

pub struct PrecedenceParser {
    pub name: Option<Node<String>>,
    pub definition: PrecedenceParserDefinition,
}

impl ConcreteAbstractPair for PrecedenceParser {
    type AbstractType = types::precedence_parser::PrecedenceParser;

    fn new(cst_node: &cst::NodeRef, value: Rc<Self::AbstractType>) -> Rc<Self> {
        return Rc::new(Self {
            name: value.name.clone().and_then(|name| {
                return Some(Node::new(cst_node.value_of_field("name"), name));
            }),
            definition: PrecedenceParserDefinition::new(cst_node, value.definition.clone()),
        });
    }
}

pub struct PrecedenceParserDefinition {
    pub operators: Vec<Node<OperatorDefinition>>,
    pub primary_expression: ParserRef,
}

impl PrecedenceParserDefinition {
    pub fn new(
        cst_node: &cst::NodeRef,
        value: types::precedence_parser::PrecedenceParserDefinition,
    ) -> Self {
        return Self {
            operators: {
                let cst_node = cst_node.field("operators");
                cst_node.zip(value.operators, OperatorDefinition::new)
            },
            primary_expression: Parser::new(
                &cst_node.value_of_field("primaryExpression"),
                value.primary_expression,
            ),
        };
    }
}

pub struct OperatorDefinition {
    pub name: Node<String>,
    pub model: Node<types::precedence_parser::OperatorModel>,
    pub definition: Node<ParserDefinition>,
}

impl OperatorDefinition {
    pub fn new(
        cst_node: &cst::NodeRef,
        value: types::precedence_parser::OperatorDefinition,
    ) -> Node<Self> {
        return Node::new(
            &cst_node,
            OperatorDefinition {
                name: Node::new(cst_node.value_of_field("name"), value.name),
                model: Node::new(cst_node.value_of_field("model"), value.model),
                definition: ParserDefinition::new(cst_node, value.definition),
            },
        );
    }
}
