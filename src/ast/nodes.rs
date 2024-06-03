#[allow(unused_imports)]
use crate::ast;

pub enum Statement {
    Function(FunctionStatement),
    Struct(StructStatement),
    Enum(EnumStatement),
    Expression(Expression),
}

pub struct FunctionStatement {
    name: String,
    arguments: Vec<Argument>,
    block: BlockExpression,
}

pub struct StructStatement {
    name: String,
    fields: Vec<Argument>,
}

pub struct EnumStatement {
    name: String,
    variants: Vec<String>,
}

pub enum Expression {
    Closure(Box<ClosureExpression>),
    Block(Box<BlockExpression>),
    Call(Box<CallExpression>),
    If(Box<IfExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Literal(LiteralExpression)
}


pub struct ClosureExpression {
    arguments: Vec<Argument>,
    expression: BlockExpression,
    return_type: Type,
}


pub struct BlockExpression {
    statements: Vec<Statement>,
    expression: Option<Expression>,
}


pub struct CallExpression {
    function_name: String,
    arguments: Vec<Argument>
}


pub struct IfExpression {
    // This will get type-checked to see if it boils down into the `bool` type.
    condition: Expression,
    body: BlockExpression,
    else_body: Box<ElseExpression>,
}

pub enum ElseExpression {
    Else(BlockExpression),
    ElseIf(IfExpression),
}


pub struct BinaryExpression {
    lhs: Expression,
    rhs: Expression,
    op: BinaryOperator,
}


pub struct UnaryExpression {
    rhs: Expression,
    op: UnaryOperator,
}

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    BitOr,
    BitAnd,
    BitXor,
    BitNot,
    BitRight,
    BitLeft,

    BoolOr,
    BoolAnd,
    BoolNot,

    Eq,
    Ne,
    Ge,
    Le,
    Gt,
    Lt
}

// Leading Plus/Minus signs
pub enum UnaryOperator {
    Plus,
    Minus,
}

pub struct LiteralExpression {
    kind: LiteralKind
}

pub enum LiteralKind {
    Bool,
    Int,
    Float,
    Str,
    Char,
    Tuple,
    List,
}

pub struct Argument {
    name: String,
    arg_type: Type,
}

pub enum Type {
    Bool,
    Int { sign: bool, kind: IntKind },
    Float { kind: FloatKind },
    Str,
    Char,
    Tuple,
    List,
    UserDefined { name: String },
}

pub enum IntKind {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
}

pub enum FloatKind {
    Bit32,
    Bit64,
}
