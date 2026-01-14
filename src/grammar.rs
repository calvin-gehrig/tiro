use std::collections::HashMap;

use crate::lexer::Token;

#[macro_export]
macro_rules! symbol {
    ( __, n ) => {
        Symbol::NonTerminal(RuleId::MaybeIgnore)
    };
    ( ___, n ) => {
        Symbol::NonTerminal(RuleId::Ignore)
    };
    ( $non_terminal:ident, n ) => {
        Symbol::NonTerminal(RuleId::$non_terminal)
    };
    ( $token:ident, t ) => {
        Symbol::Terminal(Token::$token)
    };
    ( $token:ident, s ) => {
        Symbol::Terminal(Token::$token(String::from("")))
    };
    ( $token:ident, i ) => {
        Symbol::Terminal(Token::$token(0))
    };
    ( $_:ident, $__:ident) => {
        panic!("Unexpected symbol type");
    };
}

#[macro_export]
macro_rules! grammar_rule {
    { $start_rule_id:ident := $( $start_symbol_id:ident : $start_symbol_type:ident )*,
     $( $rule_id:ident := $( $symbol_id:ident : $symbol_type:ident )*
         $( | $( $other_symbol_id:ident : $other_symbol_type:ident )* )* ),* } => {

        #[derive(Debug, Hash, PartialEq, Eq, Clone)]
        enum RuleId {
            $start_rule_id,
            $( $rule_id ),*
        }
        impl Grammar {
            pub fn new() -> Self {

                Self {
                    start: vec![
                            $( symbol!( $start_symbol_id, $start_symbol_type), )*
                        ],
                    rules: HashMap::from([ 
                        $( (
                            RuleId::$rule_id,
                            vec![ 
                                vec![ $( symbol!( $symbol_id, $symbol_type ), )* ],
                                $(
                                    vec![ $( symbol!( $other_symbol_id, $other_symbol_type ), )* ], 
                                )*
                            ]
                        ), )*
                    ])
                }

            }
        }
    };
}

type Rule = Vec<Symbol>;

#[derive(Debug, Clone)]
enum Symbol {
    Terminal(Token),
    NonTerminal(RuleId)
}

#[derive(Debug)]
pub struct Grammar {
    start: Rule,
    rules: HashMap<RuleId, Vec<Rule>>
}

impl Grammar {
    fn get_rules(&self, rule_id: RuleId) -> &Vec<Rule> {
        self.rules.get(&rule_id)
            .expect("Earley parser error")
    }
    fn starting_rule(&self) -> &Rule {
        self.start
    }
}

grammar_rule!{
    Program := Block:n __:n EndOfProgram:t,
    Block := Statement:n
        | Statement:n __:n Block:n,
    Statement := PrintStatement:n
        | VariableDeclaration:n,

    PrintStatement := Print:t __:n Expression:n,
    VariableDeclaration := Let:t __:n Id:s __:n Expression:n,

    Expression := Equality:n,

    Equality := Equality:n __:n EqualityOperator:n __:n BinaryOperation:n
        | BinaryOperation:n,
    EqualityOperator := EqualOperator:t | NotEqualOperator:t,

    BinaryOperation := BinaryOperation:n __:n BinaryOperator:n __:n Value:n
        | Value:n,
    BinaryOperator := AddOperator:t | SubtractOperator:t | MultiplyOperator:t | DivideOperator:t,

    Value := Id:s | StringToken:s | Number:i,

    MaybeIgnore := Ignore:n
        | Null:n,
    Ignore := Whitespace:t
        | Whitespace:t Ignore:n,
    Null :=
}
