#[macro_export]
macro_rules! opt {
    () => { None };
    ($($value:tt)*) => { Some($($value)*) };
}

#[macro_export]
macro_rules! stmt {
    (print $value:expr) => {
        Statement::Print { value: $value }
    };
    (decl $name:literal $value:expr) => {
        Statement::VariableDeclaration {
            identifier: $name.to_string(),
            variable_type: None,
            value: $value
        }
    };
    (decl $name:literal $vartype:literal $value:expr) => {
        Statement::VariableDeclaration {
            identifier: $name.to_string(),
            variable_type: Some($vartype.to_string()),
            value: $value
        }
    };
    (assign $id:literal $value:expr) => {
        Statement::VariableAssignment {
            identifier: $id,
            value: $value
        }
    };
    (func $name:literal ( $( $id:literal $param_type:literal ),* ) $return_type:literal { $( $stmt:expr ),* } $( $ret:expr)? ) => {
        Statement::FunctionDeclaration {
            identifier: $name.to_string(),
            param_list: vec![ 
                $( Parameter {
                    identifier: $id.to_string(),
                    param_type: $param_type.to_string()
                } ),*
            ],
            return_type: Some($return_type.to_string()),
            block: Box::new(vec![
                $( $stmt, )* 
                stmt!(retstmt $name $( $ret )? )
            ])
        }
    };
    (func $name:literal ( $( $id:literal $param_type:literal ),* ) { $( $stmt:expr ),* } $( $ret:expr)? ) => {
        Statement::FunctionDeclaration {
            identifier: $name.to_string(),
            param_list: vec![ 
                $( Parameter {
                    identifier: $id.to_string(),
                    param_type: $param_type.to_string()
                } ),*
            ],
            return_type: None,
            block: Box::new(vec![
                $( $stmt, )* 
                stmt!(retstmt $name $( $ret )? )
            ])
        }
    };
    (def $id:literal { $( $stmt:expr ),* } $( $ret:expr)? ) => {
        Statement::FunctionDefinition {
            identifier: $id,
            block: Box::new(vec![
                $( $stmt, )* 
                stmt!(ret $id $( $ret )? )
            ])
        }
    };
    (retstmt $name:literal) => {
        Statement::ReturnStatement { function:$name.to_string(), return_value: None }
    };
    (retstmt $name:literal $value:expr) => {
        Statement::ReturnStatement { function:$name.to_string(), return_value: Some($value) }
    };
    (ret $id:literal) => {
        Statement::ResolvedReturn { function:$id, return_value: None }
    };
    (ret $id:literal $value:expr) => {
        Statement::ResolvedReturn { function:$id, return_value: Some($value) }
    };
    (call $expression:expr) => {
        Statement::Call { expression:$expression }
    };
}

#[macro_export]
macro_rules! expr {
    (stri $value:literal) => {
        Expression::StringValue { value:$value.to_string() }
    };
    (num $value:literal) => {
        Expression::Number { value:$value }
    };
    (var $id:literal) => {
        Expression::Variable { identifier:$id.to_string() }
    };
    (loc $id:literal $depth:literal) => {
        Expression::LocalVar { id:$id, depth:$depth }
    };
    (bin $op:ident $lhs:expr, $rhs:expr) => {
        Expression::BinaryOperation {
            op_type: OperationType::$op,
            lhs: Box::new($lhs),
            rhs: Box::new($rhs)
        }
    };
    (fncall $id:literal ( $( $arg:expr ),* ) ) => {
        Expression::FunctionCall {
            identifier:$id.to_string(),
            argument_list: Box::new(vec![ $( $arg ),* ])
        }
    };
    (rescall $id:literal ( $( $arg:expr ),* ) ) => {
        Expression::ResolvedFunctionCall {
            id:$id,
            argument_list: Box::new(vec![ $( $arg ),* ])
        }
    };
    (cast $otype:literal $oper:expr) => {
        Expression::Cast {
            operand:Box::new($oper),
            output_type:$otype.to_string()
        }
    };
    (rcast $otype:ident $oper:expr) => {
        Expression::ResolvedCast {
            operand:Box::new($oper),
            output_type:Type::$otype
        }
    };
    (acast $itype:ident $otype:ident $oper:expr) => {
        Expression::AnalyzedCast {
            operand:Box::new($oper),
            input_type:Type::$itype,
            output_type:Type::$otype
        }
    };
}

#[macro_export]
macro_rules! res_ast {
    (ast [ $( $stmt:expr ),* ] 
     $( var [ $( $varname:literal $id:literal $( $vartype:ident )? ),* ] )? 
     $( func [ $( $fname:literal ( $( $param_name:literal $param_type:ident ),* ) $( $return_type:ident )? ),* ] )? ) => {
        ResolvedAst {
            ast: vec![ $( $stmt ),* ],
            symtable: Symtable {
                variable_table: vec![ $(
                    $( LocalVariable {
                        identifier: $varname.to_string(),
                        index: $id,
                        vartype: opt!($(Type::$vartype)?)
                    } ),*
                )?],
                function_table: vec![ $(
                    $( Function {
                        identifier: $fname.to_string(),
                        return_type: opt!($(Type::$return_type)?),
                        param_list: vec![
                            $( ParamType {
                                identifier: $param_name.to_string(),
                                param_type: Type::$param_type
                            } ),*
                        ]
                    } ),*
                )?]
            },
            error_mode: false
        }
    };
}

#[macro_export]
macro_rules! an_ast {
    (ast [ $( $stmt:expr ),* ] 
     $( var [ $( $varname:literal $id:literal $( $vartype:ident )? ),* ] )? 
     $( func [ $( $fname:literal ( $( $param_name:literal $param_type:ident ),* ) $( $return_type:ident )? ),* ] )? ) => {
        AnalyzedAst {
            ast: vec![ $( $stmt ),* ],
            symtable: Symtable {
                variable_table: vec![ $(
                    $( LocalVariable {
                        identifier: $varname.to_string(),
                        index: $id,
                        vartype: opt!($(Type::$vartype)?)
                    } ),*
                )?],
                function_table: vec![ $(
                    $( Function {
                        identifier: $fname.to_string(),
                        return_type: opt!($(Type::$return_type)?),
                        param_list: vec![
                            $( ParamType {
                                identifier: $param_name.to_string(),
                                param_type: Type::$param_type
                            } ),*
                        ]
                    } ),*
                )?]
            }
        }
    };
}

#[macro_export]
macro_rules! comp_prog {
    (main [ $( $byte:ident $( $value:literal )? ),* ]
     $(stri [ $( $stri:literal ),* ])? 
     $( func [ $( { $( $fnbyte:ident $( $fnvalue:literal )? ),* } ),* ] )? ) => {
        CompiledProgram {
            main_program: vec![
                $( comp_prog!($byte$( $value)?) ),*
            ].into_iter().flatten().collect(),
            string_pool: vec![ $(
                $( $stri.to_string() ),*
            )? ],
            function_pool: vec![ $(
                $( vec![
                    $( comp_prog!($fnbyte$( $fnvalue)?) ),*
                ].into_iter().flatten().collect() ),*
            )? ]
        }
    };
    (val8 $num:literal) => {
        vec![NumSize::_8 as u8,
            u8::from($num)]
    };
    (val32 $num:literal) => {
        {
            vec![NumSize::_32 as u8]
                .append(u32::from($num).to_ne_bytes())
        }
    };
    (val64 $num:literal) => {
        {
            vec![NumSize::_64 as u8]
                .append(u64::from($num).to_ne_bytes())
        }
    };
    ($op:ident) => {
        vec![Op::$op as u8]
    };
}
