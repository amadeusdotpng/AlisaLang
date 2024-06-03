pub enum Nonterminal {
    Program,

    Statement, 
    Expression,

    VarDecl,
    FuncDecl,
    StructDecl,
    EnumDecl,
    ExpressionStatement,

    Params,

    IfExpression,
    BlockExpression,
    ClosureExpression,
    OperationExpression,

    AssignExpression,

    BoolOr,
    BoolAnd,
    BoolNot,
    Comparison,

    BitOr,
    BitXor,
    BitAnd,
    BitShift,

    Sum,
    Term,
    Factor,
    Primary,

    Literal,
    Boolean,
    Tuple,
    List,

    Type,
    Primitive,
    UserDefined
}
