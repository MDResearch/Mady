// codegen file by version 0.1.0
// don't edit this

/// fold with Result<?, Self::Error> trait
pub trait RFold {
    type Error;
    fn rfold_abi(&mut self, t: syn::Abi) -> Result<syn::Abi, Self::Error> {
        rfold_abi(self, t)
    }
    fn rfold_anglebracketedgenericarguments(
        &mut self,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::AngleBracketedGenericArguments, Self::Error> {
        rfold_anglebracketedgenericarguments(self, t)
    }
    fn rfold_arm(&mut self, t: syn::Arm) -> Result<syn::Arm, Self::Error> {
        rfold_arm(self, t)
    }
    fn rfold_attrstyle(&mut self, t: syn::AttrStyle) -> Result<syn::AttrStyle, Self::Error> {
        rfold_attrstyle(self, t)
    }
    fn rfold_attrstyle_inner(
        &mut self,
        t: syn::token::Bang,
    ) -> Result<syn::AttrStyle, Self::Error> {
        rfold_attrstyle_inner(self, t)
    }
    fn rfold_attribute(&mut self, t: syn::Attribute) -> Result<syn::Attribute, Self::Error> {
        rfold_attribute(self, t)
    }
    fn rfold_barefnarg(&mut self, t: syn::BareFnArg) -> Result<syn::BareFnArg, Self::Error> {
        rfold_barefnarg(self, t)
    }
    fn rfold_binop(&mut self, t: syn::BinOp) -> Result<syn::BinOp, Self::Error> {
        rfold_binop(self, t)
    }
    fn rfold_binop_add(&mut self, t: syn::token::Add) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_add(self, t)
    }
    fn rfold_binop_sub(&mut self, t: syn::token::Sub) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_sub(self, t)
    }
    fn rfold_binop_mul(&mut self, t: syn::token::Star) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_mul(self, t)
    }
    fn rfold_binop_div(&mut self, t: syn::token::Div) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_div(self, t)
    }
    fn rfold_binop_rem(&mut self, t: syn::token::Rem) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_rem(self, t)
    }
    fn rfold_binop_and(&mut self, t: syn::token::AndAnd) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_and(self, t)
    }
    fn rfold_binop_or(&mut self, t: syn::token::OrOr) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_or(self, t)
    }
    fn rfold_binop_bitxor(&mut self, t: syn::token::Caret) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitxor(self, t)
    }
    fn rfold_binop_bitand(&mut self, t: syn::token::And) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitand(self, t)
    }
    fn rfold_binop_bitor(&mut self, t: syn::token::Or) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitor(self, t)
    }
    fn rfold_binop_shl(&mut self, t: syn::token::Shl) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_shl(self, t)
    }
    fn rfold_binop_shr(&mut self, t: syn::token::Shr) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_shr(self, t)
    }
    fn rfold_binop_eq(&mut self, t: syn::token::EqEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_eq(self, t)
    }
    fn rfold_binop_lt(&mut self, t: syn::token::Lt) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_lt(self, t)
    }
    fn rfold_binop_le(&mut self, t: syn::token::Le) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_le(self, t)
    }
    fn rfold_binop_ne(&mut self, t: syn::token::Ne) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_ne(self, t)
    }
    fn rfold_binop_ge(&mut self, t: syn::token::Ge) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_ge(self, t)
    }
    fn rfold_binop_gt(&mut self, t: syn::token::Gt) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_gt(self, t)
    }
    fn rfold_binop_addeq(&mut self, t: syn::token::AddEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_addeq(self, t)
    }
    fn rfold_binop_subeq(&mut self, t: syn::token::SubEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_subeq(self, t)
    }
    fn rfold_binop_muleq(&mut self, t: syn::token::MulEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_muleq(self, t)
    }
    fn rfold_binop_diveq(&mut self, t: syn::token::DivEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_diveq(self, t)
    }
    fn rfold_binop_remeq(&mut self, t: syn::token::RemEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_remeq(self, t)
    }
    fn rfold_binop_bitxoreq(&mut self, t: syn::token::CaretEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitxoreq(self, t)
    }
    fn rfold_binop_bitandeq(&mut self, t: syn::token::AndEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitandeq(self, t)
    }
    fn rfold_binop_bitoreq(&mut self, t: syn::token::OrEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_bitoreq(self, t)
    }
    fn rfold_binop_shleq(&mut self, t: syn::token::ShlEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_shleq(self, t)
    }
    fn rfold_binop_shreq(&mut self, t: syn::token::ShrEq) -> Result<syn::BinOp, Self::Error> {
        rfold_binop_shreq(self, t)
    }
    fn rfold_binding(&mut self, t: syn::Binding) -> Result<syn::Binding, Self::Error> {
        rfold_binding(self, t)
    }
    fn rfold_block(&mut self, t: syn::Block) -> Result<syn::Block, Self::Error> {
        rfold_block(self, t)
    }
    fn rfold_boundlifetimes(
        &mut self,
        t: syn::BoundLifetimes,
    ) -> Result<syn::BoundLifetimes, Self::Error> {
        rfold_boundlifetimes(self, t)
    }
    fn rfold_constparam(&mut self, t: syn::ConstParam) -> Result<syn::ConstParam, Self::Error> {
        rfold_constparam(self, t)
    }
    fn rfold_constraint(&mut self, t: syn::Constraint) -> Result<syn::Constraint, Self::Error> {
        rfold_constraint(self, t)
    }
    fn rfold_data(&mut self, t: syn::Data) -> Result<syn::Data, Self::Error> {
        rfold_data(self, t)
    }
    fn rfold_data_struct(&mut self, t: syn::DataStruct) -> Result<syn::Data, Self::Error> {
        rfold_data_struct(self, t)
    }
    fn rfold_data_enum(&mut self, t: syn::DataEnum) -> Result<syn::Data, Self::Error> {
        rfold_data_enum(self, t)
    }
    fn rfold_data_union(&mut self, t: syn::DataUnion) -> Result<syn::Data, Self::Error> {
        rfold_data_union(self, t)
    }
    fn rfold_dataenum(&mut self, t: syn::DataEnum) -> Result<syn::DataEnum, Self::Error> {
        rfold_dataenum(self, t)
    }
    fn rfold_datastruct(&mut self, t: syn::DataStruct) -> Result<syn::DataStruct, Self::Error> {
        rfold_datastruct(self, t)
    }
    fn rfold_dataunion(&mut self, t: syn::DataUnion) -> Result<syn::DataUnion, Self::Error> {
        rfold_dataunion(self, t)
    }
    fn rfold_deriveinput(&mut self, t: syn::DeriveInput) -> Result<syn::DeriveInput, Self::Error> {
        rfold_deriveinput(self, t)
    }
    fn rfold_expr(&mut self, t: syn::Expr) -> Result<syn::Expr, Self::Error> {
        rfold_expr(self, t)
    }
    fn rfold_expr_array(&mut self, t: syn::ExprArray) -> Result<syn::Expr, Self::Error> {
        rfold_expr_array(self, t)
    }
    fn rfold_expr_assign(&mut self, t: syn::ExprAssign) -> Result<syn::Expr, Self::Error> {
        rfold_expr_assign(self, t)
    }
    fn rfold_expr_assignop(&mut self, t: syn::ExprAssignOp) -> Result<syn::Expr, Self::Error> {
        rfold_expr_assignop(self, t)
    }
    fn rfold_expr_async(&mut self, t: syn::ExprAsync) -> Result<syn::Expr, Self::Error> {
        rfold_expr_async(self, t)
    }
    fn rfold_expr_await(&mut self, t: syn::ExprAwait) -> Result<syn::Expr, Self::Error> {
        rfold_expr_await(self, t)
    }
    fn rfold_expr_binary(&mut self, t: syn::ExprBinary) -> Result<syn::Expr, Self::Error> {
        rfold_expr_binary(self, t)
    }
    fn rfold_expr_block(&mut self, t: syn::ExprBlock) -> Result<syn::Expr, Self::Error> {
        rfold_expr_block(self, t)
    }
    fn rfold_expr_box(&mut self, t: syn::ExprBox) -> Result<syn::Expr, Self::Error> {
        rfold_expr_box(self, t)
    }
    fn rfold_expr_break(&mut self, t: syn::ExprBreak) -> Result<syn::Expr, Self::Error> {
        rfold_expr_break(self, t)
    }
    fn rfold_expr_call(&mut self, t: syn::ExprCall) -> Result<syn::Expr, Self::Error> {
        rfold_expr_call(self, t)
    }
    fn rfold_expr_cast(&mut self, t: syn::ExprCast) -> Result<syn::Expr, Self::Error> {
        rfold_expr_cast(self, t)
    }
    fn rfold_expr_closure(&mut self, t: syn::ExprClosure) -> Result<syn::Expr, Self::Error> {
        rfold_expr_closure(self, t)
    }
    fn rfold_expr_continue(&mut self, t: syn::ExprContinue) -> Result<syn::Expr, Self::Error> {
        rfold_expr_continue(self, t)
    }
    fn rfold_expr_field(&mut self, t: syn::ExprField) -> Result<syn::Expr, Self::Error> {
        rfold_expr_field(self, t)
    }
    fn rfold_expr_forloop(&mut self, t: syn::ExprForLoop) -> Result<syn::Expr, Self::Error> {
        rfold_expr_forloop(self, t)
    }
    fn rfold_expr_group(&mut self, t: syn::ExprGroup) -> Result<syn::Expr, Self::Error> {
        rfold_expr_group(self, t)
    }
    fn rfold_expr_if(&mut self, t: syn::ExprIf) -> Result<syn::Expr, Self::Error> {
        rfold_expr_if(self, t)
    }
    fn rfold_expr_index(&mut self, t: syn::ExprIndex) -> Result<syn::Expr, Self::Error> {
        rfold_expr_index(self, t)
    }
    fn rfold_expr_let(&mut self, t: syn::ExprLet) -> Result<syn::Expr, Self::Error> {
        rfold_expr_let(self, t)
    }
    fn rfold_expr_lit(&mut self, t: syn::ExprLit) -> Result<syn::Expr, Self::Error> {
        rfold_expr_lit(self, t)
    }
    fn rfold_expr_loop(&mut self, t: syn::ExprLoop) -> Result<syn::Expr, Self::Error> {
        rfold_expr_loop(self, t)
    }
    fn rfold_expr_macro(&mut self, t: syn::ExprMacro) -> Result<syn::Expr, Self::Error> {
        rfold_expr_macro(self, t)
    }
    fn rfold_expr_match(&mut self, t: syn::ExprMatch) -> Result<syn::Expr, Self::Error> {
        rfold_expr_match(self, t)
    }
    fn rfold_expr_methodcall(&mut self, t: syn::ExprMethodCall) -> Result<syn::Expr, Self::Error> {
        rfold_expr_methodcall(self, t)
    }
    fn rfold_expr_paren(&mut self, t: syn::ExprParen) -> Result<syn::Expr, Self::Error> {
        rfold_expr_paren(self, t)
    }
    fn rfold_expr_path(&mut self, t: syn::ExprPath) -> Result<syn::Expr, Self::Error> {
        rfold_expr_path(self, t)
    }
    fn rfold_expr_range(&mut self, t: syn::ExprRange) -> Result<syn::Expr, Self::Error> {
        rfold_expr_range(self, t)
    }
    fn rfold_expr_reference(&mut self, t: syn::ExprReference) -> Result<syn::Expr, Self::Error> {
        rfold_expr_reference(self, t)
    }
    fn rfold_expr_repeat(&mut self, t: syn::ExprRepeat) -> Result<syn::Expr, Self::Error> {
        rfold_expr_repeat(self, t)
    }
    fn rfold_expr_return(&mut self, t: syn::ExprReturn) -> Result<syn::Expr, Self::Error> {
        rfold_expr_return(self, t)
    }
    fn rfold_expr_struct(&mut self, t: syn::ExprStruct) -> Result<syn::Expr, Self::Error> {
        rfold_expr_struct(self, t)
    }
    fn rfold_expr_try(&mut self, t: syn::ExprTry) -> Result<syn::Expr, Self::Error> {
        rfold_expr_try(self, t)
    }
    fn rfold_expr_tryblock(&mut self, t: syn::ExprTryBlock) -> Result<syn::Expr, Self::Error> {
        rfold_expr_tryblock(self, t)
    }
    fn rfold_expr_tuple(&mut self, t: syn::ExprTuple) -> Result<syn::Expr, Self::Error> {
        rfold_expr_tuple(self, t)
    }
    fn rfold_expr_type(&mut self, t: syn::ExprType) -> Result<syn::Expr, Self::Error> {
        rfold_expr_type(self, t)
    }
    fn rfold_expr_unary(&mut self, t: syn::ExprUnary) -> Result<syn::Expr, Self::Error> {
        rfold_expr_unary(self, t)
    }
    fn rfold_expr_unsafe(&mut self, t: syn::ExprUnsafe) -> Result<syn::Expr, Self::Error> {
        rfold_expr_unsafe(self, t)
    }
    fn rfold_expr_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Expr, Self::Error> {
        rfold_expr_verbatim(self, t)
    }
    fn rfold_expr_while(&mut self, t: syn::ExprWhile) -> Result<syn::Expr, Self::Error> {
        rfold_expr_while(self, t)
    }
    fn rfold_expr_yield(&mut self, t: syn::ExprYield) -> Result<syn::Expr, Self::Error> {
        rfold_expr_yield(self, t)
    }
    fn rfold_exprarray(&mut self, t: syn::ExprArray) -> Result<syn::ExprArray, Self::Error> {
        rfold_exprarray(self, t)
    }
    fn rfold_exprassign(&mut self, t: syn::ExprAssign) -> Result<syn::ExprAssign, Self::Error> {
        rfold_exprassign(self, t)
    }
    fn rfold_exprassignop(
        &mut self,
        t: syn::ExprAssignOp,
    ) -> Result<syn::ExprAssignOp, Self::Error> {
        rfold_exprassignop(self, t)
    }
    fn rfold_exprasync(&mut self, t: syn::ExprAsync) -> Result<syn::ExprAsync, Self::Error> {
        rfold_exprasync(self, t)
    }
    fn rfold_exprawait(&mut self, t: syn::ExprAwait) -> Result<syn::ExprAwait, Self::Error> {
        rfold_exprawait(self, t)
    }
    fn rfold_exprbinary(&mut self, t: syn::ExprBinary) -> Result<syn::ExprBinary, Self::Error> {
        rfold_exprbinary(self, t)
    }
    fn rfold_exprblock(&mut self, t: syn::ExprBlock) -> Result<syn::ExprBlock, Self::Error> {
        rfold_exprblock(self, t)
    }
    fn rfold_exprbox(&mut self, t: syn::ExprBox) -> Result<syn::ExprBox, Self::Error> {
        rfold_exprbox(self, t)
    }
    fn rfold_exprbreak(&mut self, t: syn::ExprBreak) -> Result<syn::ExprBreak, Self::Error> {
        rfold_exprbreak(self, t)
    }
    fn rfold_exprcall(&mut self, t: syn::ExprCall) -> Result<syn::ExprCall, Self::Error> {
        rfold_exprcall(self, t)
    }
    fn rfold_exprcast(&mut self, t: syn::ExprCast) -> Result<syn::ExprCast, Self::Error> {
        rfold_exprcast(self, t)
    }
    fn rfold_exprclosure(&mut self, t: syn::ExprClosure) -> Result<syn::ExprClosure, Self::Error> {
        rfold_exprclosure(self, t)
    }
    fn rfold_exprcontinue(
        &mut self,
        t: syn::ExprContinue,
    ) -> Result<syn::ExprContinue, Self::Error> {
        rfold_exprcontinue(self, t)
    }
    fn rfold_exprfield(&mut self, t: syn::ExprField) -> Result<syn::ExprField, Self::Error> {
        rfold_exprfield(self, t)
    }
    fn rfold_exprforloop(&mut self, t: syn::ExprForLoop) -> Result<syn::ExprForLoop, Self::Error> {
        rfold_exprforloop(self, t)
    }
    fn rfold_exprgroup(&mut self, t: syn::ExprGroup) -> Result<syn::ExprGroup, Self::Error> {
        rfold_exprgroup(self, t)
    }
    fn rfold_exprif(&mut self, t: syn::ExprIf) -> Result<syn::ExprIf, Self::Error> {
        rfold_exprif(self, t)
    }
    fn rfold_exprindex(&mut self, t: syn::ExprIndex) -> Result<syn::ExprIndex, Self::Error> {
        rfold_exprindex(self, t)
    }
    fn rfold_exprlet(&mut self, t: syn::ExprLet) -> Result<syn::ExprLet, Self::Error> {
        rfold_exprlet(self, t)
    }
    fn rfold_exprlit(&mut self, t: syn::ExprLit) -> Result<syn::ExprLit, Self::Error> {
        rfold_exprlit(self, t)
    }
    fn rfold_exprloop(&mut self, t: syn::ExprLoop) -> Result<syn::ExprLoop, Self::Error> {
        rfold_exprloop(self, t)
    }
    fn rfold_exprmacro(&mut self, t: syn::ExprMacro) -> Result<syn::ExprMacro, Self::Error> {
        rfold_exprmacro(self, t)
    }
    fn rfold_exprmatch(&mut self, t: syn::ExprMatch) -> Result<syn::ExprMatch, Self::Error> {
        rfold_exprmatch(self, t)
    }
    fn rfold_exprmethodcall(
        &mut self,
        t: syn::ExprMethodCall,
    ) -> Result<syn::ExprMethodCall, Self::Error> {
        rfold_exprmethodcall(self, t)
    }
    fn rfold_exprparen(&mut self, t: syn::ExprParen) -> Result<syn::ExprParen, Self::Error> {
        rfold_exprparen(self, t)
    }
    fn rfold_exprpath(&mut self, t: syn::ExprPath) -> Result<syn::ExprPath, Self::Error> {
        rfold_exprpath(self, t)
    }
    fn rfold_exprrange(&mut self, t: syn::ExprRange) -> Result<syn::ExprRange, Self::Error> {
        rfold_exprrange(self, t)
    }
    fn rfold_exprreference(
        &mut self,
        t: syn::ExprReference,
    ) -> Result<syn::ExprReference, Self::Error> {
        rfold_exprreference(self, t)
    }
    fn rfold_exprrepeat(&mut self, t: syn::ExprRepeat) -> Result<syn::ExprRepeat, Self::Error> {
        rfold_exprrepeat(self, t)
    }
    fn rfold_exprreturn(&mut self, t: syn::ExprReturn) -> Result<syn::ExprReturn, Self::Error> {
        rfold_exprreturn(self, t)
    }
    fn rfold_exprstruct(&mut self, t: syn::ExprStruct) -> Result<syn::ExprStruct, Self::Error> {
        rfold_exprstruct(self, t)
    }
    fn rfold_exprtry(&mut self, t: syn::ExprTry) -> Result<syn::ExprTry, Self::Error> {
        rfold_exprtry(self, t)
    }
    fn rfold_exprtryblock(
        &mut self,
        t: syn::ExprTryBlock,
    ) -> Result<syn::ExprTryBlock, Self::Error> {
        rfold_exprtryblock(self, t)
    }
    fn rfold_exprtuple(&mut self, t: syn::ExprTuple) -> Result<syn::ExprTuple, Self::Error> {
        rfold_exprtuple(self, t)
    }
    fn rfold_exprtype(&mut self, t: syn::ExprType) -> Result<syn::ExprType, Self::Error> {
        rfold_exprtype(self, t)
    }
    fn rfold_exprunary(&mut self, t: syn::ExprUnary) -> Result<syn::ExprUnary, Self::Error> {
        rfold_exprunary(self, t)
    }
    fn rfold_exprunsafe(&mut self, t: syn::ExprUnsafe) -> Result<syn::ExprUnsafe, Self::Error> {
        rfold_exprunsafe(self, t)
    }
    fn rfold_exprwhile(&mut self, t: syn::ExprWhile) -> Result<syn::ExprWhile, Self::Error> {
        rfold_exprwhile(self, t)
    }
    fn rfold_expryield(&mut self, t: syn::ExprYield) -> Result<syn::ExprYield, Self::Error> {
        rfold_expryield(self, t)
    }
    fn rfold_field(&mut self, t: syn::Field) -> Result<syn::Field, Self::Error> {
        rfold_field(self, t)
    }
    fn rfold_fieldpat(&mut self, t: syn::FieldPat) -> Result<syn::FieldPat, Self::Error> {
        rfold_fieldpat(self, t)
    }
    fn rfold_fieldvalue(&mut self, t: syn::FieldValue) -> Result<syn::FieldValue, Self::Error> {
        rfold_fieldvalue(self, t)
    }
    fn rfold_fields(&mut self, t: syn::Fields) -> Result<syn::Fields, Self::Error> {
        rfold_fields(self, t)
    }
    fn rfold_fields_named(&mut self, t: syn::FieldsNamed) -> Result<syn::Fields, Self::Error> {
        rfold_fields_named(self, t)
    }
    fn rfold_fields_unnamed(&mut self, t: syn::FieldsUnnamed) -> Result<syn::Fields, Self::Error> {
        rfold_fields_unnamed(self, t)
    }
    fn rfold_fieldsnamed(&mut self, t: syn::FieldsNamed) -> Result<syn::FieldsNamed, Self::Error> {
        rfold_fieldsnamed(self, t)
    }
    fn rfold_fieldsunnamed(
        &mut self,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::FieldsUnnamed, Self::Error> {
        rfold_fieldsunnamed(self, t)
    }
    fn rfold_file(&mut self, t: syn::File) -> Result<syn::File, Self::Error> {
        rfold_file(self, t)
    }
    fn rfold_fnarg(&mut self, t: syn::FnArg) -> Result<syn::FnArg, Self::Error> {
        rfold_fnarg(self, t)
    }
    fn rfold_fnarg_receiver(&mut self, t: syn::Receiver) -> Result<syn::FnArg, Self::Error> {
        rfold_fnarg_receiver(self, t)
    }
    fn rfold_fnarg_typed(&mut self, t: syn::PatType) -> Result<syn::FnArg, Self::Error> {
        rfold_fnarg_typed(self, t)
    }
    fn rfold_foreignitem(&mut self, t: syn::ForeignItem) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem(self, t)
    }
    fn rfold_foreignitem_fn(
        &mut self,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem_fn(self, t)
    }
    fn rfold_foreignitem_static(
        &mut self,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem_static(self, t)
    }
    fn rfold_foreignitem_type(
        &mut self,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem_type(self, t)
    }
    fn rfold_foreignitem_macro(
        &mut self,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem_macro(self, t)
    }
    fn rfold_foreignitem_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ForeignItem, Self::Error> {
        rfold_foreignitem_verbatim(self, t)
    }
    fn rfold_foreignitemfn(
        &mut self,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItemFn, Self::Error> {
        rfold_foreignitemfn(self, t)
    }
    fn rfold_foreignitemmacro(
        &mut self,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItemMacro, Self::Error> {
        rfold_foreignitemmacro(self, t)
    }
    fn rfold_foreignitemstatic(
        &mut self,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItemStatic, Self::Error> {
        rfold_foreignitemstatic(self, t)
    }
    fn rfold_foreignitemtype(
        &mut self,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItemType, Self::Error> {
        rfold_foreignitemtype(self, t)
    }
    fn rfold_genericargument(
        &mut self,
        t: syn::GenericArgument,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument(self, t)
    }
    fn rfold_genericargument_lifetime(
        &mut self,
        t: syn::Lifetime,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument_lifetime(self, t)
    }
    fn rfold_genericargument_type(
        &mut self,
        t: syn::Type,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument_type(self, t)
    }
    fn rfold_genericargument_binding(
        &mut self,
        t: syn::Binding,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument_binding(self, t)
    }
    fn rfold_genericargument_constraint(
        &mut self,
        t: syn::Constraint,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument_constraint(self, t)
    }
    fn rfold_genericargument_const(
        &mut self,
        t: syn::Expr,
    ) -> Result<syn::GenericArgument, Self::Error> {
        rfold_genericargument_const(self, t)
    }
    fn rfold_genericmethodargument(
        &mut self,
        t: syn::GenericMethodArgument,
    ) -> Result<syn::GenericMethodArgument, Self::Error> {
        rfold_genericmethodargument(self, t)
    }
    fn rfold_genericmethodargument_type(
        &mut self,
        t: syn::Type,
    ) -> Result<syn::GenericMethodArgument, Self::Error> {
        rfold_genericmethodargument_type(self, t)
    }
    fn rfold_genericmethodargument_const(
        &mut self,
        t: syn::Expr,
    ) -> Result<syn::GenericMethodArgument, Self::Error> {
        rfold_genericmethodargument_const(self, t)
    }
    fn rfold_genericparam(
        &mut self,
        t: syn::GenericParam,
    ) -> Result<syn::GenericParam, Self::Error> {
        rfold_genericparam(self, t)
    }
    fn rfold_genericparam_type(
        &mut self,
        t: syn::TypeParam,
    ) -> Result<syn::GenericParam, Self::Error> {
        rfold_genericparam_type(self, t)
    }
    fn rfold_genericparam_lifetime(
        &mut self,
        t: syn::LifetimeDef,
    ) -> Result<syn::GenericParam, Self::Error> {
        rfold_genericparam_lifetime(self, t)
    }
    fn rfold_genericparam_const(
        &mut self,
        t: syn::ConstParam,
    ) -> Result<syn::GenericParam, Self::Error> {
        rfold_genericparam_const(self, t)
    }
    fn rfold_generics(&mut self, t: syn::Generics) -> Result<syn::Generics, Self::Error> {
        rfold_generics(self, t)
    }
    fn rfold_implitem(&mut self, t: syn::ImplItem) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem(self, t)
    }
    fn rfold_implitem_const(
        &mut self,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem_const(self, t)
    }
    fn rfold_implitem_method(
        &mut self,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem_method(self, t)
    }
    fn rfold_implitem_type(&mut self, t: syn::ImplItemType) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem_type(self, t)
    }
    fn rfold_implitem_macro(
        &mut self,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem_macro(self, t)
    }
    fn rfold_implitem_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ImplItem, Self::Error> {
        rfold_implitem_verbatim(self, t)
    }
    fn rfold_implitemconst(
        &mut self,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItemConst, Self::Error> {
        rfold_implitemconst(self, t)
    }
    fn rfold_implitemmacro(
        &mut self,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItemMacro, Self::Error> {
        rfold_implitemmacro(self, t)
    }
    fn rfold_implitemmethod(
        &mut self,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItemMethod, Self::Error> {
        rfold_implitemmethod(self, t)
    }
    fn rfold_implitemtype(
        &mut self,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItemType, Self::Error> {
        rfold_implitemtype(self, t)
    }
    fn rfold_index(&mut self, t: syn::Index) -> Result<syn::Index, Self::Error> {
        rfold_index(self, t)
    }
    fn rfold_item(&mut self, t: syn::Item) -> Result<syn::Item, Self::Error> {
        rfold_item(self, t)
    }
    fn rfold_item_const(&mut self, t: syn::ItemConst) -> Result<syn::Item, Self::Error> {
        rfold_item_const(self, t)
    }
    fn rfold_item_enum(&mut self, t: syn::ItemEnum) -> Result<syn::Item, Self::Error> {
        rfold_item_enum(self, t)
    }
    fn rfold_item_externcrate(
        &mut self,
        t: syn::ItemExternCrate,
    ) -> Result<syn::Item, Self::Error> {
        rfold_item_externcrate(self, t)
    }
    fn rfold_item_fn(&mut self, t: syn::ItemFn) -> Result<syn::Item, Self::Error> {
        rfold_item_fn(self, t)
    }
    fn rfold_item_foreignmod(&mut self, t: syn::ItemForeignMod) -> Result<syn::Item, Self::Error> {
        rfold_item_foreignmod(self, t)
    }
    fn rfold_item_impl(&mut self, t: syn::ItemImpl) -> Result<syn::Item, Self::Error> {
        rfold_item_impl(self, t)
    }
    fn rfold_item_macro(&mut self, t: syn::ItemMacro) -> Result<syn::Item, Self::Error> {
        rfold_item_macro(self, t)
    }
    fn rfold_item_macro2(&mut self, t: syn::ItemMacro2) -> Result<syn::Item, Self::Error> {
        rfold_item_macro2(self, t)
    }
    fn rfold_item_mod(&mut self, t: syn::ItemMod) -> Result<syn::Item, Self::Error> {
        rfold_item_mod(self, t)
    }
    fn rfold_item_static(&mut self, t: syn::ItemStatic) -> Result<syn::Item, Self::Error> {
        rfold_item_static(self, t)
    }
    fn rfold_item_struct(&mut self, t: syn::ItemStruct) -> Result<syn::Item, Self::Error> {
        rfold_item_struct(self, t)
    }
    fn rfold_item_trait(&mut self, t: syn::ItemTrait) -> Result<syn::Item, Self::Error> {
        rfold_item_trait(self, t)
    }
    fn rfold_item_traitalias(&mut self, t: syn::ItemTraitAlias) -> Result<syn::Item, Self::Error> {
        rfold_item_traitalias(self, t)
    }
    fn rfold_item_type(&mut self, t: syn::ItemType) -> Result<syn::Item, Self::Error> {
        rfold_item_type(self, t)
    }
    fn rfold_item_union(&mut self, t: syn::ItemUnion) -> Result<syn::Item, Self::Error> {
        rfold_item_union(self, t)
    }
    fn rfold_item_use(&mut self, t: syn::ItemUse) -> Result<syn::Item, Self::Error> {
        rfold_item_use(self, t)
    }
    fn rfold_item_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Item, Self::Error> {
        rfold_item_verbatim(self, t)
    }
    fn rfold_itemconst(&mut self, t: syn::ItemConst) -> Result<syn::ItemConst, Self::Error> {
        rfold_itemconst(self, t)
    }
    fn rfold_itemenum(&mut self, t: syn::ItemEnum) -> Result<syn::ItemEnum, Self::Error> {
        rfold_itemenum(self, t)
    }
    fn rfold_itemexterncrate(
        &mut self,
        t: syn::ItemExternCrate,
    ) -> Result<syn::ItemExternCrate, Self::Error> {
        rfold_itemexterncrate(self, t)
    }
    fn rfold_itemfn(&mut self, t: syn::ItemFn) -> Result<syn::ItemFn, Self::Error> {
        rfold_itemfn(self, t)
    }
    fn rfold_itemforeignmod(
        &mut self,
        t: syn::ItemForeignMod,
    ) -> Result<syn::ItemForeignMod, Self::Error> {
        rfold_itemforeignmod(self, t)
    }
    fn rfold_itemimpl(&mut self, t: syn::ItemImpl) -> Result<syn::ItemImpl, Self::Error> {
        rfold_itemimpl(self, t)
    }
    fn rfold_itemmacro(&mut self, t: syn::ItemMacro) -> Result<syn::ItemMacro, Self::Error> {
        rfold_itemmacro(self, t)
    }
    fn rfold_itemmacro2(&mut self, t: syn::ItemMacro2) -> Result<syn::ItemMacro2, Self::Error> {
        rfold_itemmacro2(self, t)
    }
    fn rfold_itemmod(&mut self, t: syn::ItemMod) -> Result<syn::ItemMod, Self::Error> {
        rfold_itemmod(self, t)
    }
    fn rfold_itemstatic(&mut self, t: syn::ItemStatic) -> Result<syn::ItemStatic, Self::Error> {
        rfold_itemstatic(self, t)
    }
    fn rfold_itemstruct(&mut self, t: syn::ItemStruct) -> Result<syn::ItemStruct, Self::Error> {
        rfold_itemstruct(self, t)
    }
    fn rfold_itemtrait(&mut self, t: syn::ItemTrait) -> Result<syn::ItemTrait, Self::Error> {
        rfold_itemtrait(self, t)
    }
    fn rfold_itemtraitalias(
        &mut self,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::ItemTraitAlias, Self::Error> {
        rfold_itemtraitalias(self, t)
    }
    fn rfold_itemtype(&mut self, t: syn::ItemType) -> Result<syn::ItemType, Self::Error> {
        rfold_itemtype(self, t)
    }
    fn rfold_itemunion(&mut self, t: syn::ItemUnion) -> Result<syn::ItemUnion, Self::Error> {
        rfold_itemunion(self, t)
    }
    fn rfold_itemuse(&mut self, t: syn::ItemUse) -> Result<syn::ItemUse, Self::Error> {
        rfold_itemuse(self, t)
    }
    fn rfold_label(&mut self, t: syn::Label) -> Result<syn::Label, Self::Error> {
        rfold_label(self, t)
    }
    fn rfold_lifetime(&mut self, t: syn::Lifetime) -> Result<syn::Lifetime, Self::Error> {
        rfold_lifetime(self, t)
    }
    fn rfold_lifetimedef(&mut self, t: syn::LifetimeDef) -> Result<syn::LifetimeDef, Self::Error> {
        rfold_lifetimedef(self, t)
    }
    fn rfold_lit(&mut self, t: syn::Lit) -> Result<syn::Lit, Self::Error> {
        rfold_lit(self, t)
    }
    fn rfold_lit_str(&mut self, t: syn::LitStr) -> Result<syn::Lit, Self::Error> {
        rfold_lit_str(self, t)
    }
    fn rfold_lit_bytestr(&mut self, t: syn::LitByteStr) -> Result<syn::Lit, Self::Error> {
        rfold_lit_bytestr(self, t)
    }
    fn rfold_lit_byte(&mut self, t: syn::LitByte) -> Result<syn::Lit, Self::Error> {
        rfold_lit_byte(self, t)
    }
    fn rfold_lit_char(&mut self, t: syn::LitChar) -> Result<syn::Lit, Self::Error> {
        rfold_lit_char(self, t)
    }
    fn rfold_lit_int(&mut self, t: syn::LitInt) -> Result<syn::Lit, Self::Error> {
        rfold_lit_int(self, t)
    }
    fn rfold_lit_float(&mut self, t: syn::LitFloat) -> Result<syn::Lit, Self::Error> {
        rfold_lit_float(self, t)
    }
    fn rfold_lit_bool(&mut self, t: syn::LitBool) -> Result<syn::Lit, Self::Error> {
        rfold_lit_bool(self, t)
    }
    fn rfold_lit_verbatim(&mut self, t: proc_macro2::Literal) -> Result<syn::Lit, Self::Error> {
        rfold_lit_verbatim(self, t)
    }
    fn rfold_litbool(&mut self, t: syn::LitBool) -> Result<syn::LitBool, Self::Error> {
        rfold_litbool(self, t)
    }
    fn rfold_litbyte(&mut self, t: syn::LitByte) -> Result<syn::LitByte, Self::Error> {
        rfold_litbyte(self, t)
    }
    fn rfold_litbytestr(&mut self, t: syn::LitByteStr) -> Result<syn::LitByteStr, Self::Error> {
        rfold_litbytestr(self, t)
    }
    fn rfold_litchar(&mut self, t: syn::LitChar) -> Result<syn::LitChar, Self::Error> {
        rfold_litchar(self, t)
    }
    fn rfold_litfloat(&mut self, t: syn::LitFloat) -> Result<syn::LitFloat, Self::Error> {
        rfold_litfloat(self, t)
    }
    fn rfold_litint(&mut self, t: syn::LitInt) -> Result<syn::LitInt, Self::Error> {
        rfold_litint(self, t)
    }
    fn rfold_litstr(&mut self, t: syn::LitStr) -> Result<syn::LitStr, Self::Error> {
        rfold_litstr(self, t)
    }
    fn rfold_local(&mut self, t: syn::Local) -> Result<syn::Local, Self::Error> {
        rfold_local(self, t)
    }
    fn rfold_macro(&mut self, t: syn::Macro) -> Result<syn::Macro, Self::Error> {
        rfold_macro(self, t)
    }
    fn rfold_macrodelimiter(
        &mut self,
        t: syn::MacroDelimiter,
    ) -> Result<syn::MacroDelimiter, Self::Error> {
        rfold_macrodelimiter(self, t)
    }
    fn rfold_macrodelimiter_paren(
        &mut self,
        t: syn::token::Paren,
    ) -> Result<syn::MacroDelimiter, Self::Error> {
        rfold_macrodelimiter_paren(self, t)
    }
    fn rfold_macrodelimiter_brace(
        &mut self,
        t: syn::token::Brace,
    ) -> Result<syn::MacroDelimiter, Self::Error> {
        rfold_macrodelimiter_brace(self, t)
    }
    fn rfold_macrodelimiter_bracket(
        &mut self,
        t: syn::token::Bracket,
    ) -> Result<syn::MacroDelimiter, Self::Error> {
        rfold_macrodelimiter_bracket(self, t)
    }
    fn rfold_member(&mut self, t: syn::Member) -> Result<syn::Member, Self::Error> {
        rfold_member(self, t)
    }
    fn rfold_member_named(&mut self, t: proc_macro2::Ident) -> Result<syn::Member, Self::Error> {
        rfold_member_named(self, t)
    }
    fn rfold_member_unnamed(&mut self, t: syn::Index) -> Result<syn::Member, Self::Error> {
        rfold_member_unnamed(self, t)
    }
    fn rfold_meta(&mut self, t: syn::Meta) -> Result<syn::Meta, Self::Error> {
        rfold_meta(self, t)
    }
    fn rfold_meta_path(&mut self, t: syn::Path) -> Result<syn::Meta, Self::Error> {
        rfold_meta_path(self, t)
    }
    fn rfold_meta_list(&mut self, t: syn::MetaList) -> Result<syn::Meta, Self::Error> {
        rfold_meta_list(self, t)
    }
    fn rfold_meta_namevalue(&mut self, t: syn::MetaNameValue) -> Result<syn::Meta, Self::Error> {
        rfold_meta_namevalue(self, t)
    }
    fn rfold_metalist(&mut self, t: syn::MetaList) -> Result<syn::MetaList, Self::Error> {
        rfold_metalist(self, t)
    }
    fn rfold_metanamevalue(
        &mut self,
        t: syn::MetaNameValue,
    ) -> Result<syn::MetaNameValue, Self::Error> {
        rfold_metanamevalue(self, t)
    }
    fn rfold_methodturbofish(
        &mut self,
        t: syn::MethodTurbofish,
    ) -> Result<syn::MethodTurbofish, Self::Error> {
        rfold_methodturbofish(self, t)
    }
    fn rfold_nestedmeta(&mut self, t: syn::NestedMeta) -> Result<syn::NestedMeta, Self::Error> {
        rfold_nestedmeta(self, t)
    }
    fn rfold_nestedmeta_meta(&mut self, t: syn::Meta) -> Result<syn::NestedMeta, Self::Error> {
        rfold_nestedmeta_meta(self, t)
    }
    fn rfold_nestedmeta_lit(&mut self, t: syn::Lit) -> Result<syn::NestedMeta, Self::Error> {
        rfold_nestedmeta_lit(self, t)
    }
    fn rfold_parenthesizedgenericarguments(
        &mut self,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::ParenthesizedGenericArguments, Self::Error> {
        rfold_parenthesizedgenericarguments(self, t)
    }
    fn rfold_pat(&mut self, t: syn::Pat) -> Result<syn::Pat, Self::Error> {
        rfold_pat(self, t)
    }
    fn rfold_pat_box(&mut self, t: syn::PatBox) -> Result<syn::Pat, Self::Error> {
        rfold_pat_box(self, t)
    }
    fn rfold_pat_ident(&mut self, t: syn::PatIdent) -> Result<syn::Pat, Self::Error> {
        rfold_pat_ident(self, t)
    }
    fn rfold_pat_lit(&mut self, t: syn::PatLit) -> Result<syn::Pat, Self::Error> {
        rfold_pat_lit(self, t)
    }
    fn rfold_pat_macro(&mut self, t: syn::PatMacro) -> Result<syn::Pat, Self::Error> {
        rfold_pat_macro(self, t)
    }
    fn rfold_pat_or(&mut self, t: syn::PatOr) -> Result<syn::Pat, Self::Error> {
        rfold_pat_or(self, t)
    }
    fn rfold_pat_path(&mut self, t: syn::PatPath) -> Result<syn::Pat, Self::Error> {
        rfold_pat_path(self, t)
    }
    fn rfold_pat_range(&mut self, t: syn::PatRange) -> Result<syn::Pat, Self::Error> {
        rfold_pat_range(self, t)
    }
    fn rfold_pat_reference(&mut self, t: syn::PatReference) -> Result<syn::Pat, Self::Error> {
        rfold_pat_reference(self, t)
    }
    fn rfold_pat_rest(&mut self, t: syn::PatRest) -> Result<syn::Pat, Self::Error> {
        rfold_pat_rest(self, t)
    }
    fn rfold_pat_slice(&mut self, t: syn::PatSlice) -> Result<syn::Pat, Self::Error> {
        rfold_pat_slice(self, t)
    }
    fn rfold_pat_struct(&mut self, t: syn::PatStruct) -> Result<syn::Pat, Self::Error> {
        rfold_pat_struct(self, t)
    }
    fn rfold_pat_tuple(&mut self, t: syn::PatTuple) -> Result<syn::Pat, Self::Error> {
        rfold_pat_tuple(self, t)
    }
    fn rfold_pat_tuplestruct(&mut self, t: syn::PatTupleStruct) -> Result<syn::Pat, Self::Error> {
        rfold_pat_tuplestruct(self, t)
    }
    fn rfold_pat_type(&mut self, t: syn::PatType) -> Result<syn::Pat, Self::Error> {
        rfold_pat_type(self, t)
    }
    fn rfold_pat_verbatim(&mut self, t: proc_macro2::TokenStream) -> Result<syn::Pat, Self::Error> {
        rfold_pat_verbatim(self, t)
    }
    fn rfold_pat_wild(&mut self, t: syn::PatWild) -> Result<syn::Pat, Self::Error> {
        rfold_pat_wild(self, t)
    }
    fn rfold_patbox(&mut self, t: syn::PatBox) -> Result<syn::PatBox, Self::Error> {
        rfold_patbox(self, t)
    }
    fn rfold_patident(&mut self, t: syn::PatIdent) -> Result<syn::PatIdent, Self::Error> {
        rfold_patident(self, t)
    }
    fn rfold_patlit(&mut self, t: syn::PatLit) -> Result<syn::PatLit, Self::Error> {
        rfold_patlit(self, t)
    }
    fn rfold_patmacro(&mut self, t: syn::PatMacro) -> Result<syn::PatMacro, Self::Error> {
        rfold_patmacro(self, t)
    }
    fn rfold_pator(&mut self, t: syn::PatOr) -> Result<syn::PatOr, Self::Error> {
        rfold_pator(self, t)
    }
    fn rfold_patpath(&mut self, t: syn::PatPath) -> Result<syn::PatPath, Self::Error> {
        rfold_patpath(self, t)
    }
    fn rfold_patrange(&mut self, t: syn::PatRange) -> Result<syn::PatRange, Self::Error> {
        rfold_patrange(self, t)
    }
    fn rfold_patreference(
        &mut self,
        t: syn::PatReference,
    ) -> Result<syn::PatReference, Self::Error> {
        rfold_patreference(self, t)
    }
    fn rfold_patrest(&mut self, t: syn::PatRest) -> Result<syn::PatRest, Self::Error> {
        rfold_patrest(self, t)
    }
    fn rfold_patslice(&mut self, t: syn::PatSlice) -> Result<syn::PatSlice, Self::Error> {
        rfold_patslice(self, t)
    }
    fn rfold_patstruct(&mut self, t: syn::PatStruct) -> Result<syn::PatStruct, Self::Error> {
        rfold_patstruct(self, t)
    }
    fn rfold_pattuple(&mut self, t: syn::PatTuple) -> Result<syn::PatTuple, Self::Error> {
        rfold_pattuple(self, t)
    }
    fn rfold_pattuplestruct(
        &mut self,
        t: syn::PatTupleStruct,
    ) -> Result<syn::PatTupleStruct, Self::Error> {
        rfold_pattuplestruct(self, t)
    }
    fn rfold_pattype(&mut self, t: syn::PatType) -> Result<syn::PatType, Self::Error> {
        rfold_pattype(self, t)
    }
    fn rfold_patwild(&mut self, t: syn::PatWild) -> Result<syn::PatWild, Self::Error> {
        rfold_patwild(self, t)
    }
    fn rfold_path(&mut self, t: syn::Path) -> Result<syn::Path, Self::Error> {
        rfold_path(self, t)
    }
    fn rfold_patharguments(
        &mut self,
        t: syn::PathArguments,
    ) -> Result<syn::PathArguments, Self::Error> {
        rfold_patharguments(self, t)
    }
    fn rfold_patharguments_anglebracketed(
        &mut self,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::PathArguments, Self::Error> {
        rfold_patharguments_anglebracketed(self, t)
    }
    fn rfold_patharguments_parenthesized(
        &mut self,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::PathArguments, Self::Error> {
        rfold_patharguments_parenthesized(self, t)
    }
    fn rfold_pathsegment(&mut self, t: syn::PathSegment) -> Result<syn::PathSegment, Self::Error> {
        rfold_pathsegment(self, t)
    }
    fn rfold_predicateeq(&mut self, t: syn::PredicateEq) -> Result<syn::PredicateEq, Self::Error> {
        rfold_predicateeq(self, t)
    }
    fn rfold_predicatelifetime(
        &mut self,
        t: syn::PredicateLifetime,
    ) -> Result<syn::PredicateLifetime, Self::Error> {
        rfold_predicatelifetime(self, t)
    }
    fn rfold_predicatetype(
        &mut self,
        t: syn::PredicateType,
    ) -> Result<syn::PredicateType, Self::Error> {
        rfold_predicatetype(self, t)
    }
    fn rfold_qself(&mut self, t: syn::QSelf) -> Result<syn::QSelf, Self::Error> {
        rfold_qself(self, t)
    }
    fn rfold_rangelimits(&mut self, t: syn::RangeLimits) -> Result<syn::RangeLimits, Self::Error> {
        rfold_rangelimits(self, t)
    }
    fn rfold_rangelimits_halfopen(
        &mut self,
        t: syn::token::Dot2,
    ) -> Result<syn::RangeLimits, Self::Error> {
        rfold_rangelimits_halfopen(self, t)
    }
    fn rfold_rangelimits_closed(
        &mut self,
        t: syn::token::DotDotEq,
    ) -> Result<syn::RangeLimits, Self::Error> {
        rfold_rangelimits_closed(self, t)
    }
    fn rfold_receiver(&mut self, t: syn::Receiver) -> Result<syn::Receiver, Self::Error> {
        rfold_receiver(self, t)
    }
    fn rfold_returntype(&mut self, t: syn::ReturnType) -> Result<syn::ReturnType, Self::Error> {
        rfold_returntype(self, t)
    }
    fn rfold_returntype_type(
        &mut self,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<syn::ReturnType, Self::Error> {
        rfold_returntype_type(self, t)
    }
    fn rfold_signature(&mut self, t: syn::Signature) -> Result<syn::Signature, Self::Error> {
        rfold_signature(self, t)
    }
    fn rfold_stmt(&mut self, t: syn::Stmt) -> Result<syn::Stmt, Self::Error> {
        rfold_stmt(self, t)
    }
    fn rfold_stmt_local(&mut self, t: syn::Local) -> Result<syn::Stmt, Self::Error> {
        rfold_stmt_local(self, t)
    }
    fn rfold_stmt_item(&mut self, t: syn::Item) -> Result<syn::Stmt, Self::Error> {
        rfold_stmt_item(self, t)
    }
    fn rfold_stmt_expr(&mut self, t: syn::Expr) -> Result<syn::Stmt, Self::Error> {
        rfold_stmt_expr(self, t)
    }
    fn rfold_stmt_semi(
        &mut self,
        t: (syn::Expr, syn::token::Semi),
    ) -> Result<syn::Stmt, Self::Error> {
        rfold_stmt_semi(self, t)
    }
    fn rfold_traitbound(&mut self, t: syn::TraitBound) -> Result<syn::TraitBound, Self::Error> {
        rfold_traitbound(self, t)
    }
    fn rfold_traitboundmodifier(
        &mut self,
        t: syn::TraitBoundModifier,
    ) -> Result<syn::TraitBoundModifier, Self::Error> {
        rfold_traitboundmodifier(self, t)
    }
    fn rfold_traitboundmodifier_maybe(
        &mut self,
        t: syn::token::Question,
    ) -> Result<syn::TraitBoundModifier, Self::Error> {
        rfold_traitboundmodifier_maybe(self, t)
    }
    fn rfold_traititem(&mut self, t: syn::TraitItem) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem(self, t)
    }
    fn rfold_traititem_const(
        &mut self,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem_const(self, t)
    }
    fn rfold_traititem_method(
        &mut self,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem_method(self, t)
    }
    fn rfold_traititem_type(
        &mut self,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem_type(self, t)
    }
    fn rfold_traititem_macro(
        &mut self,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem_macro(self, t)
    }
    fn rfold_traititem_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::TraitItem, Self::Error> {
        rfold_traititem_verbatim(self, t)
    }
    fn rfold_traititemconst(
        &mut self,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItemConst, Self::Error> {
        rfold_traititemconst(self, t)
    }
    fn rfold_traititemmacro(
        &mut self,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItemMacro, Self::Error> {
        rfold_traititemmacro(self, t)
    }
    fn rfold_traititemmethod(
        &mut self,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItemMethod, Self::Error> {
        rfold_traititemmethod(self, t)
    }
    fn rfold_traititemtype(
        &mut self,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItemType, Self::Error> {
        rfold_traititemtype(self, t)
    }
    fn rfold_type(&mut self, t: syn::Type) -> Result<syn::Type, Self::Error> {
        rfold_type(self, t)
    }
    fn rfold_type_array(&mut self, t: syn::TypeArray) -> Result<syn::Type, Self::Error> {
        rfold_type_array(self, t)
    }
    fn rfold_type_barefn(&mut self, t: syn::TypeBareFn) -> Result<syn::Type, Self::Error> {
        rfold_type_barefn(self, t)
    }
    fn rfold_type_group(&mut self, t: syn::TypeGroup) -> Result<syn::Type, Self::Error> {
        rfold_type_group(self, t)
    }
    fn rfold_type_impltrait(&mut self, t: syn::TypeImplTrait) -> Result<syn::Type, Self::Error> {
        rfold_type_impltrait(self, t)
    }
    fn rfold_type_infer(&mut self, t: syn::TypeInfer) -> Result<syn::Type, Self::Error> {
        rfold_type_infer(self, t)
    }
    fn rfold_type_macro(&mut self, t: syn::TypeMacro) -> Result<syn::Type, Self::Error> {
        rfold_type_macro(self, t)
    }
    fn rfold_type_never(&mut self, t: syn::TypeNever) -> Result<syn::Type, Self::Error> {
        rfold_type_never(self, t)
    }
    fn rfold_type_paren(&mut self, t: syn::TypeParen) -> Result<syn::Type, Self::Error> {
        rfold_type_paren(self, t)
    }
    fn rfold_type_path(&mut self, t: syn::TypePath) -> Result<syn::Type, Self::Error> {
        rfold_type_path(self, t)
    }
    fn rfold_type_ptr(&mut self, t: syn::TypePtr) -> Result<syn::Type, Self::Error> {
        rfold_type_ptr(self, t)
    }
    fn rfold_type_reference(&mut self, t: syn::TypeReference) -> Result<syn::Type, Self::Error> {
        rfold_type_reference(self, t)
    }
    fn rfold_type_slice(&mut self, t: syn::TypeSlice) -> Result<syn::Type, Self::Error> {
        rfold_type_slice(self, t)
    }
    fn rfold_type_traitobject(
        &mut self,
        t: syn::TypeTraitObject,
    ) -> Result<syn::Type, Self::Error> {
        rfold_type_traitobject(self, t)
    }
    fn rfold_type_tuple(&mut self, t: syn::TypeTuple) -> Result<syn::Type, Self::Error> {
        rfold_type_tuple(self, t)
    }
    fn rfold_type_verbatim(
        &mut self,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Type, Self::Error> {
        rfold_type_verbatim(self, t)
    }
    fn rfold_typearray(&mut self, t: syn::TypeArray) -> Result<syn::TypeArray, Self::Error> {
        rfold_typearray(self, t)
    }
    fn rfold_typebarefn(&mut self, t: syn::TypeBareFn) -> Result<syn::TypeBareFn, Self::Error> {
        rfold_typebarefn(self, t)
    }
    fn rfold_typegroup(&mut self, t: syn::TypeGroup) -> Result<syn::TypeGroup, Self::Error> {
        rfold_typegroup(self, t)
    }
    fn rfold_typeimpltrait(
        &mut self,
        t: syn::TypeImplTrait,
    ) -> Result<syn::TypeImplTrait, Self::Error> {
        rfold_typeimpltrait(self, t)
    }
    fn rfold_typeinfer(&mut self, t: syn::TypeInfer) -> Result<syn::TypeInfer, Self::Error> {
        rfold_typeinfer(self, t)
    }
    fn rfold_typemacro(&mut self, t: syn::TypeMacro) -> Result<syn::TypeMacro, Self::Error> {
        rfold_typemacro(self, t)
    }
    fn rfold_typenever(&mut self, t: syn::TypeNever) -> Result<syn::TypeNever, Self::Error> {
        rfold_typenever(self, t)
    }
    fn rfold_typeparam(&mut self, t: syn::TypeParam) -> Result<syn::TypeParam, Self::Error> {
        rfold_typeparam(self, t)
    }
    fn rfold_typeparambound(
        &mut self,
        t: syn::TypeParamBound,
    ) -> Result<syn::TypeParamBound, Self::Error> {
        rfold_typeparambound(self, t)
    }
    fn rfold_typeparambound_trait(
        &mut self,
        t: syn::TraitBound,
    ) -> Result<syn::TypeParamBound, Self::Error> {
        rfold_typeparambound_trait(self, t)
    }
    fn rfold_typeparambound_lifetime(
        &mut self,
        t: syn::Lifetime,
    ) -> Result<syn::TypeParamBound, Self::Error> {
        rfold_typeparambound_lifetime(self, t)
    }
    fn rfold_typeparen(&mut self, t: syn::TypeParen) -> Result<syn::TypeParen, Self::Error> {
        rfold_typeparen(self, t)
    }
    fn rfold_typepath(&mut self, t: syn::TypePath) -> Result<syn::TypePath, Self::Error> {
        rfold_typepath(self, t)
    }
    fn rfold_typeptr(&mut self, t: syn::TypePtr) -> Result<syn::TypePtr, Self::Error> {
        rfold_typeptr(self, t)
    }
    fn rfold_typereference(
        &mut self,
        t: syn::TypeReference,
    ) -> Result<syn::TypeReference, Self::Error> {
        rfold_typereference(self, t)
    }
    fn rfold_typeslice(&mut self, t: syn::TypeSlice) -> Result<syn::TypeSlice, Self::Error> {
        rfold_typeslice(self, t)
    }
    fn rfold_typetraitobject(
        &mut self,
        t: syn::TypeTraitObject,
    ) -> Result<syn::TypeTraitObject, Self::Error> {
        rfold_typetraitobject(self, t)
    }
    fn rfold_typetuple(&mut self, t: syn::TypeTuple) -> Result<syn::TypeTuple, Self::Error> {
        rfold_typetuple(self, t)
    }
    fn rfold_unop(&mut self, t: syn::UnOp) -> Result<syn::UnOp, Self::Error> {
        rfold_unop(self, t)
    }
    fn rfold_unop_deref(&mut self, t: syn::token::Star) -> Result<syn::UnOp, Self::Error> {
        rfold_unop_deref(self, t)
    }
    fn rfold_unop_not(&mut self, t: syn::token::Bang) -> Result<syn::UnOp, Self::Error> {
        rfold_unop_not(self, t)
    }
    fn rfold_unop_neg(&mut self, t: syn::token::Sub) -> Result<syn::UnOp, Self::Error> {
        rfold_unop_neg(self, t)
    }
    fn rfold_useglob(&mut self, t: syn::UseGlob) -> Result<syn::UseGlob, Self::Error> {
        rfold_useglob(self, t)
    }
    fn rfold_usegroup(&mut self, t: syn::UseGroup) -> Result<syn::UseGroup, Self::Error> {
        rfold_usegroup(self, t)
    }
    fn rfold_usename(&mut self, t: syn::UseName) -> Result<syn::UseName, Self::Error> {
        rfold_usename(self, t)
    }
    fn rfold_usepath(&mut self, t: syn::UsePath) -> Result<syn::UsePath, Self::Error> {
        rfold_usepath(self, t)
    }
    fn rfold_userename(&mut self, t: syn::UseRename) -> Result<syn::UseRename, Self::Error> {
        rfold_userename(self, t)
    }
    fn rfold_usetree(&mut self, t: syn::UseTree) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree(self, t)
    }
    fn rfold_usetree_path(&mut self, t: syn::UsePath) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree_path(self, t)
    }
    fn rfold_usetree_name(&mut self, t: syn::UseName) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree_name(self, t)
    }
    fn rfold_usetree_rename(&mut self, t: syn::UseRename) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree_rename(self, t)
    }
    fn rfold_usetree_glob(&mut self, t: syn::UseGlob) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree_glob(self, t)
    }
    fn rfold_usetree_group(&mut self, t: syn::UseGroup) -> Result<syn::UseTree, Self::Error> {
        rfold_usetree_group(self, t)
    }
    fn rfold_variadic(&mut self, t: syn::Variadic) -> Result<syn::Variadic, Self::Error> {
        rfold_variadic(self, t)
    }
    fn rfold_variant(&mut self, t: syn::Variant) -> Result<syn::Variant, Self::Error> {
        rfold_variant(self, t)
    }
    fn rfold_viscrate(&mut self, t: syn::VisCrate) -> Result<syn::VisCrate, Self::Error> {
        rfold_viscrate(self, t)
    }
    fn rfold_vispublic(&mut self, t: syn::VisPublic) -> Result<syn::VisPublic, Self::Error> {
        rfold_vispublic(self, t)
    }
    fn rfold_visrestricted(
        &mut self,
        t: syn::VisRestricted,
    ) -> Result<syn::VisRestricted, Self::Error> {
        rfold_visrestricted(self, t)
    }
    fn rfold_visibility(&mut self, t: syn::Visibility) -> Result<syn::Visibility, Self::Error> {
        rfold_visibility(self, t)
    }
    fn rfold_visibility_public(
        &mut self,
        t: syn::VisPublic,
    ) -> Result<syn::Visibility, Self::Error> {
        rfold_visibility_public(self, t)
    }
    fn rfold_visibility_crate(&mut self, t: syn::VisCrate) -> Result<syn::Visibility, Self::Error> {
        rfold_visibility_crate(self, t)
    }
    fn rfold_visibility_restricted(
        &mut self,
        t: syn::VisRestricted,
    ) -> Result<syn::Visibility, Self::Error> {
        rfold_visibility_restricted(self, t)
    }
    fn rfold_whereclause(&mut self, t: syn::WhereClause) -> Result<syn::WhereClause, Self::Error> {
        rfold_whereclause(self, t)
    }
    fn rfold_wherepredicate(
        &mut self,
        t: syn::WherePredicate,
    ) -> Result<syn::WherePredicate, Self::Error> {
        rfold_wherepredicate(self, t)
    }
    fn rfold_wherepredicate_type(
        &mut self,
        t: syn::PredicateType,
    ) -> Result<syn::WherePredicate, Self::Error> {
        rfold_wherepredicate_type(self, t)
    }
    fn rfold_wherepredicate_lifetime(
        &mut self,
        t: syn::PredicateLifetime,
    ) -> Result<syn::WherePredicate, Self::Error> {
        rfold_wherepredicate_lifetime(self, t)
    }
    fn rfold_wherepredicate_eq(
        &mut self,
        t: syn::PredicateEq,
    ) -> Result<syn::WherePredicate, Self::Error> {
        rfold_wherepredicate_eq(self, t)
    }
}
pub fn rfold_abi<F>(f: &mut F, t: syn::Abi) -> Result<syn::Abi, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.extern_token = t.extern_token;
    t.name = match t.name {
        Some(o) => Some(f.rfold_litstr(o)?),
        None => None,
    };
    Ok(t)
}
pub fn rfold_anglebracketedgenericarguments<F>(
    f: &mut F,
    t: syn::AngleBracketedGenericArguments,
) -> Result<syn::AngleBracketedGenericArguments, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.colon2_token = t.colon2_token;
    t.lt_token = t.lt_token;
    t.args = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.args {
            tmp.push(f.rfold_genericargument(v)?);
        }
        tmp
    };
    t.gt_token = t.gt_token;
    Ok(t)
}
pub fn rfold_arm<F>(f: &mut F, t: syn::Arm) -> Result<syn::Arm, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.pat = f.rfold_pat(t.pat)?;
    t.guard = match t.guard {
        Some(o) => Some((o.0, Box::new(f.rfold_expr(*o.1)?))),
        None => None,
    };
    t.fat_arrow_token = t.fat_arrow_token;
    t.body = Box::new(f.rfold_expr(*t.body)?);
    t.comma = t.comma;
    Ok(t)
}
pub fn rfold_attrstyle<F>(
    f: &mut F,
    t: syn::AttrStyle,
) -> Result<syn::AttrStyle, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::AttrStyle::Inner(tmp0) => f.rfold_attrstyle_inner(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_attrstyle_inner<F>(
    _f: &mut F,
    t: syn::token::Bang,
) -> Result<syn::AttrStyle, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::AttrStyle::Inner(t))
}
pub fn rfold_attribute<F>(
    f: &mut F,
    t: syn::Attribute,
) -> Result<syn::Attribute, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.pound_token = t.pound_token;
    t.style = f.rfold_attrstyle(t.style)?;
    t.bracket_token = t.bracket_token;
    t.path = f.rfold_path(t.path)?;
    t.tokens = t.tokens;
    Ok(t)
}
pub fn rfold_barefnarg<F>(
    f: &mut F,
    t: syn::BareFnArg,
) -> Result<syn::BareFnArg, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.name = match t.name {
        Some(o) => Some((o.0, o.1)),
        None => None,
    };
    t.ty = f.rfold_type(t.ty)?;
    Ok(t)
}
pub fn rfold_binop<F>(f: &mut F, t: syn::BinOp) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::BinOp::Add(tmp0) => f.rfold_binop_add(tmp0)?,
        syn::BinOp::Sub(tmp0) => f.rfold_binop_sub(tmp0)?,
        syn::BinOp::Mul(tmp0) => f.rfold_binop_mul(tmp0)?,
        syn::BinOp::Div(tmp0) => f.rfold_binop_div(tmp0)?,
        syn::BinOp::Rem(tmp0) => f.rfold_binop_rem(tmp0)?,
        syn::BinOp::And(tmp0) => f.rfold_binop_and(tmp0)?,
        syn::BinOp::Or(tmp0) => f.rfold_binop_or(tmp0)?,
        syn::BinOp::BitXor(tmp0) => f.rfold_binop_bitxor(tmp0)?,
        syn::BinOp::BitAnd(tmp0) => f.rfold_binop_bitand(tmp0)?,
        syn::BinOp::BitOr(tmp0) => f.rfold_binop_bitor(tmp0)?,
        syn::BinOp::Shl(tmp0) => f.rfold_binop_shl(tmp0)?,
        syn::BinOp::Shr(tmp0) => f.rfold_binop_shr(tmp0)?,
        syn::BinOp::Eq(tmp0) => f.rfold_binop_eq(tmp0)?,
        syn::BinOp::Lt(tmp0) => f.rfold_binop_lt(tmp0)?,
        syn::BinOp::Le(tmp0) => f.rfold_binop_le(tmp0)?,
        syn::BinOp::Ne(tmp0) => f.rfold_binop_ne(tmp0)?,
        syn::BinOp::Ge(tmp0) => f.rfold_binop_ge(tmp0)?,
        syn::BinOp::Gt(tmp0) => f.rfold_binop_gt(tmp0)?,
        syn::BinOp::AddEq(tmp0) => f.rfold_binop_addeq(tmp0)?,
        syn::BinOp::SubEq(tmp0) => f.rfold_binop_subeq(tmp0)?,
        syn::BinOp::MulEq(tmp0) => f.rfold_binop_muleq(tmp0)?,
        syn::BinOp::DivEq(tmp0) => f.rfold_binop_diveq(tmp0)?,
        syn::BinOp::RemEq(tmp0) => f.rfold_binop_remeq(tmp0)?,
        syn::BinOp::BitXorEq(tmp0) => f.rfold_binop_bitxoreq(tmp0)?,
        syn::BinOp::BitAndEq(tmp0) => f.rfold_binop_bitandeq(tmp0)?,
        syn::BinOp::BitOrEq(tmp0) => f.rfold_binop_bitoreq(tmp0)?,
        syn::BinOp::ShlEq(tmp0) => f.rfold_binop_shleq(tmp0)?,
        syn::BinOp::ShrEq(tmp0) => f.rfold_binop_shreq(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_binop_add<F>(_f: &mut F, t: syn::token::Add) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Add(t))
}
pub fn rfold_binop_sub<F>(_f: &mut F, t: syn::token::Sub) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Sub(t))
}
pub fn rfold_binop_mul<F>(
    _f: &mut F,
    t: syn::token::Star,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Mul(t))
}
pub fn rfold_binop_div<F>(_f: &mut F, t: syn::token::Div) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Div(t))
}
pub fn rfold_binop_rem<F>(_f: &mut F, t: syn::token::Rem) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Rem(t))
}
pub fn rfold_binop_and<F>(
    _f: &mut F,
    t: syn::token::AndAnd,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::And(t))
}
pub fn rfold_binop_or<F>(_f: &mut F, t: syn::token::OrOr) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Or(t))
}
pub fn rfold_binop_bitxor<F>(
    _f: &mut F,
    t: syn::token::Caret,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitXor(t))
}
pub fn rfold_binop_bitand<F>(
    _f: &mut F,
    t: syn::token::And,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitAnd(t))
}
pub fn rfold_binop_bitor<F>(
    _f: &mut F,
    t: syn::token::Or,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitOr(t))
}
pub fn rfold_binop_shl<F>(_f: &mut F, t: syn::token::Shl) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Shl(t))
}
pub fn rfold_binop_shr<F>(_f: &mut F, t: syn::token::Shr) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Shr(t))
}
pub fn rfold_binop_eq<F>(_f: &mut F, t: syn::token::EqEq) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Eq(t))
}
pub fn rfold_binop_lt<F>(_f: &mut F, t: syn::token::Lt) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Lt(t))
}
pub fn rfold_binop_le<F>(_f: &mut F, t: syn::token::Le) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Le(t))
}
pub fn rfold_binop_ne<F>(_f: &mut F, t: syn::token::Ne) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Ne(t))
}
pub fn rfold_binop_ge<F>(_f: &mut F, t: syn::token::Ge) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Ge(t))
}
pub fn rfold_binop_gt<F>(_f: &mut F, t: syn::token::Gt) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::Gt(t))
}
pub fn rfold_binop_addeq<F>(
    _f: &mut F,
    t: syn::token::AddEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::AddEq(t))
}
pub fn rfold_binop_subeq<F>(
    _f: &mut F,
    t: syn::token::SubEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::SubEq(t))
}
pub fn rfold_binop_muleq<F>(
    _f: &mut F,
    t: syn::token::MulEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::MulEq(t))
}
pub fn rfold_binop_diveq<F>(
    _f: &mut F,
    t: syn::token::DivEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::DivEq(t))
}
pub fn rfold_binop_remeq<F>(
    _f: &mut F,
    t: syn::token::RemEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::RemEq(t))
}
pub fn rfold_binop_bitxoreq<F>(
    _f: &mut F,
    t: syn::token::CaretEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitXorEq(t))
}
pub fn rfold_binop_bitandeq<F>(
    _f: &mut F,
    t: syn::token::AndEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitAndEq(t))
}
pub fn rfold_binop_bitoreq<F>(
    _f: &mut F,
    t: syn::token::OrEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::BitOrEq(t))
}
pub fn rfold_binop_shleq<F>(
    _f: &mut F,
    t: syn::token::ShlEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::ShlEq(t))
}
pub fn rfold_binop_shreq<F>(
    _f: &mut F,
    t: syn::token::ShrEq,
) -> Result<syn::BinOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::BinOp::ShrEq(t))
}
pub fn rfold_binding<F>(f: &mut F, t: syn::Binding) -> Result<syn::Binding, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    t.eq_token = t.eq_token;
    t.ty = f.rfold_type(t.ty)?;
    Ok(t)
}
pub fn rfold_block<F>(f: &mut F, t: syn::Block) -> Result<syn::Block, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.brace_token = t.brace_token;
    t.stmts = {
        let mut tmp = vec![];
        for v in t.stmts {
            tmp.push(f.rfold_stmt(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_boundlifetimes<F>(
    f: &mut F,
    t: syn::BoundLifetimes,
) -> Result<syn::BoundLifetimes, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.for_token = t.for_token;
    t.lt_token = t.lt_token;
    t.lifetimes = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.lifetimes {
            tmp.push(f.rfold_lifetimedef(v)?);
        }
        tmp
    };
    t.gt_token = t.gt_token;
    Ok(t)
}
pub fn rfold_constparam<F>(
    f: &mut F,
    t: syn::ConstParam,
) -> Result<syn::ConstParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.const_token = t.const_token;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = f.rfold_type(t.ty)?;
    t.eq_token = t.eq_token;
    t.default = match t.default {
        Some(o) => Some(f.rfold_expr(o)?),
        None => None,
    };
    Ok(t)
}
pub fn rfold_constraint<F>(
    f: &mut F,
    t: syn::Constraint,
) -> Result<syn::Constraint, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_data<F>(f: &mut F, t: syn::Data) -> Result<syn::Data, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Data::Struct(tmp0) => f.rfold_data_struct(tmp0)?,
        syn::Data::Enum(tmp0) => f.rfold_data_enum(tmp0)?,
        syn::Data::Union(tmp0) => f.rfold_data_union(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_data_struct<F>(f: &mut F, t: syn::DataStruct) -> Result<syn::Data, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_datastruct(t)?;
    Ok(syn::Data::Struct(t))
}
pub fn rfold_data_enum<F>(f: &mut F, t: syn::DataEnum) -> Result<syn::Data, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_dataenum(t)?;
    Ok(syn::Data::Enum(t))
}
pub fn rfold_data_union<F>(f: &mut F, t: syn::DataUnion) -> Result<syn::Data, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_dataunion(t)?;
    Ok(syn::Data::Union(t))
}
pub fn rfold_dataenum<F>(f: &mut F, t: syn::DataEnum) -> Result<syn::DataEnum, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.enum_token = t.enum_token;
    t.brace_token = t.brace_token;
    t.variants = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.variants {
            tmp.push(f.rfold_variant(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_datastruct<F>(
    f: &mut F,
    t: syn::DataStruct,
) -> Result<syn::DataStruct, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.struct_token = t.struct_token;
    t.fields = f.rfold_fields(t.fields)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_dataunion<F>(
    f: &mut F,
    t: syn::DataUnion,
) -> Result<syn::DataUnion, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.union_token = t.union_token;
    t.fields = f.rfold_fieldsnamed(t.fields)?;
    Ok(t)
}
pub fn rfold_deriveinput<F>(
    f: &mut F,
    t: syn::DeriveInput,
) -> Result<syn::DeriveInput, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.data = f.rfold_data(t.data)?;
    Ok(t)
}
pub fn rfold_expr<F>(f: &mut F, t: syn::Expr) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Expr::Array(tmp0) => f.rfold_expr_array(tmp0)?,
        syn::Expr::Assign(tmp0) => f.rfold_expr_assign(tmp0)?,
        syn::Expr::AssignOp(tmp0) => f.rfold_expr_assignop(tmp0)?,
        syn::Expr::Async(tmp0) => f.rfold_expr_async(tmp0)?,
        syn::Expr::Await(tmp0) => f.rfold_expr_await(tmp0)?,
        syn::Expr::Binary(tmp0) => f.rfold_expr_binary(tmp0)?,
        syn::Expr::Block(tmp0) => f.rfold_expr_block(tmp0)?,
        syn::Expr::Box(tmp0) => f.rfold_expr_box(tmp0)?,
        syn::Expr::Break(tmp0) => f.rfold_expr_break(tmp0)?,
        syn::Expr::Call(tmp0) => f.rfold_expr_call(tmp0)?,
        syn::Expr::Cast(tmp0) => f.rfold_expr_cast(tmp0)?,
        syn::Expr::Closure(tmp0) => f.rfold_expr_closure(tmp0)?,
        syn::Expr::Continue(tmp0) => f.rfold_expr_continue(tmp0)?,
        syn::Expr::Field(tmp0) => f.rfold_expr_field(tmp0)?,
        syn::Expr::ForLoop(tmp0) => f.rfold_expr_forloop(tmp0)?,
        syn::Expr::Group(tmp0) => f.rfold_expr_group(tmp0)?,
        syn::Expr::If(tmp0) => f.rfold_expr_if(tmp0)?,
        syn::Expr::Index(tmp0) => f.rfold_expr_index(tmp0)?,
        syn::Expr::Let(tmp0) => f.rfold_expr_let(tmp0)?,
        syn::Expr::Lit(tmp0) => f.rfold_expr_lit(tmp0)?,
        syn::Expr::Loop(tmp0) => f.rfold_expr_loop(tmp0)?,
        syn::Expr::Macro(tmp0) => f.rfold_expr_macro(tmp0)?,
        syn::Expr::Match(tmp0) => f.rfold_expr_match(tmp0)?,
        syn::Expr::MethodCall(tmp0) => f.rfold_expr_methodcall(tmp0)?,
        syn::Expr::Paren(tmp0) => f.rfold_expr_paren(tmp0)?,
        syn::Expr::Path(tmp0) => f.rfold_expr_path(tmp0)?,
        syn::Expr::Range(tmp0) => f.rfold_expr_range(tmp0)?,
        syn::Expr::Reference(tmp0) => f.rfold_expr_reference(tmp0)?,
        syn::Expr::Repeat(tmp0) => f.rfold_expr_repeat(tmp0)?,
        syn::Expr::Return(tmp0) => f.rfold_expr_return(tmp0)?,
        syn::Expr::Struct(tmp0) => f.rfold_expr_struct(tmp0)?,
        syn::Expr::Try(tmp0) => f.rfold_expr_try(tmp0)?,
        syn::Expr::TryBlock(tmp0) => f.rfold_expr_tryblock(tmp0)?,
        syn::Expr::Tuple(tmp0) => f.rfold_expr_tuple(tmp0)?,
        syn::Expr::Type(tmp0) => f.rfold_expr_type(tmp0)?,
        syn::Expr::Unary(tmp0) => f.rfold_expr_unary(tmp0)?,
        syn::Expr::Unsafe(tmp0) => f.rfold_expr_unsafe(tmp0)?,
        syn::Expr::Verbatim(tmp0) => f.rfold_expr_verbatim(tmp0)?,
        syn::Expr::While(tmp0) => f.rfold_expr_while(tmp0)?,
        syn::Expr::Yield(tmp0) => f.rfold_expr_yield(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_expr_array<F>(f: &mut F, t: syn::ExprArray) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprarray(t)?;
    Ok(syn::Expr::Array(t))
}
pub fn rfold_expr_assign<F>(f: &mut F, t: syn::ExprAssign) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprassign(t)?;
    Ok(syn::Expr::Assign(t))
}
pub fn rfold_expr_assignop<F>(
    f: &mut F,
    t: syn::ExprAssignOp,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprassignop(t)?;
    Ok(syn::Expr::AssignOp(t))
}
pub fn rfold_expr_async<F>(f: &mut F, t: syn::ExprAsync) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprasync(t)?;
    Ok(syn::Expr::Async(t))
}
pub fn rfold_expr_await<F>(f: &mut F, t: syn::ExprAwait) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprawait(t)?;
    Ok(syn::Expr::Await(t))
}
pub fn rfold_expr_binary<F>(f: &mut F, t: syn::ExprBinary) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprbinary(t)?;
    Ok(syn::Expr::Binary(t))
}
pub fn rfold_expr_block<F>(f: &mut F, t: syn::ExprBlock) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprblock(t)?;
    Ok(syn::Expr::Block(t))
}
pub fn rfold_expr_box<F>(f: &mut F, t: syn::ExprBox) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprbox(t)?;
    Ok(syn::Expr::Box(t))
}
pub fn rfold_expr_break<F>(f: &mut F, t: syn::ExprBreak) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprbreak(t)?;
    Ok(syn::Expr::Break(t))
}
pub fn rfold_expr_call<F>(f: &mut F, t: syn::ExprCall) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprcall(t)?;
    Ok(syn::Expr::Call(t))
}
pub fn rfold_expr_cast<F>(f: &mut F, t: syn::ExprCast) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprcast(t)?;
    Ok(syn::Expr::Cast(t))
}
pub fn rfold_expr_closure<F>(
    f: &mut F,
    t: syn::ExprClosure,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprclosure(t)?;
    Ok(syn::Expr::Closure(t))
}
pub fn rfold_expr_continue<F>(
    f: &mut F,
    t: syn::ExprContinue,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprcontinue(t)?;
    Ok(syn::Expr::Continue(t))
}
pub fn rfold_expr_field<F>(f: &mut F, t: syn::ExprField) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprfield(t)?;
    Ok(syn::Expr::Field(t))
}
pub fn rfold_expr_forloop<F>(
    f: &mut F,
    t: syn::ExprForLoop,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprforloop(t)?;
    Ok(syn::Expr::ForLoop(t))
}
pub fn rfold_expr_group<F>(f: &mut F, t: syn::ExprGroup) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprgroup(t)?;
    Ok(syn::Expr::Group(t))
}
pub fn rfold_expr_if<F>(f: &mut F, t: syn::ExprIf) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprif(t)?;
    Ok(syn::Expr::If(t))
}
pub fn rfold_expr_index<F>(f: &mut F, t: syn::ExprIndex) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprindex(t)?;
    Ok(syn::Expr::Index(t))
}
pub fn rfold_expr_let<F>(f: &mut F, t: syn::ExprLet) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprlet(t)?;
    Ok(syn::Expr::Let(t))
}
pub fn rfold_expr_lit<F>(f: &mut F, t: syn::ExprLit) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprlit(t)?;
    Ok(syn::Expr::Lit(t))
}
pub fn rfold_expr_loop<F>(f: &mut F, t: syn::ExprLoop) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprloop(t)?;
    Ok(syn::Expr::Loop(t))
}
pub fn rfold_expr_macro<F>(f: &mut F, t: syn::ExprMacro) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprmacro(t)?;
    Ok(syn::Expr::Macro(t))
}
pub fn rfold_expr_match<F>(f: &mut F, t: syn::ExprMatch) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprmatch(t)?;
    Ok(syn::Expr::Match(t))
}
pub fn rfold_expr_methodcall<F>(
    f: &mut F,
    t: syn::ExprMethodCall,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprmethodcall(t)?;
    Ok(syn::Expr::MethodCall(t))
}
pub fn rfold_expr_paren<F>(f: &mut F, t: syn::ExprParen) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprparen(t)?;
    Ok(syn::Expr::Paren(t))
}
pub fn rfold_expr_path<F>(f: &mut F, t: syn::ExprPath) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprpath(t)?;
    Ok(syn::Expr::Path(t))
}
pub fn rfold_expr_range<F>(f: &mut F, t: syn::ExprRange) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprrange(t)?;
    Ok(syn::Expr::Range(t))
}
pub fn rfold_expr_reference<F>(
    f: &mut F,
    t: syn::ExprReference,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprreference(t)?;
    Ok(syn::Expr::Reference(t))
}
pub fn rfold_expr_repeat<F>(f: &mut F, t: syn::ExprRepeat) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprrepeat(t)?;
    Ok(syn::Expr::Repeat(t))
}
pub fn rfold_expr_return<F>(f: &mut F, t: syn::ExprReturn) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprreturn(t)?;
    Ok(syn::Expr::Return(t))
}
pub fn rfold_expr_struct<F>(f: &mut F, t: syn::ExprStruct) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprstruct(t)?;
    Ok(syn::Expr::Struct(t))
}
pub fn rfold_expr_try<F>(f: &mut F, t: syn::ExprTry) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprtry(t)?;
    Ok(syn::Expr::Try(t))
}
pub fn rfold_expr_tryblock<F>(
    f: &mut F,
    t: syn::ExprTryBlock,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprtryblock(t)?;
    Ok(syn::Expr::TryBlock(t))
}
pub fn rfold_expr_tuple<F>(f: &mut F, t: syn::ExprTuple) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprtuple(t)?;
    Ok(syn::Expr::Tuple(t))
}
pub fn rfold_expr_type<F>(f: &mut F, t: syn::ExprType) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprtype(t)?;
    Ok(syn::Expr::Type(t))
}
pub fn rfold_expr_unary<F>(f: &mut F, t: syn::ExprUnary) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprunary(t)?;
    Ok(syn::Expr::Unary(t))
}
pub fn rfold_expr_unsafe<F>(f: &mut F, t: syn::ExprUnsafe) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprunsafe(t)?;
    Ok(syn::Expr::Unsafe(t))
}
pub fn rfold_expr_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Expr::Verbatim(t))
}
pub fn rfold_expr_while<F>(f: &mut F, t: syn::ExprWhile) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_exprwhile(t)?;
    Ok(syn::Expr::While(t))
}
pub fn rfold_expr_yield<F>(f: &mut F, t: syn::ExprYield) -> Result<syn::Expr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_expryield(t)?;
    Ok(syn::Expr::Yield(t))
}
pub fn rfold_exprarray<F>(
    f: &mut F,
    t: syn::ExprArray,
) -> Result<syn::ExprArray, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.bracket_token = t.bracket_token;
    t.elems = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.elems {
            tmp.push(f.rfold_expr(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_exprassign<F>(
    f: &mut F,
    t: syn::ExprAssign,
) -> Result<syn::ExprAssign, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.left = Box::new(f.rfold_expr(*t.left)?);
    t.eq_token = t.eq_token;
    t.right = Box::new(f.rfold_expr(*t.right)?);
    Ok(t)
}
pub fn rfold_exprassignop<F>(
    f: &mut F,
    t: syn::ExprAssignOp,
) -> Result<syn::ExprAssignOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.left = Box::new(f.rfold_expr(*t.left)?);
    t.op = f.rfold_binop(t.op)?;
    t.right = Box::new(f.rfold_expr(*t.right)?);
    Ok(t)
}
pub fn rfold_exprasync<F>(
    f: &mut F,
    t: syn::ExprAsync,
) -> Result<syn::ExprAsync, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.async_token = t.async_token;
    t.capture = t.capture;
    t.block = f.rfold_block(t.block)?;
    Ok(t)
}
pub fn rfold_exprawait<F>(
    f: &mut F,
    t: syn::ExprAwait,
) -> Result<syn::ExprAwait, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.base = Box::new(f.rfold_expr(*t.base)?);
    t.dot_token = t.dot_token;
    t.await_token = t.await_token;
    Ok(t)
}
pub fn rfold_exprbinary<F>(
    f: &mut F,
    t: syn::ExprBinary,
) -> Result<syn::ExprBinary, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.left = Box::new(f.rfold_expr(*t.left)?);
    t.op = f.rfold_binop(t.op)?;
    t.right = Box::new(f.rfold_expr(*t.right)?);
    Ok(t)
}
pub fn rfold_exprblock<F>(
    f: &mut F,
    t: syn::ExprBlock,
) -> Result<syn::ExprBlock, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.label = match t.label {
        Some(o) => Some(f.rfold_label(o)?),
        None => None,
    };
    t.block = f.rfold_block(t.block)?;
    Ok(t)
}
pub fn rfold_exprbox<F>(f: &mut F, t: syn::ExprBox) -> Result<syn::ExprBox, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.box_token = t.box_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprbreak<F>(
    f: &mut F,
    t: syn::ExprBreak,
) -> Result<syn::ExprBreak, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.break_token = t.break_token;
    t.label = match t.label {
        Some(o) => Some(f.rfold_lifetime(o)?),
        None => None,
    };
    t.expr = match t.expr {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprcall<F>(f: &mut F, t: syn::ExprCall) -> Result<syn::ExprCall, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.func = Box::new(f.rfold_expr(*t.func)?);
    t.paren_token = t.paren_token;
    t.args = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.args {
            tmp.push(f.rfold_expr(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_exprcast<F>(f: &mut F, t: syn::ExprCast) -> Result<syn::ExprCast, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.as_token = t.as_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    Ok(t)
}
pub fn rfold_exprclosure<F>(
    f: &mut F,
    t: syn::ExprClosure,
) -> Result<syn::ExprClosure, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.movability = t.movability;
    t.asyncness = t.asyncness;
    t.capture = t.capture;
    t.or1_token = t.or1_token;
    t.inputs = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.inputs {
            tmp.push(f.rfold_pat(v)?);
        }
        tmp
    };
    t.or2_token = t.or2_token;
    t.output = f.rfold_returntype(t.output)?;
    t.body = Box::new(f.rfold_expr(*t.body)?);
    Ok(t)
}
pub fn rfold_exprcontinue<F>(
    f: &mut F,
    t: syn::ExprContinue,
) -> Result<syn::ExprContinue, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.continue_token = t.continue_token;
    t.label = match t.label {
        Some(o) => Some(f.rfold_lifetime(o)?),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprfield<F>(
    f: &mut F,
    t: syn::ExprField,
) -> Result<syn::ExprField, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.base = Box::new(f.rfold_expr(*t.base)?);
    t.dot_token = t.dot_token;
    t.member = f.rfold_member(t.member)?;
    Ok(t)
}
pub fn rfold_exprforloop<F>(
    f: &mut F,
    t: syn::ExprForLoop,
) -> Result<syn::ExprForLoop, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.label = match t.label {
        Some(o) => Some(f.rfold_label(o)?),
        None => None,
    };
    t.for_token = t.for_token;
    t.pat = f.rfold_pat(t.pat)?;
    t.in_token = t.in_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.body = f.rfold_block(t.body)?;
    Ok(t)
}
pub fn rfold_exprgroup<F>(
    f: &mut F,
    t: syn::ExprGroup,
) -> Result<syn::ExprGroup, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.group_token = t.group_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprif<F>(f: &mut F, t: syn::ExprIf) -> Result<syn::ExprIf, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.if_token = t.if_token;
    t.cond = Box::new(f.rfold_expr(*t.cond)?);
    t.then_branch = f.rfold_block(t.then_branch)?;
    t.else_branch = match t.else_branch {
        Some(o) => Some((o.0, Box::new(f.rfold_expr(*o.1)?))),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprindex<F>(
    f: &mut F,
    t: syn::ExprIndex,
) -> Result<syn::ExprIndex, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.bracket_token = t.bracket_token;
    t.index = Box::new(f.rfold_expr(*t.index)?);
    Ok(t)
}
pub fn rfold_exprlet<F>(f: &mut F, t: syn::ExprLet) -> Result<syn::ExprLet, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.let_token = t.let_token;
    t.pat = f.rfold_pat(t.pat)?;
    t.eq_token = t.eq_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprlit<F>(f: &mut F, t: syn::ExprLit) -> Result<syn::ExprLit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.lit = f.rfold_lit(t.lit)?;
    Ok(t)
}
pub fn rfold_exprloop<F>(f: &mut F, t: syn::ExprLoop) -> Result<syn::ExprLoop, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.label = match t.label {
        Some(o) => Some(f.rfold_label(o)?),
        None => None,
    };
    t.loop_token = t.loop_token;
    t.body = f.rfold_block(t.body)?;
    Ok(t)
}
pub fn rfold_exprmacro<F>(
    f: &mut F,
    t: syn::ExprMacro,
) -> Result<syn::ExprMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.mac = f.rfold_macro(t.mac)?;
    Ok(t)
}
pub fn rfold_exprmatch<F>(
    f: &mut F,
    t: syn::ExprMatch,
) -> Result<syn::ExprMatch, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.match_token = t.match_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.brace_token = t.brace_token;
    t.arms = {
        let mut tmp = vec![];
        for v in t.arms {
            tmp.push(f.rfold_arm(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_exprmethodcall<F>(
    f: &mut F,
    t: syn::ExprMethodCall,
) -> Result<syn::ExprMethodCall, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.receiver = Box::new(f.rfold_expr(*t.receiver)?);
    t.dot_token = t.dot_token;
    t.method = t.method;
    t.turbofish = match t.turbofish {
        Some(o) => Some(f.rfold_methodturbofish(o)?),
        None => None,
    };
    t.paren_token = t.paren_token;
    t.args = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.args {
            tmp.push(f.rfold_expr(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_exprparen<F>(
    f: &mut F,
    t: syn::ExprParen,
) -> Result<syn::ExprParen, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.paren_token = t.paren_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprpath<F>(f: &mut F, t: syn::ExprPath) -> Result<syn::ExprPath, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.qself = match t.qself {
        Some(o) => Some(f.rfold_qself(o)?),
        None => None,
    };
    t.path = f.rfold_path(t.path)?;
    Ok(t)
}
pub fn rfold_exprrange<F>(
    f: &mut F,
    t: syn::ExprRange,
) -> Result<syn::ExprRange, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.from = match t.from {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    t.limits = f.rfold_rangelimits(t.limits)?;
    t.to = match t.to {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprreference<F>(
    f: &mut F,
    t: syn::ExprReference,
) -> Result<syn::ExprReference, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.and_token = t.and_token;
    t.raw = t.raw;
    t.mutability = t.mutability;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprrepeat<F>(
    f: &mut F,
    t: syn::ExprRepeat,
) -> Result<syn::ExprRepeat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.bracket_token = t.bracket_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.semi_token = t.semi_token;
    t.len = Box::new(f.rfold_expr(*t.len)?);
    Ok(t)
}
pub fn rfold_exprreturn<F>(
    f: &mut F,
    t: syn::ExprReturn,
) -> Result<syn::ExprReturn, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.return_token = t.return_token;
    t.expr = match t.expr {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprstruct<F>(
    f: &mut F,
    t: syn::ExprStruct,
) -> Result<syn::ExprStruct, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.path = f.rfold_path(t.path)?;
    t.brace_token = t.brace_token;
    t.fields = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.fields {
            tmp.push(f.rfold_fieldvalue(v)?);
        }
        tmp
    };
    t.dot2_token = t.dot2_token;
    t.rest = match t.rest {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_exprtry<F>(f: &mut F, t: syn::ExprTry) -> Result<syn::ExprTry, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.question_token = t.question_token;
    Ok(t)
}
pub fn rfold_exprtryblock<F>(
    f: &mut F,
    t: syn::ExprTryBlock,
) -> Result<syn::ExprTryBlock, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.try_token = t.try_token;
    t.block = f.rfold_block(t.block)?;
    Ok(t)
}
pub fn rfold_exprtuple<F>(
    f: &mut F,
    t: syn::ExprTuple,
) -> Result<syn::ExprTuple, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.paren_token = t.paren_token;
    t.elems = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.elems {
            tmp.push(f.rfold_expr(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_exprtype<F>(f: &mut F, t: syn::ExprType) -> Result<syn::ExprType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.colon_token = t.colon_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    Ok(t)
}
pub fn rfold_exprunary<F>(
    f: &mut F,
    t: syn::ExprUnary,
) -> Result<syn::ExprUnary, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.op = f.rfold_unop(t.op)?;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_exprunsafe<F>(
    f: &mut F,
    t: syn::ExprUnsafe,
) -> Result<syn::ExprUnsafe, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.unsafe_token = t.unsafe_token;
    t.block = f.rfold_block(t.block)?;
    Ok(t)
}
pub fn rfold_exprwhile<F>(
    f: &mut F,
    t: syn::ExprWhile,
) -> Result<syn::ExprWhile, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.label = match t.label {
        Some(o) => Some(f.rfold_label(o)?),
        None => None,
    };
    t.while_token = t.while_token;
    t.cond = Box::new(f.rfold_expr(*t.cond)?);
    t.body = f.rfold_block(t.body)?;
    Ok(t)
}
pub fn rfold_expryield<F>(
    f: &mut F,
    t: syn::ExprYield,
) -> Result<syn::ExprYield, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.yield_token = t.yield_token;
    t.expr = match t.expr {
        Some(o) => Some(Box::new(f.rfold_expr(*o)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_field<F>(f: &mut F, t: syn::Field) -> Result<syn::Field, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = f.rfold_type(t.ty)?;
    Ok(t)
}
pub fn rfold_fieldpat<F>(f: &mut F, t: syn::FieldPat) -> Result<syn::FieldPat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.member = f.rfold_member(t.member)?;
    t.colon_token = t.colon_token;
    t.pat = Box::new(f.rfold_pat(*t.pat)?);
    Ok(t)
}
pub fn rfold_fieldvalue<F>(
    f: &mut F,
    t: syn::FieldValue,
) -> Result<syn::FieldValue, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.member = f.rfold_member(t.member)?;
    t.colon_token = t.colon_token;
    t.expr = f.rfold_expr(t.expr)?;
    Ok(t)
}
pub fn rfold_fields<F>(f: &mut F, t: syn::Fields) -> Result<syn::Fields, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Fields::Named(tmp0) => f.rfold_fields_named(tmp0)?,
        syn::Fields::Unnamed(tmp0) => f.rfold_fields_unnamed(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_fields_named<F>(
    f: &mut F,
    t: syn::FieldsNamed,
) -> Result<syn::Fields, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_fieldsnamed(t)?;
    Ok(syn::Fields::Named(t))
}
pub fn rfold_fields_unnamed<F>(
    f: &mut F,
    t: syn::FieldsUnnamed,
) -> Result<syn::Fields, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_fieldsunnamed(t)?;
    Ok(syn::Fields::Unnamed(t))
}
pub fn rfold_fieldsnamed<F>(
    f: &mut F,
    t: syn::FieldsNamed,
) -> Result<syn::FieldsNamed, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.brace_token = t.brace_token;
    t.named = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.named {
            tmp.push(f.rfold_field(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_fieldsunnamed<F>(
    f: &mut F,
    t: syn::FieldsUnnamed,
) -> Result<syn::FieldsUnnamed, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.paren_token = t.paren_token;
    t.unnamed = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.unnamed {
            tmp.push(f.rfold_field(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_file<F>(f: &mut F, t: syn::File) -> Result<syn::File, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.shebang = t.shebang;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.items = {
        let mut tmp = vec![];
        for v in t.items {
            tmp.push(f.rfold_item(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_fnarg<F>(f: &mut F, t: syn::FnArg) -> Result<syn::FnArg, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::FnArg::Receiver(tmp0) => f.rfold_fnarg_receiver(tmp0)?,
        syn::FnArg::Typed(tmp0) => f.rfold_fnarg_typed(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_fnarg_receiver<F>(
    f: &mut F,
    t: syn::Receiver,
) -> Result<syn::FnArg, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_receiver(t)?;
    Ok(syn::FnArg::Receiver(t))
}
pub fn rfold_fnarg_typed<F>(f: &mut F, t: syn::PatType) -> Result<syn::FnArg, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_pattype(t)?;
    Ok(syn::FnArg::Typed(t))
}
pub fn rfold_foreignitem<F>(
    f: &mut F,
    t: syn::ForeignItem,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::ForeignItem::Fn(tmp0) => f.rfold_foreignitem_fn(tmp0)?,
        syn::ForeignItem::Static(tmp0) => f.rfold_foreignitem_static(tmp0)?,
        syn::ForeignItem::Type(tmp0) => f.rfold_foreignitem_type(tmp0)?,
        syn::ForeignItem::Macro(tmp0) => f.rfold_foreignitem_macro(tmp0)?,
        syn::ForeignItem::Verbatim(tmp0) => f.rfold_foreignitem_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_foreignitem_fn<F>(
    f: &mut F,
    t: syn::ForeignItemFn,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_foreignitemfn(t)?;
    Ok(syn::ForeignItem::Fn(t))
}
pub fn rfold_foreignitem_static<F>(
    f: &mut F,
    t: syn::ForeignItemStatic,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_foreignitemstatic(t)?;
    Ok(syn::ForeignItem::Static(t))
}
pub fn rfold_foreignitem_type<F>(
    f: &mut F,
    t: syn::ForeignItemType,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_foreignitemtype(t)?;
    Ok(syn::ForeignItem::Type(t))
}
pub fn rfold_foreignitem_macro<F>(
    f: &mut F,
    t: syn::ForeignItemMacro,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_foreignitemmacro(t)?;
    Ok(syn::ForeignItem::Macro(t))
}
pub fn rfold_foreignitem_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::ForeignItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::ForeignItem::Verbatim(t))
}
pub fn rfold_foreignitemfn<F>(
    f: &mut F,
    t: syn::ForeignItemFn,
) -> Result<syn::ForeignItemFn, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.sig = f.rfold_signature(t.sig)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_foreignitemmacro<F>(
    f: &mut F,
    t: syn::ForeignItemMacro,
) -> Result<syn::ForeignItemMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.mac = f.rfold_macro(t.mac)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_foreignitemstatic<F>(
    f: &mut F,
    t: syn::ForeignItemStatic,
) -> Result<syn::ForeignItemStatic, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.static_token = t.static_token;
    t.mutability = t.mutability;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_foreignitemtype<F>(
    f: &mut F,
    t: syn::ForeignItemType,
) -> Result<syn::ForeignItemType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.type_token = t.type_token;
    t.ident = t.ident;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_genericargument<F>(
    f: &mut F,
    t: syn::GenericArgument,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::GenericArgument::Lifetime(tmp0) => f.rfold_genericargument_lifetime(tmp0)?,
        syn::GenericArgument::Type(tmp0) => f.rfold_genericargument_type(tmp0)?,
        syn::GenericArgument::Binding(tmp0) => f.rfold_genericargument_binding(tmp0)?,
        syn::GenericArgument::Constraint(tmp0) => f.rfold_genericargument_constraint(tmp0)?,
        syn::GenericArgument::Const(tmp0) => f.rfold_genericargument_const(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_genericargument_lifetime<F>(
    f: &mut F,
    t: syn::Lifetime,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_lifetime(t)?;
    Ok(syn::GenericArgument::Lifetime(t))
}
pub fn rfold_genericargument_type<F>(
    f: &mut F,
    t: syn::Type,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_type(t)?;
    Ok(syn::GenericArgument::Type(t))
}
pub fn rfold_genericargument_binding<F>(
    f: &mut F,
    t: syn::Binding,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_binding(t)?;
    Ok(syn::GenericArgument::Binding(t))
}
pub fn rfold_genericargument_constraint<F>(
    f: &mut F,
    t: syn::Constraint,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_constraint(t)?;
    Ok(syn::GenericArgument::Constraint(t))
}
pub fn rfold_genericargument_const<F>(
    f: &mut F,
    t: syn::Expr,
) -> Result<syn::GenericArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_expr(t)?;
    Ok(syn::GenericArgument::Const(t))
}
pub fn rfold_genericmethodargument<F>(
    f: &mut F,
    t: syn::GenericMethodArgument,
) -> Result<syn::GenericMethodArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::GenericMethodArgument::Type(tmp0) => f.rfold_genericmethodargument_type(tmp0)?,
        syn::GenericMethodArgument::Const(tmp0) => f.rfold_genericmethodargument_const(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_genericmethodargument_type<F>(
    f: &mut F,
    t: syn::Type,
) -> Result<syn::GenericMethodArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_type(t)?;
    Ok(syn::GenericMethodArgument::Type(t))
}
pub fn rfold_genericmethodargument_const<F>(
    f: &mut F,
    t: syn::Expr,
) -> Result<syn::GenericMethodArgument, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_expr(t)?;
    Ok(syn::GenericMethodArgument::Const(t))
}
pub fn rfold_genericparam<F>(
    f: &mut F,
    t: syn::GenericParam,
) -> Result<syn::GenericParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::GenericParam::Type(tmp0) => f.rfold_genericparam_type(tmp0)?,
        syn::GenericParam::Lifetime(tmp0) => f.rfold_genericparam_lifetime(tmp0)?,
        syn::GenericParam::Const(tmp0) => f.rfold_genericparam_const(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_genericparam_type<F>(
    f: &mut F,
    t: syn::TypeParam,
) -> Result<syn::GenericParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeparam(t)?;
    Ok(syn::GenericParam::Type(t))
}
pub fn rfold_genericparam_lifetime<F>(
    f: &mut F,
    t: syn::LifetimeDef,
) -> Result<syn::GenericParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_lifetimedef(t)?;
    Ok(syn::GenericParam::Lifetime(t))
}
pub fn rfold_genericparam_const<F>(
    f: &mut F,
    t: syn::ConstParam,
) -> Result<syn::GenericParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_constparam(t)?;
    Ok(syn::GenericParam::Const(t))
}
pub fn rfold_generics<F>(f: &mut F, t: syn::Generics) -> Result<syn::Generics, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lt_token = t.lt_token;
    t.params = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.params {
            tmp.push(f.rfold_genericparam(v)?);
        }
        tmp
    };
    t.gt_token = t.gt_token;
    t.where_clause = match t.where_clause {
        Some(o) => Some(f.rfold_whereclause(o)?),
        None => None,
    };
    Ok(t)
}
pub fn rfold_implitem<F>(f: &mut F, t: syn::ImplItem) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::ImplItem::Const(tmp0) => f.rfold_implitem_const(tmp0)?,
        syn::ImplItem::Method(tmp0) => f.rfold_implitem_method(tmp0)?,
        syn::ImplItem::Type(tmp0) => f.rfold_implitem_type(tmp0)?,
        syn::ImplItem::Macro(tmp0) => f.rfold_implitem_macro(tmp0)?,
        syn::ImplItem::Verbatim(tmp0) => f.rfold_implitem_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_implitem_const<F>(
    f: &mut F,
    t: syn::ImplItemConst,
) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_implitemconst(t)?;
    Ok(syn::ImplItem::Const(t))
}
pub fn rfold_implitem_method<F>(
    f: &mut F,
    t: syn::ImplItemMethod,
) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_implitemmethod(t)?;
    Ok(syn::ImplItem::Method(t))
}
pub fn rfold_implitem_type<F>(
    f: &mut F,
    t: syn::ImplItemType,
) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_implitemtype(t)?;
    Ok(syn::ImplItem::Type(t))
}
pub fn rfold_implitem_macro<F>(
    f: &mut F,
    t: syn::ImplItemMacro,
) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_implitemmacro(t)?;
    Ok(syn::ImplItem::Macro(t))
}
pub fn rfold_implitem_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::ImplItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::ImplItem::Verbatim(t))
}
pub fn rfold_implitemconst<F>(
    f: &mut F,
    t: syn::ImplItemConst,
) -> Result<syn::ImplItemConst, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.defaultness = t.defaultness;
    t.const_token = t.const_token;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = f.rfold_type(t.ty)?;
    t.eq_token = t.eq_token;
    t.expr = f.rfold_expr(t.expr)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_implitemmacro<F>(
    f: &mut F,
    t: syn::ImplItemMacro,
) -> Result<syn::ImplItemMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.mac = f.rfold_macro(t.mac)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_implitemmethod<F>(
    f: &mut F,
    t: syn::ImplItemMethod,
) -> Result<syn::ImplItemMethod, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.defaultness = t.defaultness;
    t.sig = f.rfold_signature(t.sig)?;
    t.block = f.rfold_block(t.block)?;
    Ok(t)
}
pub fn rfold_implitemtype<F>(
    f: &mut F,
    t: syn::ImplItemType,
) -> Result<syn::ImplItemType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.defaultness = t.defaultness;
    t.type_token = t.type_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.eq_token = t.eq_token;
    t.ty = f.rfold_type(t.ty)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_index<F>(_f: &mut F, t: syn::Index) -> Result<syn::Index, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.index = t.index;
    t.span = t.span;
    Ok(t)
}
pub fn rfold_item<F>(f: &mut F, t: syn::Item) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Item::Const(tmp0) => f.rfold_item_const(tmp0)?,
        syn::Item::Enum(tmp0) => f.rfold_item_enum(tmp0)?,
        syn::Item::ExternCrate(tmp0) => f.rfold_item_externcrate(tmp0)?,
        syn::Item::Fn(tmp0) => f.rfold_item_fn(tmp0)?,
        syn::Item::ForeignMod(tmp0) => f.rfold_item_foreignmod(tmp0)?,
        syn::Item::Impl(tmp0) => f.rfold_item_impl(tmp0)?,
        syn::Item::Macro(tmp0) => f.rfold_item_macro(tmp0)?,
        syn::Item::Macro2(tmp0) => f.rfold_item_macro2(tmp0)?,
        syn::Item::Mod(tmp0) => f.rfold_item_mod(tmp0)?,
        syn::Item::Static(tmp0) => f.rfold_item_static(tmp0)?,
        syn::Item::Struct(tmp0) => f.rfold_item_struct(tmp0)?,
        syn::Item::Trait(tmp0) => f.rfold_item_trait(tmp0)?,
        syn::Item::TraitAlias(tmp0) => f.rfold_item_traitalias(tmp0)?,
        syn::Item::Type(tmp0) => f.rfold_item_type(tmp0)?,
        syn::Item::Union(tmp0) => f.rfold_item_union(tmp0)?,
        syn::Item::Use(tmp0) => f.rfold_item_use(tmp0)?,
        syn::Item::Verbatim(tmp0) => f.rfold_item_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_item_const<F>(f: &mut F, t: syn::ItemConst) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemconst(t)?;
    Ok(syn::Item::Const(t))
}
pub fn rfold_item_enum<F>(f: &mut F, t: syn::ItemEnum) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemenum(t)?;
    Ok(syn::Item::Enum(t))
}
pub fn rfold_item_externcrate<F>(
    f: &mut F,
    t: syn::ItemExternCrate,
) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemexterncrate(t)?;
    Ok(syn::Item::ExternCrate(t))
}
pub fn rfold_item_fn<F>(f: &mut F, t: syn::ItemFn) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemfn(t)?;
    Ok(syn::Item::Fn(t))
}
pub fn rfold_item_foreignmod<F>(
    f: &mut F,
    t: syn::ItemForeignMod,
) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemforeignmod(t)?;
    Ok(syn::Item::ForeignMod(t))
}
pub fn rfold_item_impl<F>(f: &mut F, t: syn::ItemImpl) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemimpl(t)?;
    Ok(syn::Item::Impl(t))
}
pub fn rfold_item_macro<F>(f: &mut F, t: syn::ItemMacro) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemmacro(t)?;
    Ok(syn::Item::Macro(t))
}
pub fn rfold_item_macro2<F>(f: &mut F, t: syn::ItemMacro2) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemmacro2(t)?;
    Ok(syn::Item::Macro2(t))
}
pub fn rfold_item_mod<F>(f: &mut F, t: syn::ItemMod) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemmod(t)?;
    Ok(syn::Item::Mod(t))
}
pub fn rfold_item_static<F>(f: &mut F, t: syn::ItemStatic) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemstatic(t)?;
    Ok(syn::Item::Static(t))
}
pub fn rfold_item_struct<F>(f: &mut F, t: syn::ItemStruct) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemstruct(t)?;
    Ok(syn::Item::Struct(t))
}
pub fn rfold_item_trait<F>(f: &mut F, t: syn::ItemTrait) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemtrait(t)?;
    Ok(syn::Item::Trait(t))
}
pub fn rfold_item_traitalias<F>(
    f: &mut F,
    t: syn::ItemTraitAlias,
) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemtraitalias(t)?;
    Ok(syn::Item::TraitAlias(t))
}
pub fn rfold_item_type<F>(f: &mut F, t: syn::ItemType) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemtype(t)?;
    Ok(syn::Item::Type(t))
}
pub fn rfold_item_union<F>(f: &mut F, t: syn::ItemUnion) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemunion(t)?;
    Ok(syn::Item::Union(t))
}
pub fn rfold_item_use<F>(f: &mut F, t: syn::ItemUse) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_itemuse(t)?;
    Ok(syn::Item::Use(t))
}
pub fn rfold_item_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::Item, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Item::Verbatim(t))
}
pub fn rfold_itemconst<F>(
    f: &mut F,
    t: syn::ItemConst,
) -> Result<syn::ItemConst, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.const_token = t.const_token;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    t.eq_token = t.eq_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemenum<F>(f: &mut F, t: syn::ItemEnum) -> Result<syn::ItemEnum, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.enum_token = t.enum_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.brace_token = t.brace_token;
    t.variants = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.variants {
            tmp.push(f.rfold_variant(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_itemexterncrate<F>(
    f: &mut F,
    t: syn::ItemExternCrate,
) -> Result<syn::ItemExternCrate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.extern_token = t.extern_token;
    t.crate_token = t.crate_token;
    t.ident = t.ident;
    t.rename = match t.rename {
        Some(o) => Some((o.0, o.1)),
        None => None,
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemfn<F>(f: &mut F, t: syn::ItemFn) -> Result<syn::ItemFn, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.sig = f.rfold_signature(t.sig)?;
    t.block = Box::new(f.rfold_block(*t.block)?);
    Ok(t)
}
pub fn rfold_itemforeignmod<F>(
    f: &mut F,
    t: syn::ItemForeignMod,
) -> Result<syn::ItemForeignMod, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.abi = f.rfold_abi(t.abi)?;
    t.brace_token = t.brace_token;
    t.items = {
        let mut tmp = vec![];
        for v in t.items {
            tmp.push(f.rfold_foreignitem(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_itemimpl<F>(f: &mut F, t: syn::ItemImpl) -> Result<syn::ItemImpl, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.defaultness = t.defaultness;
    t.unsafety = t.unsafety;
    t.impl_token = t.impl_token;
    t.generics = f.rfold_generics(t.generics)?;
    t.trait_ = match t.trait_ {
        Some(o) => Some((o.0, f.rfold_path(o.1)?, o.2)),
        None => None,
    };
    t.self_ty = Box::new(f.rfold_type(*t.self_ty)?);
    t.brace_token = t.brace_token;
    t.items = {
        let mut tmp = vec![];
        for v in t.items {
            tmp.push(f.rfold_implitem(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_itemmacro<F>(
    f: &mut F,
    t: syn::ItemMacro,
) -> Result<syn::ItemMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.ident = t.ident;
    t.mac = f.rfold_macro(t.mac)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemmacro2<F>(
    f: &mut F,
    t: syn::ItemMacro2,
) -> Result<syn::ItemMacro2, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.macro_token = t.macro_token;
    t.ident = t.ident;
    t.rules = t.rules;
    Ok(t)
}
pub fn rfold_itemmod<F>(f: &mut F, t: syn::ItemMod) -> Result<syn::ItemMod, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.mod_token = t.mod_token;
    t.ident = t.ident;
    t.content = match t.content {
        Some(o) => Some((o.0, {
            let mut tmp = vec![];
            for v in o.1 {
                tmp.push(f.rfold_item(v)?);
            }
            tmp
        })),
        None => None,
    };
    t.semi = t.semi;
    Ok(t)
}
pub fn rfold_itemstatic<F>(
    f: &mut F,
    t: syn::ItemStatic,
) -> Result<syn::ItemStatic, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.static_token = t.static_token;
    t.mutability = t.mutability;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    t.eq_token = t.eq_token;
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemstruct<F>(
    f: &mut F,
    t: syn::ItemStruct,
) -> Result<syn::ItemStruct, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.struct_token = t.struct_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.fields = f.rfold_fields(t.fields)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemtrait<F>(
    f: &mut F,
    t: syn::ItemTrait,
) -> Result<syn::ItemTrait, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.unsafety = t.unsafety;
    t.auto_token = t.auto_token;
    t.trait_token = t.trait_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.colon_token = t.colon_token;
    t.supertraits = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.supertraits {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    t.brace_token = t.brace_token;
    t.items = {
        let mut tmp = vec![];
        for v in t.items {
            tmp.push(f.rfold_traititem(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_itemtraitalias<F>(
    f: &mut F,
    t: syn::ItemTraitAlias,
) -> Result<syn::ItemTraitAlias, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.trait_token = t.trait_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.eq_token = t.eq_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemtype<F>(f: &mut F, t: syn::ItemType) -> Result<syn::ItemType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.type_token = t.type_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.eq_token = t.eq_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_itemunion<F>(
    f: &mut F,
    t: syn::ItemUnion,
) -> Result<syn::ItemUnion, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.union_token = t.union_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.fields = f.rfold_fieldsnamed(t.fields)?;
    Ok(t)
}
pub fn rfold_itemuse<F>(f: &mut F, t: syn::ItemUse) -> Result<syn::ItemUse, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.vis = f.rfold_visibility(t.vis)?;
    t.use_token = t.use_token;
    t.leading_colon = t.leading_colon;
    t.tree = f.rfold_usetree(t.tree)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_label<F>(f: &mut F, t: syn::Label) -> Result<syn::Label, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.name = f.rfold_lifetime(t.name)?;
    t.colon_token = t.colon_token;
    Ok(t)
}
pub fn rfold_lifetime<F>(_f: &mut F, t: syn::Lifetime) -> Result<syn::Lifetime, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.apostrophe = t.apostrophe;
    t.ident = t.ident;
    Ok(t)
}
pub fn rfold_lifetimedef<F>(
    f: &mut F,
    t: syn::LifetimeDef,
) -> Result<syn::LifetimeDef, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.lifetime = f.rfold_lifetime(t.lifetime)?;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_lifetime(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_lit<F>(f: &mut F, t: syn::Lit) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Lit::Str(tmp0) => f.rfold_lit_str(tmp0)?,
        syn::Lit::ByteStr(tmp0) => f.rfold_lit_bytestr(tmp0)?,
        syn::Lit::Byte(tmp0) => f.rfold_lit_byte(tmp0)?,
        syn::Lit::Char(tmp0) => f.rfold_lit_char(tmp0)?,
        syn::Lit::Int(tmp0) => f.rfold_lit_int(tmp0)?,
        syn::Lit::Float(tmp0) => f.rfold_lit_float(tmp0)?,
        syn::Lit::Bool(tmp0) => f.rfold_lit_bool(tmp0)?,
        syn::Lit::Verbatim(tmp0) => f.rfold_lit_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_lit_str<F>(f: &mut F, t: syn::LitStr) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litstr(t)?;
    Ok(syn::Lit::Str(t))
}
pub fn rfold_lit_bytestr<F>(f: &mut F, t: syn::LitByteStr) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litbytestr(t)?;
    Ok(syn::Lit::ByteStr(t))
}
pub fn rfold_lit_byte<F>(f: &mut F, t: syn::LitByte) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litbyte(t)?;
    Ok(syn::Lit::Byte(t))
}
pub fn rfold_lit_char<F>(f: &mut F, t: syn::LitChar) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litchar(t)?;
    Ok(syn::Lit::Char(t))
}
pub fn rfold_lit_int<F>(f: &mut F, t: syn::LitInt) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litint(t)?;
    Ok(syn::Lit::Int(t))
}
pub fn rfold_lit_float<F>(f: &mut F, t: syn::LitFloat) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litfloat(t)?;
    Ok(syn::Lit::Float(t))
}
pub fn rfold_lit_bool<F>(f: &mut F, t: syn::LitBool) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_litbool(t)?;
    Ok(syn::Lit::Bool(t))
}
pub fn rfold_lit_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::Literal,
) -> Result<syn::Lit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Lit::Verbatim(t))
}
pub fn rfold_litbool<F>(_f: &mut F, t: syn::LitBool) -> Result<syn::LitBool, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.value = t.value;
    t.span = t.span;
    Ok(t)
}
pub fn rfold_litbyte<F>(_f: &mut F, t: syn::LitByte) -> Result<syn::LitByte, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_litbytestr<F>(
    _f: &mut F,
    t: syn::LitByteStr,
) -> Result<syn::LitByteStr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_litchar<F>(_f: &mut F, t: syn::LitChar) -> Result<syn::LitChar, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_litfloat<F>(_f: &mut F, t: syn::LitFloat) -> Result<syn::LitFloat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_litint<F>(_f: &mut F, t: syn::LitInt) -> Result<syn::LitInt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_litstr<F>(_f: &mut F, t: syn::LitStr) -> Result<syn::LitStr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    Ok(t)
}
pub fn rfold_local<F>(f: &mut F, t: syn::Local) -> Result<syn::Local, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.let_token = t.let_token;
    t.pat = f.rfold_pat(t.pat)?;
    t.init = match t.init {
        Some(o) => Some((o.0, Box::new(f.rfold_expr(*o.1)?))),
        None => None,
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_macro<F>(f: &mut F, t: syn::Macro) -> Result<syn::Macro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.path = f.rfold_path(t.path)?;
    t.bang_token = t.bang_token;
    t.delimiter = f.rfold_macrodelimiter(t.delimiter)?;
    t.tokens = t.tokens;
    Ok(t)
}
pub fn rfold_macrodelimiter<F>(
    f: &mut F,
    t: syn::MacroDelimiter,
) -> Result<syn::MacroDelimiter, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::MacroDelimiter::Paren(tmp0) => f.rfold_macrodelimiter_paren(tmp0)?,
        syn::MacroDelimiter::Brace(tmp0) => f.rfold_macrodelimiter_brace(tmp0)?,
        syn::MacroDelimiter::Bracket(tmp0) => f.rfold_macrodelimiter_bracket(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_macrodelimiter_paren<F>(
    _f: &mut F,
    t: syn::token::Paren,
) -> Result<syn::MacroDelimiter, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::MacroDelimiter::Paren(t))
}
pub fn rfold_macrodelimiter_brace<F>(
    _f: &mut F,
    t: syn::token::Brace,
) -> Result<syn::MacroDelimiter, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::MacroDelimiter::Brace(t))
}
pub fn rfold_macrodelimiter_bracket<F>(
    _f: &mut F,
    t: syn::token::Bracket,
) -> Result<syn::MacroDelimiter, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::MacroDelimiter::Bracket(t))
}
pub fn rfold_member<F>(f: &mut F, t: syn::Member) -> Result<syn::Member, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Member::Named(tmp0) => f.rfold_member_named(tmp0)?,
        syn::Member::Unnamed(tmp0) => f.rfold_member_unnamed(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_member_named<F>(
    _f: &mut F,
    t: proc_macro2::Ident,
) -> Result<syn::Member, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Member::Named(t))
}
pub fn rfold_member_unnamed<F>(f: &mut F, t: syn::Index) -> Result<syn::Member, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_index(t)?;
    Ok(syn::Member::Unnamed(t))
}
pub fn rfold_meta<F>(f: &mut F, t: syn::Meta) -> Result<syn::Meta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Meta::Path(tmp0) => f.rfold_meta_path(tmp0)?,
        syn::Meta::List(tmp0) => f.rfold_meta_list(tmp0)?,
        syn::Meta::NameValue(tmp0) => f.rfold_meta_namevalue(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_meta_path<F>(f: &mut F, t: syn::Path) -> Result<syn::Meta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_path(t)?;
    Ok(syn::Meta::Path(t))
}
pub fn rfold_meta_list<F>(f: &mut F, t: syn::MetaList) -> Result<syn::Meta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_metalist(t)?;
    Ok(syn::Meta::List(t))
}
pub fn rfold_meta_namevalue<F>(
    f: &mut F,
    t: syn::MetaNameValue,
) -> Result<syn::Meta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_metanamevalue(t)?;
    Ok(syn::Meta::NameValue(t))
}
pub fn rfold_metalist<F>(f: &mut F, t: syn::MetaList) -> Result<syn::MetaList, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.path = f.rfold_path(t.path)?;
    t.paren_token = t.paren_token;
    t.nested = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.nested {
            tmp.push(f.rfold_nestedmeta(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_metanamevalue<F>(
    f: &mut F,
    t: syn::MetaNameValue,
) -> Result<syn::MetaNameValue, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.path = f.rfold_path(t.path)?;
    t.eq_token = t.eq_token;
    t.lit = f.rfold_lit(t.lit)?;
    Ok(t)
}
pub fn rfold_methodturbofish<F>(
    f: &mut F,
    t: syn::MethodTurbofish,
) -> Result<syn::MethodTurbofish, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.colon2_token = t.colon2_token;
    t.lt_token = t.lt_token;
    t.args = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.args {
            tmp.push(f.rfold_genericmethodargument(v)?);
        }
        tmp
    };
    t.gt_token = t.gt_token;
    Ok(t)
}
pub fn rfold_nestedmeta<F>(
    f: &mut F,
    t: syn::NestedMeta,
) -> Result<syn::NestedMeta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::NestedMeta::Meta(tmp0) => f.rfold_nestedmeta_meta(tmp0)?,
        syn::NestedMeta::Lit(tmp0) => f.rfold_nestedmeta_lit(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_nestedmeta_meta<F>(
    f: &mut F,
    t: syn::Meta,
) -> Result<syn::NestedMeta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_meta(t)?;
    Ok(syn::NestedMeta::Meta(t))
}
pub fn rfold_nestedmeta_lit<F>(
    f: &mut F,
    t: syn::Lit,
) -> Result<syn::NestedMeta, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_lit(t)?;
    Ok(syn::NestedMeta::Lit(t))
}
pub fn rfold_parenthesizedgenericarguments<F>(
    f: &mut F,
    t: syn::ParenthesizedGenericArguments,
) -> Result<syn::ParenthesizedGenericArguments, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.paren_token = t.paren_token;
    t.inputs = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.inputs {
            tmp.push(f.rfold_type(v)?);
        }
        tmp
    };
    t.output = f.rfold_returntype(t.output)?;
    Ok(t)
}
pub fn rfold_pat<F>(f: &mut F, t: syn::Pat) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Pat::Box(tmp0) => f.rfold_pat_box(tmp0)?,
        syn::Pat::Ident(tmp0) => f.rfold_pat_ident(tmp0)?,
        syn::Pat::Lit(tmp0) => f.rfold_pat_lit(tmp0)?,
        syn::Pat::Macro(tmp0) => f.rfold_pat_macro(tmp0)?,
        syn::Pat::Or(tmp0) => f.rfold_pat_or(tmp0)?,
        syn::Pat::Path(tmp0) => f.rfold_pat_path(tmp0)?,
        syn::Pat::Range(tmp0) => f.rfold_pat_range(tmp0)?,
        syn::Pat::Reference(tmp0) => f.rfold_pat_reference(tmp0)?,
        syn::Pat::Rest(tmp0) => f.rfold_pat_rest(tmp0)?,
        syn::Pat::Slice(tmp0) => f.rfold_pat_slice(tmp0)?,
        syn::Pat::Struct(tmp0) => f.rfold_pat_struct(tmp0)?,
        syn::Pat::Tuple(tmp0) => f.rfold_pat_tuple(tmp0)?,
        syn::Pat::TupleStruct(tmp0) => f.rfold_pat_tuplestruct(tmp0)?,
        syn::Pat::Type(tmp0) => f.rfold_pat_type(tmp0)?,
        syn::Pat::Verbatim(tmp0) => f.rfold_pat_verbatim(tmp0)?,
        syn::Pat::Wild(tmp0) => f.rfold_pat_wild(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_pat_box<F>(f: &mut F, t: syn::PatBox) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patbox(t)?;
    Ok(syn::Pat::Box(t))
}
pub fn rfold_pat_ident<F>(f: &mut F, t: syn::PatIdent) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patident(t)?;
    Ok(syn::Pat::Ident(t))
}
pub fn rfold_pat_lit<F>(f: &mut F, t: syn::PatLit) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patlit(t)?;
    Ok(syn::Pat::Lit(t))
}
pub fn rfold_pat_macro<F>(f: &mut F, t: syn::PatMacro) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patmacro(t)?;
    Ok(syn::Pat::Macro(t))
}
pub fn rfold_pat_or<F>(f: &mut F, t: syn::PatOr) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_pator(t)?;
    Ok(syn::Pat::Or(t))
}
pub fn rfold_pat_path<F>(f: &mut F, t: syn::PatPath) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patpath(t)?;
    Ok(syn::Pat::Path(t))
}
pub fn rfold_pat_range<F>(f: &mut F, t: syn::PatRange) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patrange(t)?;
    Ok(syn::Pat::Range(t))
}
pub fn rfold_pat_reference<F>(
    f: &mut F,
    t: syn::PatReference,
) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patreference(t)?;
    Ok(syn::Pat::Reference(t))
}
pub fn rfold_pat_rest<F>(f: &mut F, t: syn::PatRest) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patrest(t)?;
    Ok(syn::Pat::Rest(t))
}
pub fn rfold_pat_slice<F>(f: &mut F, t: syn::PatSlice) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patslice(t)?;
    Ok(syn::Pat::Slice(t))
}
pub fn rfold_pat_struct<F>(f: &mut F, t: syn::PatStruct) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patstruct(t)?;
    Ok(syn::Pat::Struct(t))
}
pub fn rfold_pat_tuple<F>(f: &mut F, t: syn::PatTuple) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_pattuple(t)?;
    Ok(syn::Pat::Tuple(t))
}
pub fn rfold_pat_tuplestruct<F>(
    f: &mut F,
    t: syn::PatTupleStruct,
) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_pattuplestruct(t)?;
    Ok(syn::Pat::TupleStruct(t))
}
pub fn rfold_pat_type<F>(f: &mut F, t: syn::PatType) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_pattype(t)?;
    Ok(syn::Pat::Type(t))
}
pub fn rfold_pat_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Pat::Verbatim(t))
}
pub fn rfold_pat_wild<F>(f: &mut F, t: syn::PatWild) -> Result<syn::Pat, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_patwild(t)?;
    Ok(syn::Pat::Wild(t))
}
pub fn rfold_patbox<F>(f: &mut F, t: syn::PatBox) -> Result<syn::PatBox, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.box_token = t.box_token;
    t.pat = Box::new(f.rfold_pat(*t.pat)?);
    Ok(t)
}
pub fn rfold_patident<F>(f: &mut F, t: syn::PatIdent) -> Result<syn::PatIdent, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.by_ref = t.by_ref;
    t.mutability = t.mutability;
    t.ident = t.ident;
    t.subpat = match t.subpat {
        Some(o) => Some((o.0, Box::new(f.rfold_pat(*o.1)?))),
        None => None,
    };
    Ok(t)
}
pub fn rfold_patlit<F>(f: &mut F, t: syn::PatLit) -> Result<syn::PatLit, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.expr = Box::new(f.rfold_expr(*t.expr)?);
    Ok(t)
}
pub fn rfold_patmacro<F>(f: &mut F, t: syn::PatMacro) -> Result<syn::PatMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.mac = f.rfold_macro(t.mac)?;
    Ok(t)
}
pub fn rfold_pator<F>(f: &mut F, t: syn::PatOr) -> Result<syn::PatOr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.leading_vert = t.leading_vert;
    t.cases = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.cases {
            tmp.push(f.rfold_pat(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_patpath<F>(f: &mut F, t: syn::PatPath) -> Result<syn::PatPath, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.qself = match t.qself {
        Some(o) => Some(f.rfold_qself(o)?),
        None => None,
    };
    t.path = f.rfold_path(t.path)?;
    Ok(t)
}
pub fn rfold_patrange<F>(f: &mut F, t: syn::PatRange) -> Result<syn::PatRange, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.lo = Box::new(f.rfold_expr(*t.lo)?);
    t.limits = f.rfold_rangelimits(t.limits)?;
    t.hi = Box::new(f.rfold_expr(*t.hi)?);
    Ok(t)
}
pub fn rfold_patreference<F>(
    f: &mut F,
    t: syn::PatReference,
) -> Result<syn::PatReference, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.and_token = t.and_token;
    t.mutability = t.mutability;
    t.pat = Box::new(f.rfold_pat(*t.pat)?);
    Ok(t)
}
pub fn rfold_patrest<F>(f: &mut F, t: syn::PatRest) -> Result<syn::PatRest, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.dot2_token = t.dot2_token;
    Ok(t)
}
pub fn rfold_patslice<F>(f: &mut F, t: syn::PatSlice) -> Result<syn::PatSlice, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.bracket_token = t.bracket_token;
    t.elems = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.elems {
            tmp.push(f.rfold_pat(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_patstruct<F>(
    f: &mut F,
    t: syn::PatStruct,
) -> Result<syn::PatStruct, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.path = f.rfold_path(t.path)?;
    t.brace_token = t.brace_token;
    t.fields = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.fields {
            tmp.push(f.rfold_fieldpat(v)?);
        }
        tmp
    };
    t.dot2_token = t.dot2_token;
    Ok(t)
}
pub fn rfold_pattuple<F>(f: &mut F, t: syn::PatTuple) -> Result<syn::PatTuple, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.paren_token = t.paren_token;
    t.elems = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.elems {
            tmp.push(f.rfold_pat(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_pattuplestruct<F>(
    f: &mut F,
    t: syn::PatTupleStruct,
) -> Result<syn::PatTupleStruct, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.path = f.rfold_path(t.path)?;
    t.pat = f.rfold_pattuple(t.pat)?;
    Ok(t)
}
pub fn rfold_pattype<F>(f: &mut F, t: syn::PatType) -> Result<syn::PatType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.pat = Box::new(f.rfold_pat(*t.pat)?);
    t.colon_token = t.colon_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    Ok(t)
}
pub fn rfold_patwild<F>(f: &mut F, t: syn::PatWild) -> Result<syn::PatWild, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.underscore_token = t.underscore_token;
    Ok(t)
}
pub fn rfold_path<F>(f: &mut F, t: syn::Path) -> Result<syn::Path, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.leading_colon = t.leading_colon;
    t.segments = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.segments {
            tmp.push(f.rfold_pathsegment(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_patharguments<F>(
    f: &mut F,
    t: syn::PathArguments,
) -> Result<syn::PathArguments, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::PathArguments::AngleBracketed(tmp0) => f.rfold_patharguments_anglebracketed(tmp0)?,
        syn::PathArguments::Parenthesized(tmp0) => f.rfold_patharguments_parenthesized(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_patharguments_anglebracketed<F>(
    f: &mut F,
    t: syn::AngleBracketedGenericArguments,
) -> Result<syn::PathArguments, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_anglebracketedgenericarguments(t)?;
    Ok(syn::PathArguments::AngleBracketed(t))
}
pub fn rfold_patharguments_parenthesized<F>(
    f: &mut F,
    t: syn::ParenthesizedGenericArguments,
) -> Result<syn::PathArguments, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_parenthesizedgenericarguments(t)?;
    Ok(syn::PathArguments::Parenthesized(t))
}
pub fn rfold_pathsegment<F>(
    f: &mut F,
    t: syn::PathSegment,
) -> Result<syn::PathSegment, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    t.arguments = f.rfold_patharguments(t.arguments)?;
    Ok(t)
}
pub fn rfold_predicateeq<F>(
    f: &mut F,
    t: syn::PredicateEq,
) -> Result<syn::PredicateEq, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lhs_ty = f.rfold_type(t.lhs_ty)?;
    t.eq_token = t.eq_token;
    t.rhs_ty = f.rfold_type(t.rhs_ty)?;
    Ok(t)
}
pub fn rfold_predicatelifetime<F>(
    f: &mut F,
    t: syn::PredicateLifetime,
) -> Result<syn::PredicateLifetime, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lifetime = f.rfold_lifetime(t.lifetime)?;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_lifetime(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_predicatetype<F>(
    f: &mut F,
    t: syn::PredicateType,
) -> Result<syn::PredicateType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lifetimes = match t.lifetimes {
        Some(o) => Some(f.rfold_boundlifetimes(o)?),
        None => None,
    };
    t.bounded_ty = f.rfold_type(t.bounded_ty)?;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_qself<F>(f: &mut F, t: syn::QSelf) -> Result<syn::QSelf, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lt_token = t.lt_token;
    t.ty = Box::new(f.rfold_type(*t.ty)?);
    t.position = t.position;
    t.as_token = t.as_token;
    t.gt_token = t.gt_token;
    Ok(t)
}
pub fn rfold_rangelimits<F>(
    f: &mut F,
    t: syn::RangeLimits,
) -> Result<syn::RangeLimits, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::RangeLimits::HalfOpen(tmp0) => f.rfold_rangelimits_halfopen(tmp0)?,
        syn::RangeLimits::Closed(tmp0) => f.rfold_rangelimits_closed(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_rangelimits_halfopen<F>(
    _f: &mut F,
    t: syn::token::Dot2,
) -> Result<syn::RangeLimits, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::RangeLimits::HalfOpen(t))
}
pub fn rfold_rangelimits_closed<F>(
    _f: &mut F,
    t: syn::token::DotDotEq,
) -> Result<syn::RangeLimits, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::RangeLimits::Closed(t))
}
pub fn rfold_receiver<F>(f: &mut F, t: syn::Receiver) -> Result<syn::Receiver, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.reference = match t.reference {
        Some(o) => Some((
            o.0,
            match o.1 {
                Some(o) => Some(f.rfold_lifetime(o)?),
                None => None,
            },
        )),
        None => None,
    };
    t.mutability = t.mutability;
    t.self_token = t.self_token;
    Ok(t)
}
pub fn rfold_returntype<F>(
    f: &mut F,
    t: syn::ReturnType,
) -> Result<syn::ReturnType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::ReturnType::Type(tmp0, tmp1) => f.rfold_returntype_type((tmp0, tmp1))?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_returntype_type<F>(
    f: &mut F,
    t: (syn::token::RArrow, Box<syn::Type>),
) -> Result<syn::ReturnType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = (t.0, Box::new(f.rfold_type(*t.1)?));
    Ok(syn::ReturnType::Type(t.0, t.1))
}
pub fn rfold_signature<F>(
    f: &mut F,
    t: syn::Signature,
) -> Result<syn::Signature, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.constness = t.constness;
    t.asyncness = t.asyncness;
    t.unsafety = t.unsafety;
    t.abi = match t.abi {
        Some(o) => Some(f.rfold_abi(o)?),
        None => None,
    };
    t.fn_token = t.fn_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.paren_token = t.paren_token;
    t.inputs = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.inputs {
            tmp.push(f.rfold_fnarg(v)?);
        }
        tmp
    };
    t.variadic = match t.variadic {
        Some(o) => Some(f.rfold_variadic(o)?),
        None => None,
    };
    t.output = f.rfold_returntype(t.output)?;
    Ok(t)
}
pub fn rfold_stmt<F>(f: &mut F, t: syn::Stmt) -> Result<syn::Stmt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Stmt::Local(tmp0) => f.rfold_stmt_local(tmp0)?,
        syn::Stmt::Item(tmp0) => f.rfold_stmt_item(tmp0)?,
        syn::Stmt::Expr(tmp0) => f.rfold_stmt_expr(tmp0)?,
        syn::Stmt::Semi(tmp0, tmp1) => f.rfold_stmt_semi((tmp0, tmp1))?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_stmt_local<F>(f: &mut F, t: syn::Local) -> Result<syn::Stmt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_local(t)?;
    Ok(syn::Stmt::Local(t))
}
pub fn rfold_stmt_item<F>(f: &mut F, t: syn::Item) -> Result<syn::Stmt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_item(t)?;
    Ok(syn::Stmt::Item(t))
}
pub fn rfold_stmt_expr<F>(f: &mut F, t: syn::Expr) -> Result<syn::Stmt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_expr(t)?;
    Ok(syn::Stmt::Expr(t))
}
pub fn rfold_stmt_semi<F>(
    f: &mut F,
    t: (syn::Expr, syn::token::Semi),
) -> Result<syn::Stmt, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = (f.rfold_expr(t.0)?, t.1);
    Ok(syn::Stmt::Semi(t.0, t.1))
}
pub fn rfold_traitbound<F>(
    f: &mut F,
    t: syn::TraitBound,
) -> Result<syn::TraitBound, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.paren_token = t.paren_token;
    t.modifier = f.rfold_traitboundmodifier(t.modifier)?;
    t.lifetimes = match t.lifetimes {
        Some(o) => Some(f.rfold_boundlifetimes(o)?),
        None => None,
    };
    t.path = f.rfold_path(t.path)?;
    Ok(t)
}
pub fn rfold_traitboundmodifier<F>(
    f: &mut F,
    t: syn::TraitBoundModifier,
) -> Result<syn::TraitBoundModifier, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::TraitBoundModifier::Maybe(tmp0) => f.rfold_traitboundmodifier_maybe(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_traitboundmodifier_maybe<F>(
    _f: &mut F,
    t: syn::token::Question,
) -> Result<syn::TraitBoundModifier, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::TraitBoundModifier::Maybe(t))
}
pub fn rfold_traititem<F>(
    f: &mut F,
    t: syn::TraitItem,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::TraitItem::Const(tmp0) => f.rfold_traititem_const(tmp0)?,
        syn::TraitItem::Method(tmp0) => f.rfold_traititem_method(tmp0)?,
        syn::TraitItem::Type(tmp0) => f.rfold_traititem_type(tmp0)?,
        syn::TraitItem::Macro(tmp0) => f.rfold_traititem_macro(tmp0)?,
        syn::TraitItem::Verbatim(tmp0) => f.rfold_traititem_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_traititem_const<F>(
    f: &mut F,
    t: syn::TraitItemConst,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_traititemconst(t)?;
    Ok(syn::TraitItem::Const(t))
}
pub fn rfold_traititem_method<F>(
    f: &mut F,
    t: syn::TraitItemMethod,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_traititemmethod(t)?;
    Ok(syn::TraitItem::Method(t))
}
pub fn rfold_traititem_type<F>(
    f: &mut F,
    t: syn::TraitItemType,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_traititemtype(t)?;
    Ok(syn::TraitItem::Type(t))
}
pub fn rfold_traititem_macro<F>(
    f: &mut F,
    t: syn::TraitItemMacro,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_traititemmacro(t)?;
    Ok(syn::TraitItem::Macro(t))
}
pub fn rfold_traititem_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::TraitItem, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::TraitItem::Verbatim(t))
}
pub fn rfold_traititemconst<F>(
    f: &mut F,
    t: syn::TraitItemConst,
) -> Result<syn::TraitItemConst, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.const_token = t.const_token;
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.ty = f.rfold_type(t.ty)?;
    t.default = match t.default {
        Some(o) => Some((o.0, f.rfold_expr(o.1)?)),
        None => None,
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_traititemmacro<F>(
    f: &mut F,
    t: syn::TraitItemMacro,
) -> Result<syn::TraitItemMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.mac = f.rfold_macro(t.mac)?;
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_traititemmethod<F>(
    f: &mut F,
    t: syn::TraitItemMethod,
) -> Result<syn::TraitItemMethod, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.sig = f.rfold_signature(t.sig)?;
    t.default = match t.default {
        Some(o) => Some(f.rfold_block(o)?),
        None => None,
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_traititemtype<F>(
    f: &mut F,
    t: syn::TraitItemType,
) -> Result<syn::TraitItemType, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.type_token = t.type_token;
    t.ident = t.ident;
    t.generics = f.rfold_generics(t.generics)?;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    t.default = match t.default {
        Some(o) => Some((o.0, f.rfold_type(o.1)?)),
        None => None,
    };
    t.semi_token = t.semi_token;
    Ok(t)
}
pub fn rfold_type<F>(f: &mut F, t: syn::Type) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Type::Array(tmp0) => f.rfold_type_array(tmp0)?,
        syn::Type::BareFn(tmp0) => f.rfold_type_barefn(tmp0)?,
        syn::Type::Group(tmp0) => f.rfold_type_group(tmp0)?,
        syn::Type::ImplTrait(tmp0) => f.rfold_type_impltrait(tmp0)?,
        syn::Type::Infer(tmp0) => f.rfold_type_infer(tmp0)?,
        syn::Type::Macro(tmp0) => f.rfold_type_macro(tmp0)?,
        syn::Type::Never(tmp0) => f.rfold_type_never(tmp0)?,
        syn::Type::Paren(tmp0) => f.rfold_type_paren(tmp0)?,
        syn::Type::Path(tmp0) => f.rfold_type_path(tmp0)?,
        syn::Type::Ptr(tmp0) => f.rfold_type_ptr(tmp0)?,
        syn::Type::Reference(tmp0) => f.rfold_type_reference(tmp0)?,
        syn::Type::Slice(tmp0) => f.rfold_type_slice(tmp0)?,
        syn::Type::TraitObject(tmp0) => f.rfold_type_traitobject(tmp0)?,
        syn::Type::Tuple(tmp0) => f.rfold_type_tuple(tmp0)?,
        syn::Type::Verbatim(tmp0) => f.rfold_type_verbatim(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_type_array<F>(f: &mut F, t: syn::TypeArray) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typearray(t)?;
    Ok(syn::Type::Array(t))
}
pub fn rfold_type_barefn<F>(f: &mut F, t: syn::TypeBareFn) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typebarefn(t)?;
    Ok(syn::Type::BareFn(t))
}
pub fn rfold_type_group<F>(f: &mut F, t: syn::TypeGroup) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typegroup(t)?;
    Ok(syn::Type::Group(t))
}
pub fn rfold_type_impltrait<F>(
    f: &mut F,
    t: syn::TypeImplTrait,
) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeimpltrait(t)?;
    Ok(syn::Type::ImplTrait(t))
}
pub fn rfold_type_infer<F>(f: &mut F, t: syn::TypeInfer) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeinfer(t)?;
    Ok(syn::Type::Infer(t))
}
pub fn rfold_type_macro<F>(f: &mut F, t: syn::TypeMacro) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typemacro(t)?;
    Ok(syn::Type::Macro(t))
}
pub fn rfold_type_never<F>(f: &mut F, t: syn::TypeNever) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typenever(t)?;
    Ok(syn::Type::Never(t))
}
pub fn rfold_type_paren<F>(f: &mut F, t: syn::TypeParen) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeparen(t)?;
    Ok(syn::Type::Paren(t))
}
pub fn rfold_type_path<F>(f: &mut F, t: syn::TypePath) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typepath(t)?;
    Ok(syn::Type::Path(t))
}
pub fn rfold_type_ptr<F>(f: &mut F, t: syn::TypePtr) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeptr(t)?;
    Ok(syn::Type::Ptr(t))
}
pub fn rfold_type_reference<F>(
    f: &mut F,
    t: syn::TypeReference,
) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typereference(t)?;
    Ok(syn::Type::Reference(t))
}
pub fn rfold_type_slice<F>(f: &mut F, t: syn::TypeSlice) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typeslice(t)?;
    Ok(syn::Type::Slice(t))
}
pub fn rfold_type_traitobject<F>(
    f: &mut F,
    t: syn::TypeTraitObject,
) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typetraitobject(t)?;
    Ok(syn::Type::TraitObject(t))
}
pub fn rfold_type_tuple<F>(f: &mut F, t: syn::TypeTuple) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_typetuple(t)?;
    Ok(syn::Type::Tuple(t))
}
pub fn rfold_type_verbatim<F>(
    _f: &mut F,
    t: proc_macro2::TokenStream,
) -> Result<syn::Type, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::Type::Verbatim(t))
}
pub fn rfold_typearray<F>(
    f: &mut F,
    t: syn::TypeArray,
) -> Result<syn::TypeArray, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.bracket_token = t.bracket_token;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    t.semi_token = t.semi_token;
    t.len = f.rfold_expr(t.len)?;
    Ok(t)
}
pub fn rfold_typebarefn<F>(
    f: &mut F,
    t: syn::TypeBareFn,
) -> Result<syn::TypeBareFn, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.lifetimes = match t.lifetimes {
        Some(o) => Some(f.rfold_boundlifetimes(o)?),
        None => None,
    };
    t.unsafety = t.unsafety;
    t.abi = match t.abi {
        Some(o) => Some(f.rfold_abi(o)?),
        None => None,
    };
    t.fn_token = t.fn_token;
    t.paren_token = t.paren_token;
    t.inputs = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.inputs {
            tmp.push(f.rfold_barefnarg(v)?);
        }
        tmp
    };
    t.variadic = match t.variadic {
        Some(o) => Some(f.rfold_variadic(o)?),
        None => None,
    };
    t.output = f.rfold_returntype(t.output)?;
    Ok(t)
}
pub fn rfold_typegroup<F>(
    f: &mut F,
    t: syn::TypeGroup,
) -> Result<syn::TypeGroup, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.group_token = t.group_token;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    Ok(t)
}
pub fn rfold_typeimpltrait<F>(
    f: &mut F,
    t: syn::TypeImplTrait,
) -> Result<syn::TypeImplTrait, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.impl_token = t.impl_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_typeinfer<F>(
    _f: &mut F,
    t: syn::TypeInfer,
) -> Result<syn::TypeInfer, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.underscore_token = t.underscore_token;
    Ok(t)
}
pub fn rfold_typemacro<F>(
    f: &mut F,
    t: syn::TypeMacro,
) -> Result<syn::TypeMacro, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.mac = f.rfold_macro(t.mac)?;
    Ok(t)
}
pub fn rfold_typenever<F>(
    _f: &mut F,
    t: syn::TypeNever,
) -> Result<syn::TypeNever, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.bang_token = t.bang_token;
    Ok(t)
}
pub fn rfold_typeparam<F>(
    f: &mut F,
    t: syn::TypeParam,
) -> Result<syn::TypeParam, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.ident = t.ident;
    t.colon_token = t.colon_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    t.eq_token = t.eq_token;
    t.default = match t.default {
        Some(o) => Some(f.rfold_type(o)?),
        None => None,
    };
    Ok(t)
}
pub fn rfold_typeparambound<F>(
    f: &mut F,
    t: syn::TypeParamBound,
) -> Result<syn::TypeParamBound, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::TypeParamBound::Trait(tmp0) => f.rfold_typeparambound_trait(tmp0)?,
        syn::TypeParamBound::Lifetime(tmp0) => f.rfold_typeparambound_lifetime(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_typeparambound_trait<F>(
    f: &mut F,
    t: syn::TraitBound,
) -> Result<syn::TypeParamBound, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_traitbound(t)?;
    Ok(syn::TypeParamBound::Trait(t))
}
pub fn rfold_typeparambound_lifetime<F>(
    f: &mut F,
    t: syn::Lifetime,
) -> Result<syn::TypeParamBound, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_lifetime(t)?;
    Ok(syn::TypeParamBound::Lifetime(t))
}
pub fn rfold_typeparen<F>(
    f: &mut F,
    t: syn::TypeParen,
) -> Result<syn::TypeParen, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.paren_token = t.paren_token;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    Ok(t)
}
pub fn rfold_typepath<F>(f: &mut F, t: syn::TypePath) -> Result<syn::TypePath, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.qself = match t.qself {
        Some(o) => Some(f.rfold_qself(o)?),
        None => None,
    };
    t.path = f.rfold_path(t.path)?;
    Ok(t)
}
pub fn rfold_typeptr<F>(f: &mut F, t: syn::TypePtr) -> Result<syn::TypePtr, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.star_token = t.star_token;
    t.const_token = t.const_token;
    t.mutability = t.mutability;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    Ok(t)
}
pub fn rfold_typereference<F>(
    f: &mut F,
    t: syn::TypeReference,
) -> Result<syn::TypeReference, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.and_token = t.and_token;
    t.lifetime = match t.lifetime {
        Some(o) => Some(f.rfold_lifetime(o)?),
        None => None,
    };
    t.mutability = t.mutability;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    Ok(t)
}
pub fn rfold_typeslice<F>(
    f: &mut F,
    t: syn::TypeSlice,
) -> Result<syn::TypeSlice, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.bracket_token = t.bracket_token;
    t.elem = Box::new(f.rfold_type(*t.elem)?);
    Ok(t)
}
pub fn rfold_typetraitobject<F>(
    f: &mut F,
    t: syn::TypeTraitObject,
) -> Result<syn::TypeTraitObject, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.dyn_token = t.dyn_token;
    t.bounds = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.bounds {
            tmp.push(f.rfold_typeparambound(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_typetuple<F>(
    f: &mut F,
    t: syn::TypeTuple,
) -> Result<syn::TypeTuple, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.paren_token = t.paren_token;
    t.elems = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.elems {
            tmp.push(f.rfold_type(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_unop<F>(f: &mut F, t: syn::UnOp) -> Result<syn::UnOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::UnOp::Deref(tmp0) => f.rfold_unop_deref(tmp0)?,
        syn::UnOp::Not(tmp0) => f.rfold_unop_not(tmp0)?,
        syn::UnOp::Neg(tmp0) => f.rfold_unop_neg(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_unop_deref<F>(
    _f: &mut F,
    t: syn::token::Star,
) -> Result<syn::UnOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::UnOp::Deref(t))
}
pub fn rfold_unop_not<F>(_f: &mut F, t: syn::token::Bang) -> Result<syn::UnOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::UnOp::Not(t))
}
pub fn rfold_unop_neg<F>(_f: &mut F, t: syn::token::Sub) -> Result<syn::UnOp, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = t;
    Ok(syn::UnOp::Neg(t))
}
pub fn rfold_useglob<F>(_f: &mut F, t: syn::UseGlob) -> Result<syn::UseGlob, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.star_token = t.star_token;
    Ok(t)
}
pub fn rfold_usegroup<F>(f: &mut F, t: syn::UseGroup) -> Result<syn::UseGroup, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.brace_token = t.brace_token;
    t.items = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.items {
            tmp.push(f.rfold_usetree(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_usename<F>(_f: &mut F, t: syn::UseName) -> Result<syn::UseName, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    Ok(t)
}
pub fn rfold_usepath<F>(f: &mut F, t: syn::UsePath) -> Result<syn::UsePath, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    t.colon2_token = t.colon2_token;
    t.tree = Box::new(f.rfold_usetree(*t.tree)?);
    Ok(t)
}
pub fn rfold_userename<F>(
    _f: &mut F,
    t: syn::UseRename,
) -> Result<syn::UseRename, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.ident = t.ident;
    t.as_token = t.as_token;
    t.rename = t.rename;
    Ok(t)
}
pub fn rfold_usetree<F>(f: &mut F, t: syn::UseTree) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::UseTree::Path(tmp0) => f.rfold_usetree_path(tmp0)?,
        syn::UseTree::Name(tmp0) => f.rfold_usetree_name(tmp0)?,
        syn::UseTree::Rename(tmp0) => f.rfold_usetree_rename(tmp0)?,
        syn::UseTree::Glob(tmp0) => f.rfold_usetree_glob(tmp0)?,
        syn::UseTree::Group(tmp0) => f.rfold_usetree_group(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_usetree_path<F>(
    f: &mut F,
    t: syn::UsePath,
) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_usepath(t)?;
    Ok(syn::UseTree::Path(t))
}
pub fn rfold_usetree_name<F>(
    f: &mut F,
    t: syn::UseName,
) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_usename(t)?;
    Ok(syn::UseTree::Name(t))
}
pub fn rfold_usetree_rename<F>(
    f: &mut F,
    t: syn::UseRename,
) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_userename(t)?;
    Ok(syn::UseTree::Rename(t))
}
pub fn rfold_usetree_glob<F>(
    f: &mut F,
    t: syn::UseGlob,
) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_useglob(t)?;
    Ok(syn::UseTree::Glob(t))
}
pub fn rfold_usetree_group<F>(
    f: &mut F,
    t: syn::UseGroup,
) -> Result<syn::UseTree, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_usegroup(t)?;
    Ok(syn::UseTree::Group(t))
}
pub fn rfold_variadic<F>(f: &mut F, t: syn::Variadic) -> Result<syn::Variadic, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.dots = t.dots;
    Ok(t)
}
pub fn rfold_variant<F>(f: &mut F, t: syn::Variant) -> Result<syn::Variant, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.attrs = {
        let mut tmp = vec![];
        for v in t.attrs {
            tmp.push(f.rfold_attribute(v)?);
        }
        tmp
    };
    t.ident = t.ident;
    t.fields = f.rfold_fields(t.fields)?;
    t.discriminant = match t.discriminant {
        Some(o) => Some((o.0, f.rfold_expr(o.1)?)),
        None => None,
    };
    Ok(t)
}
pub fn rfold_viscrate<F>(_f: &mut F, t: syn::VisCrate) -> Result<syn::VisCrate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.crate_token = t.crate_token;
    Ok(t)
}
pub fn rfold_vispublic<F>(
    _f: &mut F,
    t: syn::VisPublic,
) -> Result<syn::VisPublic, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.pub_token = t.pub_token;
    Ok(t)
}
pub fn rfold_visrestricted<F>(
    f: &mut F,
    t: syn::VisRestricted,
) -> Result<syn::VisRestricted, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.pub_token = t.pub_token;
    t.paren_token = t.paren_token;
    t.in_token = t.in_token;
    t.path = Box::new(f.rfold_path(*t.path)?);
    Ok(t)
}
pub fn rfold_visibility<F>(
    f: &mut F,
    t: syn::Visibility,
) -> Result<syn::Visibility, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::Visibility::Public(tmp0) => f.rfold_visibility_public(tmp0)?,
        syn::Visibility::Crate(tmp0) => f.rfold_visibility_crate(tmp0)?,
        syn::Visibility::Restricted(tmp0) => f.rfold_visibility_restricted(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_visibility_public<F>(
    f: &mut F,
    t: syn::VisPublic,
) -> Result<syn::Visibility, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_vispublic(t)?;
    Ok(syn::Visibility::Public(t))
}
pub fn rfold_visibility_crate<F>(
    f: &mut F,
    t: syn::VisCrate,
) -> Result<syn::Visibility, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_viscrate(t)?;
    Ok(syn::Visibility::Crate(t))
}
pub fn rfold_visibility_restricted<F>(
    f: &mut F,
    t: syn::VisRestricted,
) -> Result<syn::Visibility, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_visrestricted(t)?;
    Ok(syn::Visibility::Restricted(t))
}
pub fn rfold_whereclause<F>(
    f: &mut F,
    t: syn::WhereClause,
) -> Result<syn::WhereClause, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let mut t = t;
    t.where_token = t.where_token;
    t.predicates = {
        let mut tmp = syn::punctuated::Punctuated::new();
        for v in t.predicates {
            tmp.push(f.rfold_wherepredicate(v)?);
        }
        tmp
    };
    Ok(t)
}
pub fn rfold_wherepredicate<F>(
    f: &mut F,
    t: syn::WherePredicate,
) -> Result<syn::WherePredicate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = match t {
        syn::WherePredicate::Type(tmp0) => f.rfold_wherepredicate_type(tmp0)?,
        syn::WherePredicate::Lifetime(tmp0) => f.rfold_wherepredicate_lifetime(tmp0)?,
        syn::WherePredicate::Eq(tmp0) => f.rfold_wherepredicate_eq(tmp0)?,
        _ => t,
    };
    Ok(t)
}
pub fn rfold_wherepredicate_type<F>(
    f: &mut F,
    t: syn::PredicateType,
) -> Result<syn::WherePredicate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_predicatetype(t)?;
    Ok(syn::WherePredicate::Type(t))
}
pub fn rfold_wherepredicate_lifetime<F>(
    f: &mut F,
    t: syn::PredicateLifetime,
) -> Result<syn::WherePredicate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_predicatelifetime(t)?;
    Ok(syn::WherePredicate::Lifetime(t))
}
pub fn rfold_wherepredicate_eq<F>(
    f: &mut F,
    t: syn::PredicateEq,
) -> Result<syn::WherePredicate, <F as RFold>::Error>
where
    F: RFold + ?Sized,
{
    let t = f.rfold_predicateeq(t)?;
    Ok(syn::WherePredicate::Eq(t))
}
