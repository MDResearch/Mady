// codegen file by version 0.1.0
// don't edit this

/// chain of responsibility trait
/// it is a call & return trait
#[allow(unused)]
pub trait Chain {
    type Input;
    type Err;
    fn chain_abi(
        &mut self,
        c: &mut Self::Input,
        t: syn::Abi,
    ) -> Result<syn::Abi, Self::Err> {
        Ok(t)
    }
    fn chain_anglebracketedgenericarguments(
        &mut self,
        c: &mut Self::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::AngleBracketedGenericArguments, Self::Err> {
        Ok(t)
    }
    fn chain_arm(
        &mut self,
        c: &mut Self::Input,
        t: syn::Arm,
    ) -> Result<syn::Arm, Self::Err> {
        Ok(t)
    }
    fn chain_attrstyle(
        &mut self,
        c: &mut Self::Input,
        t: syn::AttrStyle,
    ) -> Result<syn::AttrStyle, Self::Err> {
        Ok(t)
    }
    fn chain_attrstyle_inner(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Bang,
    ) -> Result<syn::AttrStyle, Self::Err> {
        Ok(syn::AttrStyle::Inner(t))
    }
    fn chain_attribute(
        &mut self,
        c: &mut Self::Input,
        t: syn::Attribute,
    ) -> Result<syn::Attribute, Self::Err> {
        Ok(t)
    }
    fn chain_barefnarg(
        &mut self,
        c: &mut Self::Input,
        t: syn::BareFnArg,
    ) -> Result<syn::BareFnArg, Self::Err> {
        Ok(t)
    }
    fn chain_binop(
        &mut self,
        c: &mut Self::Input,
        t: syn::BinOp,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(t)
    }
    fn chain_binop_add(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Add,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Add(t))
    }
    fn chain_binop_sub(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Sub,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Sub(t))
    }
    fn chain_binop_mul(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Star,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Mul(t))
    }
    fn chain_binop_div(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Div,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Div(t))
    }
    fn chain_binop_rem(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Rem,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Rem(t))
    }
    fn chain_binop_and(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::AndAnd,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::And(t))
    }
    fn chain_binop_or(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::OrOr,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Or(t))
    }
    fn chain_binop_bitxor(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Caret,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitXor(t))
    }
    fn chain_binop_bitand(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::And,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitAnd(t))
    }
    fn chain_binop_bitor(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Or,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitOr(t))
    }
    fn chain_binop_shl(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Shl,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Shl(t))
    }
    fn chain_binop_shr(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Shr,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Shr(t))
    }
    fn chain_binop_eq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::EqEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Eq(t))
    }
    fn chain_binop_lt(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Lt,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Lt(t))
    }
    fn chain_binop_le(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Le,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Le(t))
    }
    fn chain_binop_ne(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Ne,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Ne(t))
    }
    fn chain_binop_ge(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Ge,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Ge(t))
    }
    fn chain_binop_gt(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Gt,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::Gt(t))
    }
    fn chain_binop_addeq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::AddEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::AddEq(t))
    }
    fn chain_binop_subeq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::SubEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::SubEq(t))
    }
    fn chain_binop_muleq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::MulEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::MulEq(t))
    }
    fn chain_binop_diveq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::DivEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::DivEq(t))
    }
    fn chain_binop_remeq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::RemEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::RemEq(t))
    }
    fn chain_binop_bitxoreq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::CaretEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitXorEq(t))
    }
    fn chain_binop_bitandeq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::AndEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitAndEq(t))
    }
    fn chain_binop_bitoreq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::OrEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::BitOrEq(t))
    }
    fn chain_binop_shleq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::ShlEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::ShlEq(t))
    }
    fn chain_binop_shreq(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::ShrEq,
    ) -> Result<syn::BinOp, Self::Err> {
        Ok(syn::BinOp::ShrEq(t))
    }
    fn chain_binding(
        &mut self,
        c: &mut Self::Input,
        t: syn::Binding,
    ) -> Result<syn::Binding, Self::Err> {
        Ok(t)
    }
    fn chain_block(
        &mut self,
        c: &mut Self::Input,
        t: syn::Block,
    ) -> Result<syn::Block, Self::Err> {
        Ok(t)
    }
    fn chain_boundlifetimes(
        &mut self,
        c: &mut Self::Input,
        t: syn::BoundLifetimes,
    ) -> Result<syn::BoundLifetimes, Self::Err> {
        Ok(t)
    }
    fn chain_constparam(
        &mut self,
        c: &mut Self::Input,
        t: syn::ConstParam,
    ) -> Result<syn::ConstParam, Self::Err> {
        Ok(t)
    }
    fn chain_constraint(
        &mut self,
        c: &mut Self::Input,
        t: syn::Constraint,
    ) -> Result<syn::Constraint, Self::Err> {
        Ok(t)
    }
    fn chain_data(
        &mut self,
        c: &mut Self::Input,
        t: syn::Data,
    ) -> Result<syn::Data, Self::Err> {
        Ok(t)
    }
    fn chain_data_struct(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataStruct,
    ) -> Result<syn::Data, Self::Err> {
        Ok(syn::Data::Struct(t))
    }
    fn chain_data_enum(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataEnum,
    ) -> Result<syn::Data, Self::Err> {
        Ok(syn::Data::Enum(t))
    }
    fn chain_data_union(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataUnion,
    ) -> Result<syn::Data, Self::Err> {
        Ok(syn::Data::Union(t))
    }
    fn chain_dataenum(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataEnum,
    ) -> Result<syn::DataEnum, Self::Err> {
        Ok(t)
    }
    fn chain_datastruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataStruct,
    ) -> Result<syn::DataStruct, Self::Err> {
        Ok(t)
    }
    fn chain_dataunion(
        &mut self,
        c: &mut Self::Input,
        t: syn::DataUnion,
    ) -> Result<syn::DataUnion, Self::Err> {
        Ok(t)
    }
    fn chain_deriveinput(
        &mut self,
        c: &mut Self::Input,
        t: syn::DeriveInput,
    ) -> Result<syn::DeriveInput, Self::Err> {
        Ok(t)
    }
    fn chain_expr(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(t)
    }
    fn chain_expr_array(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprArray,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Array(t))
    }
    fn chain_expr_assign(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssign,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Assign(t))
    }
    fn chain_expr_assignop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssignOp,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::AssignOp(t))
    }
    fn chain_expr_async(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAsync,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Async(t))
    }
    fn chain_expr_await(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAwait,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Await(t))
    }
    fn chain_expr_binary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Binary(t))
    }
    fn chain_expr_block(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBlock,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Block(t))
    }
    fn chain_expr_box(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBox,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Box(t))
    }
    fn chain_expr_break(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBreak,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Break(t))
    }
    fn chain_expr_call(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprCall,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Call(t))
    }
    fn chain_expr_cast(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprCast,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Cast(t))
    }
    fn chain_expr_closure(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprClosure,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Closure(t))
    }
    fn chain_expr_continue(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprContinue,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Continue(t))
    }
    fn chain_expr_field(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprField,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Field(t))
    }
    fn chain_expr_forloop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprForLoop,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::ForLoop(t))
    }
    fn chain_expr_group(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprGroup,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Group(t))
    }
    fn chain_expr_if(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprIf,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::If(t))
    }
    fn chain_expr_index(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprIndex,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Index(t))
    }
    fn chain_expr_let(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLet,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Let(t))
    }
    fn chain_expr_lit(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLit,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Lit(t))
    }
    fn chain_expr_loop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLoop,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Loop(t))
    }
    fn chain_expr_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMacro,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Macro(t))
    }
    fn chain_expr_match(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMatch,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Match(t))
    }
    fn chain_expr_methodcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::MethodCall(t))
    }
    fn chain_expr_paren(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprParen,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Paren(t))
    }
    fn chain_expr_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprPath,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Path(t))
    }
    fn chain_expr_range(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprRange,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Range(t))
    }
    fn chain_expr_reference(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprReference,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Reference(t))
    }
    fn chain_expr_repeat(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprRepeat,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Repeat(t))
    }
    fn chain_expr_return(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprReturn,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Return(t))
    }
    fn chain_expr_struct(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprStruct,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Struct(t))
    }
    fn chain_expr_try(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTry,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Try(t))
    }
    fn chain_expr_tryblock(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTryBlock,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::TryBlock(t))
    }
    fn chain_expr_tuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTuple,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Tuple(t))
    }
    fn chain_expr_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprType,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Type(t))
    }
    fn chain_expr_unary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprUnary,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Unary(t))
    }
    fn chain_expr_unsafe(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprUnsafe,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Unsafe(t))
    }
    fn chain_expr_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Verbatim(t))
    }
    fn chain_expr_while(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprWhile,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::While(t))
    }
    fn chain_expr_yield(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprYield,
    ) -> Result<syn::Expr, Self::Err> {
        Ok(syn::Expr::Yield(t))
    }
    fn chain_exprarray(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprArray,
    ) -> Result<syn::ExprArray, Self::Err> {
        Ok(t)
    }
    fn chain_exprassign(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssign,
    ) -> Result<syn::ExprAssign, Self::Err> {
        Ok(t)
    }
    fn chain_exprassignop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssignOp,
    ) -> Result<syn::ExprAssignOp, Self::Err> {
        Ok(t)
    }
    fn chain_exprasync(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAsync,
    ) -> Result<syn::ExprAsync, Self::Err> {
        Ok(t)
    }
    fn chain_exprawait(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAwait,
    ) -> Result<syn::ExprAwait, Self::Err> {
        Ok(t)
    }
    fn chain_exprbinary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, Self::Err> {
        Ok(t)
    }
    fn chain_exprblock(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBlock,
    ) -> Result<syn::ExprBlock, Self::Err> {
        Ok(t)
    }
    fn chain_exprbox(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBox,
    ) -> Result<syn::ExprBox, Self::Err> {
        Ok(t)
    }
    fn chain_exprbreak(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBreak,
    ) -> Result<syn::ExprBreak, Self::Err> {
        Ok(t)
    }
    fn chain_exprcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprCall,
    ) -> Result<syn::ExprCall, Self::Err> {
        Ok(t)
    }
    fn chain_exprcast(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprCast,
    ) -> Result<syn::ExprCast, Self::Err> {
        Ok(t)
    }
    fn chain_exprclosure(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprClosure,
    ) -> Result<syn::ExprClosure, Self::Err> {
        Ok(t)
    }
    fn chain_exprcontinue(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprContinue,
    ) -> Result<syn::ExprContinue, Self::Err> {
        Ok(t)
    }
    fn chain_exprfield(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprField,
    ) -> Result<syn::ExprField, Self::Err> {
        Ok(t)
    }
    fn chain_exprforloop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprForLoop,
    ) -> Result<syn::ExprForLoop, Self::Err> {
        Ok(t)
    }
    fn chain_exprgroup(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprGroup,
    ) -> Result<syn::ExprGroup, Self::Err> {
        Ok(t)
    }
    fn chain_exprif(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprIf,
    ) -> Result<syn::ExprIf, Self::Err> {
        Ok(t)
    }
    fn chain_exprindex(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprIndex,
    ) -> Result<syn::ExprIndex, Self::Err> {
        Ok(t)
    }
    fn chain_exprlet(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLet,
    ) -> Result<syn::ExprLet, Self::Err> {
        Ok(t)
    }
    fn chain_exprlit(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLit,
    ) -> Result<syn::ExprLit, Self::Err> {
        Ok(t)
    }
    fn chain_exprloop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprLoop,
    ) -> Result<syn::ExprLoop, Self::Err> {
        Ok(t)
    }
    fn chain_exprmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMacro,
    ) -> Result<syn::ExprMacro, Self::Err> {
        Ok(t)
    }
    fn chain_exprmatch(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMatch,
    ) -> Result<syn::ExprMatch, Self::Err> {
        Ok(t)
    }
    fn chain_exprmethodcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::ExprMethodCall, Self::Err> {
        Ok(t)
    }
    fn chain_exprparen(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprParen,
    ) -> Result<syn::ExprParen, Self::Err> {
        Ok(t)
    }
    fn chain_exprpath(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprPath,
    ) -> Result<syn::ExprPath, Self::Err> {
        Ok(t)
    }
    fn chain_exprrange(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprRange,
    ) -> Result<syn::ExprRange, Self::Err> {
        Ok(t)
    }
    fn chain_exprreference(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprReference,
    ) -> Result<syn::ExprReference, Self::Err> {
        Ok(t)
    }
    fn chain_exprrepeat(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprRepeat,
    ) -> Result<syn::ExprRepeat, Self::Err> {
        Ok(t)
    }
    fn chain_exprreturn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprReturn,
    ) -> Result<syn::ExprReturn, Self::Err> {
        Ok(t)
    }
    fn chain_exprstruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprStruct,
    ) -> Result<syn::ExprStruct, Self::Err> {
        Ok(t)
    }
    fn chain_exprtry(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTry,
    ) -> Result<syn::ExprTry, Self::Err> {
        Ok(t)
    }
    fn chain_exprtryblock(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTryBlock,
    ) -> Result<syn::ExprTryBlock, Self::Err> {
        Ok(t)
    }
    fn chain_exprtuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTuple,
    ) -> Result<syn::ExprTuple, Self::Err> {
        Ok(t)
    }
    fn chain_exprtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprType,
    ) -> Result<syn::ExprType, Self::Err> {
        Ok(t)
    }
    fn chain_exprunary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprUnary,
    ) -> Result<syn::ExprUnary, Self::Err> {
        Ok(t)
    }
    fn chain_exprunsafe(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprUnsafe,
    ) -> Result<syn::ExprUnsafe, Self::Err> {
        Ok(t)
    }
    fn chain_exprwhile(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprWhile,
    ) -> Result<syn::ExprWhile, Self::Err> {
        Ok(t)
    }
    fn chain_expryield(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprYield,
    ) -> Result<syn::ExprYield, Self::Err> {
        Ok(t)
    }
    fn chain_field(
        &mut self,
        c: &mut Self::Input,
        t: syn::Field,
    ) -> Result<syn::Field, Self::Err> {
        Ok(t)
    }
    fn chain_fieldpat(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldPat,
    ) -> Result<syn::FieldPat, Self::Err> {
        Ok(t)
    }
    fn chain_fieldvalue(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldValue,
    ) -> Result<syn::FieldValue, Self::Err> {
        Ok(t)
    }
    fn chain_fields(
        &mut self,
        c: &mut Self::Input,
        t: syn::Fields,
    ) -> Result<syn::Fields, Self::Err> {
        Ok(t)
    }
    fn chain_fields_named(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldsNamed,
    ) -> Result<syn::Fields, Self::Err> {
        Ok(syn::Fields::Named(t))
    }
    fn chain_fields_unnamed(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::Fields, Self::Err> {
        Ok(syn::Fields::Unnamed(t))
    }
    fn chain_fieldsnamed(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldsNamed,
    ) -> Result<syn::FieldsNamed, Self::Err> {
        Ok(t)
    }
    fn chain_fieldsunnamed(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::FieldsUnnamed, Self::Err> {
        Ok(t)
    }
    fn chain_file(
        &mut self,
        c: &mut Self::Input,
        t: syn::File,
    ) -> Result<syn::File, Self::Err> {
        Ok(t)
    }
    fn chain_fnarg(
        &mut self,
        c: &mut Self::Input,
        t: syn::FnArg,
    ) -> Result<syn::FnArg, Self::Err> {
        Ok(t)
    }
    fn chain_fnarg_receiver(
        &mut self,
        c: &mut Self::Input,
        t: syn::Receiver,
    ) -> Result<syn::FnArg, Self::Err> {
        Ok(syn::FnArg::Receiver(t))
    }
    fn chain_fnarg_typed(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatType,
    ) -> Result<syn::FnArg, Self::Err> {
        Ok(syn::FnArg::Typed(t))
    }
    fn chain_foreignitem(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItem,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(t)
    }
    fn chain_foreignitem_fn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(syn::ForeignItem::Fn(t))
    }
    fn chain_foreignitem_static(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(syn::ForeignItem::Static(t))
    }
    fn chain_foreignitem_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(syn::ForeignItem::Type(t))
    }
    fn chain_foreignitem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(syn::ForeignItem::Macro(t))
    }
    fn chain_foreignitem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ForeignItem, Self::Err> {
        Ok(syn::ForeignItem::Verbatim(t))
    }
    fn chain_foreignitemfn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItemFn, Self::Err> {
        Ok(t)
    }
    fn chain_foreignitemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItemMacro, Self::Err> {
        Ok(t)
    }
    fn chain_foreignitemstatic(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItemStatic, Self::Err> {
        Ok(t)
    }
    fn chain_foreignitemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItemType, Self::Err> {
        Ok(t)
    }
    fn chain_genericargument(
        &mut self,
        c: &mut Self::Input,
        t: syn::GenericArgument,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(t)
    }
    fn chain_genericargument_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lifetime,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(syn::GenericArgument::Lifetime(t))
    }
    fn chain_genericargument_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::Type,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(syn::GenericArgument::Type(t))
    }
    fn chain_genericargument_binding(
        &mut self,
        c: &mut Self::Input,
        t: syn::Binding,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(syn::GenericArgument::Binding(t))
    }
    fn chain_genericargument_constraint(
        &mut self,
        c: &mut Self::Input,
        t: syn::Constraint,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(syn::GenericArgument::Constraint(t))
    }
    fn chain_genericargument_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> Result<syn::GenericArgument, Self::Err> {
        Ok(syn::GenericArgument::Const(t))
    }
    fn chain_genericmethodargument(
        &mut self,
        c: &mut Self::Input,
        t: syn::GenericMethodArgument,
    ) -> Result<syn::GenericMethodArgument, Self::Err> {
        Ok(t)
    }
    fn chain_genericmethodargument_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::Type,
    ) -> Result<syn::GenericMethodArgument, Self::Err> {
        Ok(syn::GenericMethodArgument::Type(t))
    }
    fn chain_genericmethodargument_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> Result<syn::GenericMethodArgument, Self::Err> {
        Ok(syn::GenericMethodArgument::Const(t))
    }
    fn chain_genericparam(
        &mut self,
        c: &mut Self::Input,
        t: syn::GenericParam,
    ) -> Result<syn::GenericParam, Self::Err> {
        Ok(t)
    }
    fn chain_genericparam_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParam,
    ) -> Result<syn::GenericParam, Self::Err> {
        Ok(syn::GenericParam::Type(t))
    }
    fn chain_genericparam_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::LifetimeDef,
    ) -> Result<syn::GenericParam, Self::Err> {
        Ok(syn::GenericParam::Lifetime(t))
    }
    fn chain_genericparam_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::ConstParam,
    ) -> Result<syn::GenericParam, Self::Err> {
        Ok(syn::GenericParam::Const(t))
    }
    fn chain_generics(
        &mut self,
        c: &mut Self::Input,
        t: syn::Generics,
    ) -> Result<syn::Generics, Self::Err> {
        Ok(t)
    }
    fn chain_implitem(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItem,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(t)
    }
    fn chain_implitem_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(syn::ImplItem::Const(t))
    }
    fn chain_implitem_method(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(syn::ImplItem::Method(t))
    }
    fn chain_implitem_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(syn::ImplItem::Type(t))
    }
    fn chain_implitem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(syn::ImplItem::Macro(t))
    }
    fn chain_implitem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ImplItem, Self::Err> {
        Ok(syn::ImplItem::Verbatim(t))
    }
    fn chain_implitemconst(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItemConst, Self::Err> {
        Ok(t)
    }
    fn chain_implitemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItemMacro, Self::Err> {
        Ok(t)
    }
    fn chain_implitemmethod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItemMethod, Self::Err> {
        Ok(t)
    }
    fn chain_implitemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItemType, Self::Err> {
        Ok(t)
    }
    fn chain_index(
        &mut self,
        c: &mut Self::Input,
        t: syn::Index,
    ) -> Result<syn::Index, Self::Err> {
        Ok(t)
    }
    fn chain_item(
        &mut self,
        c: &mut Self::Input,
        t: syn::Item,
    ) -> Result<syn::Item, Self::Err> {
        Ok(t)
    }
    fn chain_item_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemConst,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Const(t))
    }
    fn chain_item_enum(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemEnum,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Enum(t))
    }
    fn chain_item_externcrate(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemExternCrate,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::ExternCrate(t))
    }
    fn chain_item_fn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemFn,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Fn(t))
    }
    fn chain_item_foreignmod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemForeignMod,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::ForeignMod(t))
    }
    fn chain_item_impl(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemImpl,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Impl(t))
    }
    fn chain_item_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMacro,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Macro(t))
    }
    fn chain_item_macro2(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMacro2,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Macro2(t))
    }
    fn chain_item_mod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMod,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Mod(t))
    }
    fn chain_item_static(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemStatic,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Static(t))
    }
    fn chain_item_struct(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemStruct,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Struct(t))
    }
    fn chain_item_trait(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemTrait,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Trait(t))
    }
    fn chain_item_traitalias(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::TraitAlias(t))
    }
    fn chain_item_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemType,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Type(t))
    }
    fn chain_item_union(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemUnion,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Union(t))
    }
    fn chain_item_use(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemUse,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Use(t))
    }
    fn chain_item_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Item, Self::Err> {
        Ok(syn::Item::Verbatim(t))
    }
    fn chain_itemconst(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemConst,
    ) -> Result<syn::ItemConst, Self::Err> {
        Ok(t)
    }
    fn chain_itemenum(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemEnum,
    ) -> Result<syn::ItemEnum, Self::Err> {
        Ok(t)
    }
    fn chain_itemexterncrate(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemExternCrate,
    ) -> Result<syn::ItemExternCrate, Self::Err> {
        Ok(t)
    }
    fn chain_itemfn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemFn,
    ) -> Result<syn::ItemFn, Self::Err> {
        Ok(t)
    }
    fn chain_itemforeignmod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemForeignMod,
    ) -> Result<syn::ItemForeignMod, Self::Err> {
        Ok(t)
    }
    fn chain_itemimpl(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemImpl,
    ) -> Result<syn::ItemImpl, Self::Err> {
        Ok(t)
    }
    fn chain_itemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMacro,
    ) -> Result<syn::ItemMacro, Self::Err> {
        Ok(t)
    }
    fn chain_itemmacro2(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMacro2,
    ) -> Result<syn::ItemMacro2, Self::Err> {
        Ok(t)
    }
    fn chain_itemmod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemMod,
    ) -> Result<syn::ItemMod, Self::Err> {
        Ok(t)
    }
    fn chain_itemstatic(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemStatic,
    ) -> Result<syn::ItemStatic, Self::Err> {
        Ok(t)
    }
    fn chain_itemstruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemStruct,
    ) -> Result<syn::ItemStruct, Self::Err> {
        Ok(t)
    }
    fn chain_itemtrait(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemTrait,
    ) -> Result<syn::ItemTrait, Self::Err> {
        Ok(t)
    }
    fn chain_itemtraitalias(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::ItemTraitAlias, Self::Err> {
        Ok(t)
    }
    fn chain_itemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemType,
    ) -> Result<syn::ItemType, Self::Err> {
        Ok(t)
    }
    fn chain_itemunion(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemUnion,
    ) -> Result<syn::ItemUnion, Self::Err> {
        Ok(t)
    }
    fn chain_itemuse(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemUse,
    ) -> Result<syn::ItemUse, Self::Err> {
        Ok(t)
    }
    fn chain_label(
        &mut self,
        c: &mut Self::Input,
        t: syn::Label,
    ) -> Result<syn::Label, Self::Err> {
        Ok(t)
    }
    fn chain_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lifetime,
    ) -> Result<syn::Lifetime, Self::Err> {
        Ok(t)
    }
    fn chain_lifetimedef(
        &mut self,
        c: &mut Self::Input,
        t: syn::LifetimeDef,
    ) -> Result<syn::LifetimeDef, Self::Err> {
        Ok(t)
    }
    fn chain_lit(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lit,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(t)
    }
    fn chain_lit_str(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitStr,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Str(t))
    }
    fn chain_lit_bytestr(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitByteStr,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::ByteStr(t))
    }
    fn chain_lit_byte(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitByte,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Byte(t))
    }
    fn chain_lit_char(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitChar,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Char(t))
    }
    fn chain_lit_int(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitInt,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Int(t))
    }
    fn chain_lit_float(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitFloat,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Float(t))
    }
    fn chain_lit_bool(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitBool,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Bool(t))
    }
    fn chain_lit_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::Literal,
    ) -> Result<syn::Lit, Self::Err> {
        Ok(syn::Lit::Verbatim(t))
    }
    fn chain_litbool(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitBool,
    ) -> Result<syn::LitBool, Self::Err> {
        Ok(t)
    }
    fn chain_litbyte(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitByte,
    ) -> Result<syn::LitByte, Self::Err> {
        Ok(t)
    }
    fn chain_litbytestr(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitByteStr,
    ) -> Result<syn::LitByteStr, Self::Err> {
        Ok(t)
    }
    fn chain_litchar(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitChar,
    ) -> Result<syn::LitChar, Self::Err> {
        Ok(t)
    }
    fn chain_litfloat(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitFloat,
    ) -> Result<syn::LitFloat, Self::Err> {
        Ok(t)
    }
    fn chain_litint(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitInt,
    ) -> Result<syn::LitInt, Self::Err> {
        Ok(t)
    }
    fn chain_litstr(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitStr,
    ) -> Result<syn::LitStr, Self::Err> {
        Ok(t)
    }
    fn chain_local(
        &mut self,
        c: &mut Self::Input,
        t: syn::Local,
    ) -> Result<syn::Local, Self::Err> {
        Ok(t)
    }
    fn chain_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::Macro,
    ) -> Result<syn::Macro, Self::Err> {
        Ok(t)
    }
    fn chain_macrodelimiter(
        &mut self,
        c: &mut Self::Input,
        t: syn::MacroDelimiter,
    ) -> Result<syn::MacroDelimiter, Self::Err> {
        Ok(t)
    }
    fn chain_macrodelimiter_paren(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Paren,
    ) -> Result<syn::MacroDelimiter, Self::Err> {
        Ok(syn::MacroDelimiter::Paren(t))
    }
    fn chain_macrodelimiter_brace(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Brace,
    ) -> Result<syn::MacroDelimiter, Self::Err> {
        Ok(syn::MacroDelimiter::Brace(t))
    }
    fn chain_macrodelimiter_bracket(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Bracket,
    ) -> Result<syn::MacroDelimiter, Self::Err> {
        Ok(syn::MacroDelimiter::Bracket(t))
    }
    fn chain_member(
        &mut self,
        c: &mut Self::Input,
        t: syn::Member,
    ) -> Result<syn::Member, Self::Err> {
        Ok(t)
    }
    fn chain_member_named(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::Ident,
    ) -> Result<syn::Member, Self::Err> {
        Ok(syn::Member::Named(t))
    }
    fn chain_member_unnamed(
        &mut self,
        c: &mut Self::Input,
        t: syn::Index,
    ) -> Result<syn::Member, Self::Err> {
        Ok(syn::Member::Unnamed(t))
    }
    fn chain_meta(
        &mut self,
        c: &mut Self::Input,
        t: syn::Meta,
    ) -> Result<syn::Meta, Self::Err> {
        Ok(t)
    }
    fn chain_meta_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::Path,
    ) -> Result<syn::Meta, Self::Err> {
        Ok(syn::Meta::Path(t))
    }
    fn chain_meta_list(
        &mut self,
        c: &mut Self::Input,
        t: syn::MetaList,
    ) -> Result<syn::Meta, Self::Err> {
        Ok(syn::Meta::List(t))
    }
    fn chain_meta_namevalue(
        &mut self,
        c: &mut Self::Input,
        t: syn::MetaNameValue,
    ) -> Result<syn::Meta, Self::Err> {
        Ok(syn::Meta::NameValue(t))
    }
    fn chain_metalist(
        &mut self,
        c: &mut Self::Input,
        t: syn::MetaList,
    ) -> Result<syn::MetaList, Self::Err> {
        Ok(t)
    }
    fn chain_metanamevalue(
        &mut self,
        c: &mut Self::Input,
        t: syn::MetaNameValue,
    ) -> Result<syn::MetaNameValue, Self::Err> {
        Ok(t)
    }
    fn chain_methodturbofish(
        &mut self,
        c: &mut Self::Input,
        t: syn::MethodTurbofish,
    ) -> Result<syn::MethodTurbofish, Self::Err> {
        Ok(t)
    }
    fn chain_nestedmeta(
        &mut self,
        c: &mut Self::Input,
        t: syn::NestedMeta,
    ) -> Result<syn::NestedMeta, Self::Err> {
        Ok(t)
    }
    fn chain_nestedmeta_meta(
        &mut self,
        c: &mut Self::Input,
        t: syn::Meta,
    ) -> Result<syn::NestedMeta, Self::Err> {
        Ok(syn::NestedMeta::Meta(t))
    }
    fn chain_nestedmeta_lit(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lit,
    ) -> Result<syn::NestedMeta, Self::Err> {
        Ok(syn::NestedMeta::Lit(t))
    }
    fn chain_parenthesizedgenericarguments(
        &mut self,
        c: &mut Self::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::ParenthesizedGenericArguments, Self::Err> {
        Ok(t)
    }
    fn chain_pat(
        &mut self,
        c: &mut Self::Input,
        t: syn::Pat,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(t)
    }
    fn chain_pat_box(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatBox,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Box(t))
    }
    fn chain_pat_ident(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatIdent,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Ident(t))
    }
    fn chain_pat_lit(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatLit,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Lit(t))
    }
    fn chain_pat_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatMacro,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Macro(t))
    }
    fn chain_pat_or(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatOr,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Or(t))
    }
    fn chain_pat_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatPath,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Path(t))
    }
    fn chain_pat_range(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatRange,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Range(t))
    }
    fn chain_pat_reference(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatReference,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Reference(t))
    }
    fn chain_pat_rest(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatRest,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Rest(t))
    }
    fn chain_pat_slice(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatSlice,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Slice(t))
    }
    fn chain_pat_struct(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatStruct,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Struct(t))
    }
    fn chain_pat_tuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatTuple,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Tuple(t))
    }
    fn chain_pat_tuplestruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatTupleStruct,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::TupleStruct(t))
    }
    fn chain_pat_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatType,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Type(t))
    }
    fn chain_pat_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Verbatim(t))
    }
    fn chain_pat_wild(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatWild,
    ) -> Result<syn::Pat, Self::Err> {
        Ok(syn::Pat::Wild(t))
    }
    fn chain_patbox(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatBox,
    ) -> Result<syn::PatBox, Self::Err> {
        Ok(t)
    }
    fn chain_patident(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatIdent,
    ) -> Result<syn::PatIdent, Self::Err> {
        Ok(t)
    }
    fn chain_patlit(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatLit,
    ) -> Result<syn::PatLit, Self::Err> {
        Ok(t)
    }
    fn chain_patmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatMacro,
    ) -> Result<syn::PatMacro, Self::Err> {
        Ok(t)
    }
    fn chain_pator(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatOr,
    ) -> Result<syn::PatOr, Self::Err> {
        Ok(t)
    }
    fn chain_patpath(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatPath,
    ) -> Result<syn::PatPath, Self::Err> {
        Ok(t)
    }
    fn chain_patrange(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatRange,
    ) -> Result<syn::PatRange, Self::Err> {
        Ok(t)
    }
    fn chain_patreference(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatReference,
    ) -> Result<syn::PatReference, Self::Err> {
        Ok(t)
    }
    fn chain_patrest(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatRest,
    ) -> Result<syn::PatRest, Self::Err> {
        Ok(t)
    }
    fn chain_patslice(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatSlice,
    ) -> Result<syn::PatSlice, Self::Err> {
        Ok(t)
    }
    fn chain_patstruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatStruct,
    ) -> Result<syn::PatStruct, Self::Err> {
        Ok(t)
    }
    fn chain_pattuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatTuple,
    ) -> Result<syn::PatTuple, Self::Err> {
        Ok(t)
    }
    fn chain_pattuplestruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatTupleStruct,
    ) -> Result<syn::PatTupleStruct, Self::Err> {
        Ok(t)
    }
    fn chain_pattype(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatType,
    ) -> Result<syn::PatType, Self::Err> {
        Ok(t)
    }
    fn chain_patwild(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatWild,
    ) -> Result<syn::PatWild, Self::Err> {
        Ok(t)
    }
    fn chain_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::Path,
    ) -> Result<syn::Path, Self::Err> {
        Ok(t)
    }
    fn chain_patharguments(
        &mut self,
        c: &mut Self::Input,
        t: syn::PathArguments,
    ) -> Result<syn::PathArguments, Self::Err> {
        Ok(t)
    }
    fn chain_patharguments_anglebracketed(
        &mut self,
        c: &mut Self::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::PathArguments, Self::Err> {
        Ok(syn::PathArguments::AngleBracketed(t))
    }
    fn chain_patharguments_parenthesized(
        &mut self,
        c: &mut Self::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::PathArguments, Self::Err> {
        Ok(syn::PathArguments::Parenthesized(t))
    }
    fn chain_pathsegment(
        &mut self,
        c: &mut Self::Input,
        t: syn::PathSegment,
    ) -> Result<syn::PathSegment, Self::Err> {
        Ok(t)
    }
    fn chain_predicateeq(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateEq,
    ) -> Result<syn::PredicateEq, Self::Err> {
        Ok(t)
    }
    fn chain_predicatelifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateLifetime,
    ) -> Result<syn::PredicateLifetime, Self::Err> {
        Ok(t)
    }
    fn chain_predicatetype(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateType,
    ) -> Result<syn::PredicateType, Self::Err> {
        Ok(t)
    }
    fn chain_qself(
        &mut self,
        c: &mut Self::Input,
        t: syn::QSelf,
    ) -> Result<syn::QSelf, Self::Err> {
        Ok(t)
    }
    fn chain_rangelimits(
        &mut self,
        c: &mut Self::Input,
        t: syn::RangeLimits,
    ) -> Result<syn::RangeLimits, Self::Err> {
        Ok(t)
    }
    fn chain_rangelimits_halfopen(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Dot2,
    ) -> Result<syn::RangeLimits, Self::Err> {
        Ok(syn::RangeLimits::HalfOpen(t))
    }
    fn chain_rangelimits_closed(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::DotDotEq,
    ) -> Result<syn::RangeLimits, Self::Err> {
        Ok(syn::RangeLimits::Closed(t))
    }
    fn chain_receiver(
        &mut self,
        c: &mut Self::Input,
        t: syn::Receiver,
    ) -> Result<syn::Receiver, Self::Err> {
        Ok(t)
    }
    fn chain_returntype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ReturnType,
    ) -> Result<syn::ReturnType, Self::Err> {
        Ok(t)
    }
    fn chain_returntype_type(
        &mut self,
        c: &mut Self::Input,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<syn::ReturnType, Self::Err> {
        Ok(syn::ReturnType::Type(t.0, t.1))
    }
    fn chain_signature(
        &mut self,
        c: &mut Self::Input,
        t: syn::Signature,
    ) -> Result<syn::Signature, Self::Err> {
        Ok(t)
    }
    fn chain_stmt(
        &mut self,
        c: &mut Self::Input,
        t: syn::Stmt,
    ) -> Result<syn::Stmt, Self::Err> {
        Ok(t)
    }
    fn chain_stmt_local(
        &mut self,
        c: &mut Self::Input,
        t: syn::Local,
    ) -> Result<syn::Stmt, Self::Err> {
        Ok(syn::Stmt::Local(t))
    }
    fn chain_stmt_item(
        &mut self,
        c: &mut Self::Input,
        t: syn::Item,
    ) -> Result<syn::Stmt, Self::Err> {
        Ok(syn::Stmt::Item(t))
    }
    fn chain_stmt_expr(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> Result<syn::Stmt, Self::Err> {
        Ok(syn::Stmt::Expr(t))
    }
    fn chain_stmt_semi(
        &mut self,
        c: &mut Self::Input,
        t: (syn::Expr, syn::token::Semi),
    ) -> Result<syn::Stmt, Self::Err> {
        Ok(syn::Stmt::Semi(t.0, t.1))
    }
    fn chain_traitbound(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitBound,
    ) -> Result<syn::TraitBound, Self::Err> {
        Ok(t)
    }
    fn chain_traitboundmodifier(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitBoundModifier,
    ) -> Result<syn::TraitBoundModifier, Self::Err> {
        Ok(t)
    }
    fn chain_traitboundmodifier_maybe(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Question,
    ) -> Result<syn::TraitBoundModifier, Self::Err> {
        Ok(syn::TraitBoundModifier::Maybe(t))
    }
    fn chain_traititem(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItem,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(t)
    }
    fn chain_traititem_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(syn::TraitItem::Const(t))
    }
    fn chain_traititem_method(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(syn::TraitItem::Method(t))
    }
    fn chain_traititem_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(syn::TraitItem::Type(t))
    }
    fn chain_traititem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(syn::TraitItem::Macro(t))
    }
    fn chain_traititem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::TraitItem, Self::Err> {
        Ok(syn::TraitItem::Verbatim(t))
    }
    fn chain_traititemconst(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItemConst, Self::Err> {
        Ok(t)
    }
    fn chain_traititemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItemMacro, Self::Err> {
        Ok(t)
    }
    fn chain_traititemmethod(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItemMethod, Self::Err> {
        Ok(t)
    }
    fn chain_traititemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItemType, Self::Err> {
        Ok(t)
    }
    fn chain_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::Type,
    ) -> Result<syn::Type, Self::Err> {
        Ok(t)
    }
    fn chain_type_array(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeArray,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Array(t))
    }
    fn chain_type_barefn(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeBareFn,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::BareFn(t))
    }
    fn chain_type_group(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeGroup,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Group(t))
    }
    fn chain_type_impltrait(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeImplTrait,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::ImplTrait(t))
    }
    fn chain_type_infer(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeInfer,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Infer(t))
    }
    fn chain_type_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeMacro,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Macro(t))
    }
    fn chain_type_never(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeNever,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Never(t))
    }
    fn chain_type_paren(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParen,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Paren(t))
    }
    fn chain_type_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypePath,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Path(t))
    }
    fn chain_type_ptr(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypePtr,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Ptr(t))
    }
    fn chain_type_reference(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeReference,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Reference(t))
    }
    fn chain_type_slice(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeSlice,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Slice(t))
    }
    fn chain_type_traitobject(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTraitObject,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::TraitObject(t))
    }
    fn chain_type_tuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTuple,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Tuple(t))
    }
    fn chain_type_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Type, Self::Err> {
        Ok(syn::Type::Verbatim(t))
    }
    fn chain_typearray(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeArray,
    ) -> Result<syn::TypeArray, Self::Err> {
        Ok(t)
    }
    fn chain_typebarefn(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeBareFn,
    ) -> Result<syn::TypeBareFn, Self::Err> {
        Ok(t)
    }
    fn chain_typegroup(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeGroup,
    ) -> Result<syn::TypeGroup, Self::Err> {
        Ok(t)
    }
    fn chain_typeimpltrait(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeImplTrait,
    ) -> Result<syn::TypeImplTrait, Self::Err> {
        Ok(t)
    }
    fn chain_typeinfer(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeInfer,
    ) -> Result<syn::TypeInfer, Self::Err> {
        Ok(t)
    }
    fn chain_typemacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeMacro,
    ) -> Result<syn::TypeMacro, Self::Err> {
        Ok(t)
    }
    fn chain_typenever(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeNever,
    ) -> Result<syn::TypeNever, Self::Err> {
        Ok(t)
    }
    fn chain_typeparam(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParam,
    ) -> Result<syn::TypeParam, Self::Err> {
        Ok(t)
    }
    fn chain_typeparambound(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParamBound,
    ) -> Result<syn::TypeParamBound, Self::Err> {
        Ok(t)
    }
    fn chain_typeparambound_trait(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitBound,
    ) -> Result<syn::TypeParamBound, Self::Err> {
        Ok(syn::TypeParamBound::Trait(t))
    }
    fn chain_typeparambound_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lifetime,
    ) -> Result<syn::TypeParamBound, Self::Err> {
        Ok(syn::TypeParamBound::Lifetime(t))
    }
    fn chain_typeparen(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParen,
    ) -> Result<syn::TypeParen, Self::Err> {
        Ok(t)
    }
    fn chain_typepath(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypePath,
    ) -> Result<syn::TypePath, Self::Err> {
        Ok(t)
    }
    fn chain_typeptr(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypePtr,
    ) -> Result<syn::TypePtr, Self::Err> {
        Ok(t)
    }
    fn chain_typereference(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeReference,
    ) -> Result<syn::TypeReference, Self::Err> {
        Ok(t)
    }
    fn chain_typeslice(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeSlice,
    ) -> Result<syn::TypeSlice, Self::Err> {
        Ok(t)
    }
    fn chain_typetraitobject(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTraitObject,
    ) -> Result<syn::TypeTraitObject, Self::Err> {
        Ok(t)
    }
    fn chain_typetuple(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTuple,
    ) -> Result<syn::TypeTuple, Self::Err> {
        Ok(t)
    }
    fn chain_unop(
        &mut self,
        c: &mut Self::Input,
        t: syn::UnOp,
    ) -> Result<syn::UnOp, Self::Err> {
        Ok(t)
    }
    fn chain_unop_deref(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Star,
    ) -> Result<syn::UnOp, Self::Err> {
        Ok(syn::UnOp::Deref(t))
    }
    fn chain_unop_not(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Bang,
    ) -> Result<syn::UnOp, Self::Err> {
        Ok(syn::UnOp::Not(t))
    }
    fn chain_unop_neg(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Sub,
    ) -> Result<syn::UnOp, Self::Err> {
        Ok(syn::UnOp::Neg(t))
    }
    fn chain_useglob(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseGlob,
    ) -> Result<syn::UseGlob, Self::Err> {
        Ok(t)
    }
    fn chain_usegroup(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseGroup,
    ) -> Result<syn::UseGroup, Self::Err> {
        Ok(t)
    }
    fn chain_usename(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseName,
    ) -> Result<syn::UseName, Self::Err> {
        Ok(t)
    }
    fn chain_usepath(
        &mut self,
        c: &mut Self::Input,
        t: syn::UsePath,
    ) -> Result<syn::UsePath, Self::Err> {
        Ok(t)
    }
    fn chain_userename(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseRename,
    ) -> Result<syn::UseRename, Self::Err> {
        Ok(t)
    }
    fn chain_usetree(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseTree,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(t)
    }
    fn chain_usetree_path(
        &mut self,
        c: &mut Self::Input,
        t: syn::UsePath,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(syn::UseTree::Path(t))
    }
    fn chain_usetree_name(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseName,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(syn::UseTree::Name(t))
    }
    fn chain_usetree_rename(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseRename,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(syn::UseTree::Rename(t))
    }
    fn chain_usetree_glob(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseGlob,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(syn::UseTree::Glob(t))
    }
    fn chain_usetree_group(
        &mut self,
        c: &mut Self::Input,
        t: syn::UseGroup,
    ) -> Result<syn::UseTree, Self::Err> {
        Ok(syn::UseTree::Group(t))
    }
    fn chain_variadic(
        &mut self,
        c: &mut Self::Input,
        t: syn::Variadic,
    ) -> Result<syn::Variadic, Self::Err> {
        Ok(t)
    }
    fn chain_variant(
        &mut self,
        c: &mut Self::Input,
        t: syn::Variant,
    ) -> Result<syn::Variant, Self::Err> {
        Ok(t)
    }
    fn chain_viscrate(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisCrate,
    ) -> Result<syn::VisCrate, Self::Err> {
        Ok(t)
    }
    fn chain_vispublic(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisPublic,
    ) -> Result<syn::VisPublic, Self::Err> {
        Ok(t)
    }
    fn chain_visrestricted(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisRestricted,
    ) -> Result<syn::VisRestricted, Self::Err> {
        Ok(t)
    }
    fn chain_visibility(
        &mut self,
        c: &mut Self::Input,
        t: syn::Visibility,
    ) -> Result<syn::Visibility, Self::Err> {
        Ok(t)
    }
    fn chain_visibility_public(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisPublic,
    ) -> Result<syn::Visibility, Self::Err> {
        Ok(syn::Visibility::Public(t))
    }
    fn chain_visibility_crate(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisCrate,
    ) -> Result<syn::Visibility, Self::Err> {
        Ok(syn::Visibility::Crate(t))
    }
    fn chain_visibility_restricted(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisRestricted,
    ) -> Result<syn::Visibility, Self::Err> {
        Ok(syn::Visibility::Restricted(t))
    }
    fn chain_whereclause(
        &mut self,
        c: &mut Self::Input,
        t: syn::WhereClause,
    ) -> Result<syn::WhereClause, Self::Err> {
        Ok(t)
    }
    fn chain_wherepredicate(
        &mut self,
        c: &mut Self::Input,
        t: syn::WherePredicate,
    ) -> Result<syn::WherePredicate, Self::Err> {
        Ok(t)
    }
    fn chain_wherepredicate_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateType,
    ) -> Result<syn::WherePredicate, Self::Err> {
        Ok(syn::WherePredicate::Type(t))
    }
    fn chain_wherepredicate_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateLifetime,
    ) -> Result<syn::WherePredicate, Self::Err> {
        Ok(syn::WherePredicate::Lifetime(t))
    }
    fn chain_wherepredicate_eq(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateEq,
    ) -> Result<syn::WherePredicate, Self::Err> {
        Ok(syn::WherePredicate::Eq(t))
    }
}
