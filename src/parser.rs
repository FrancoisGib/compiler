use crate::tokenizer;
use tokenizer::{Token, tokenize};

trait NodeParser {
    fn parse(parser: &mut Parser) {
    }
}

enum Literal {
    StringLiteral(String),
    NumericLiteral(i32),
}

enum Operand {
    Identifier(AstNode::Identifier),
    Literal(Literal),
}

type Operator = String;

enum AstNode {
    AssignmentExpression(AstNode::Identifier, Operand),
    BinaryExpression(Operand, Operator, Operator),
    Identifier(String, AstNode::LiteralNode),
    LiteralNode(Literal),
    BlockStatement(Vec<AstNode>)
}

impl NodeParser for AstNode::AssignmentExpression {
    fn parse(parser: &mut Parser) {

    }
}

impl NodeParser for AstNode {
    fn parse(parser: &mut Parser) {

    }
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn parse(tokens: Vec<Token>) -> AstNode {
        let mut root_vec = vec![];
        let root = AstNode::BlockStatement(root_vec);
        root
    }
}
