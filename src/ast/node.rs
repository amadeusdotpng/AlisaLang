#[allow(unused_imports)]
use crate::ast;

#[derive(Debug)]
pub enum Statement {
    Function(FunctionStatement),
    Struct(StructStatement),
    Enum(EnumStatement),
    Let(LetStatement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Closure(Box<ClosureExpression>),
    Block(Box<BlockExpression>),
    Call(Box<CallExpression>),
    If(Box<IfExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Literal(LiteralExpression)
}



#[derive(Debug)]
pub struct FunctionStatement {
    pub name: String,
    pub arguments: Vec<Parameter>,
    pub return_type: Type,
    pub block: BlockExpression,
}

#[derive(Debug)]
pub struct StructStatement {
    pub name: String,
    pub fields: Vec<Parameter>,
}

#[derive(Debug)]
pub struct EnumStatement {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}



#[derive(Debug)]
pub struct ClosureExpression {
    pub arguments: Vec<Parameter>,
    pub expression: BlockExpression,
    pub return_type: Type,
}

#[derive(Debug)]
pub struct CallExpression {
    pub function_name: String,
    pub arguments: Vec<Parameter>
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
    pub expression: Option<Expression>,
}

#[derive(Debug)]
pub struct IfExpression {
    // This will get type-checked to see if it boils down into the `bool` type.
    pub condition: Expression,
    pub body: BlockExpression,
    pub else_body: Box<ElseExpression>,
}

#[derive(Debug)]
pub enum ElseExpression {
    Else(BlockExpression),
    ElseIf(IfExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub rhs: Expression,
    pub op: BinaryOperator,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub rhs: Expression,
    pub op : UnaryOperator,
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub kind: LiteralKind
}

#[derive(Debug)]
pub enum LiteralKind {
    Bool(bool),
    Int(u128),
    Float(f64),
    Str(String),
    Char(char),
    Tuple(Tuple),
    List(List),
}

#[derive(Debug)]
pub struct Tuple(pub Vec<Expression>);
#[derive(Debug)]
pub struct List(pub Vec<Expression>);

#[derive(Debug)]
pub enum Type {
    Bool,
    Int { sign: bool, kind: IntKind },
    Float { kind: FloatKind },
    Str,
    Char,
    Tuple(TupleType),
    List(Box<Type>),
    Fn { arguments: Vec<Type>, return_type: Box<Type> },
    Void,
    UserDefined { name: String },
}

#[derive(Debug)]
pub enum IntKind {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
}

#[derive(Debug)]
pub enum FloatKind {
    Bit32,
    Bit64,
}

#[derive(Debug)]
pub struct TupleType(pub Vec<Type>);
