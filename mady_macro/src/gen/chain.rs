// codegen file by version 0.1.0
// don't edit this

/// chain of responsibility trait
/// it is a call & return trait
#[allow(unused)]
trait Chain {
    type Input;
    fn chain_abi(&mut self, c: &mut Self::Input, t: syn::Abi) -> syn::Abi {
        t
    }
    fn chain_anglebracketedgenericarguments(
        &mut self,
        c: &mut Self::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> syn::AngleBracketedGenericArguments {
        t
    }
    fn chain_arm(&mut self, c: &mut Self::Input, t: syn::Arm) -> syn::Arm {
        t
    }
    fn chain_attrstyle_inner(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Bang,
    ) -> syn::AttrStyle {
        syn::AttrStyle::Inner(t)
    }
    fn chain_attribute(&mut self, c: &mut Self::Input, t: syn::Attribute) -> syn::Attribute {
        t
    }
    fn chain_barefnarg(&mut self, c: &mut Self::Input, t: syn::BareFnArg) -> syn::BareFnArg {
        t
    }
    fn chain_binop_add(&mut self, c: &mut Self::Input, t: syn::token::Add) -> syn::BinOp {
        syn::BinOp::Add(t)
    }
    fn chain_binop_sub(&mut self, c: &mut Self::Input, t: syn::token::Sub) -> syn::BinOp {
        syn::BinOp::Sub(t)
    }
    fn chain_binop_mul(&mut self, c: &mut Self::Input, t: syn::token::Star) -> syn::BinOp {
        syn::BinOp::Mul(t)
    }
    fn chain_binop_div(&mut self, c: &mut Self::Input, t: syn::token::Div) -> syn::BinOp {
        syn::BinOp::Div(t)
    }
    fn chain_binop_rem(&mut self, c: &mut Self::Input, t: syn::token::Rem) -> syn::BinOp {
        syn::BinOp::Rem(t)
    }
    fn chain_binop_and(&mut self, c: &mut Self::Input, t: syn::token::AndAnd) -> syn::BinOp {
        syn::BinOp::And(t)
    }
    fn chain_binop_or(&mut self, c: &mut Self::Input, t: syn::token::OrOr) -> syn::BinOp {
        syn::BinOp::Or(t)
    }
    fn chain_binop_bitxor(&mut self, c: &mut Self::Input, t: syn::token::Caret) -> syn::BinOp {
        syn::BinOp::BitXor(t)
    }
    fn chain_binop_bitand(&mut self, c: &mut Self::Input, t: syn::token::And) -> syn::BinOp {
        syn::BinOp::BitAnd(t)
    }
    fn chain_binop_bitor(&mut self, c: &mut Self::Input, t: syn::token::Or) -> syn::BinOp {
        syn::BinOp::BitOr(t)
    }
    fn chain_binop_shl(&mut self, c: &mut Self::Input, t: syn::token::Shl) -> syn::BinOp {
        syn::BinOp::Shl(t)
    }
    fn chain_binop_shr(&mut self, c: &mut Self::Input, t: syn::token::Shr) -> syn::BinOp {
        syn::BinOp::Shr(t)
    }
    fn chain_binop_eq(&mut self, c: &mut Self::Input, t: syn::token::EqEq) -> syn::BinOp {
        syn::BinOp::Eq(t)
    }
    fn chain_binop_lt(&mut self, c: &mut Self::Input, t: syn::token::Lt) -> syn::BinOp {
        syn::BinOp::Lt(t)
    }
    fn chain_binop_le(&mut self, c: &mut Self::Input, t: syn::token::Le) -> syn::BinOp {
        syn::BinOp::Le(t)
    }
    fn chain_binop_ne(&mut self, c: &mut Self::Input, t: syn::token::Ne) -> syn::BinOp {
        syn::BinOp::Ne(t)
    }
    fn chain_binop_ge(&mut self, c: &mut Self::Input, t: syn::token::Ge) -> syn::BinOp {
        syn::BinOp::Ge(t)
    }
    fn chain_binop_gt(&mut self, c: &mut Self::Input, t: syn::token::Gt) -> syn::BinOp {
        syn::BinOp::Gt(t)
    }
    fn chain_binop_addeq(&mut self, c: &mut Self::Input, t: syn::token::AddEq) -> syn::BinOp {
        syn::BinOp::AddEq(t)
    }
    fn chain_binop_subeq(&mut self, c: &mut Self::Input, t: syn::token::SubEq) -> syn::BinOp {
        syn::BinOp::SubEq(t)
    }
    fn chain_binop_muleq(&mut self, c: &mut Self::Input, t: syn::token::MulEq) -> syn::BinOp {
        syn::BinOp::MulEq(t)
    }
    fn chain_binop_diveq(&mut self, c: &mut Self::Input, t: syn::token::DivEq) -> syn::BinOp {
        syn::BinOp::DivEq(t)
    }
    fn chain_binop_remeq(&mut self, c: &mut Self::Input, t: syn::token::RemEq) -> syn::BinOp {
        syn::BinOp::RemEq(t)
    }
    fn chain_binop_bitxoreq(&mut self, c: &mut Self::Input, t: syn::token::CaretEq) -> syn::BinOp {
        syn::BinOp::BitXorEq(t)
    }
    fn chain_binop_bitandeq(&mut self, c: &mut Self::Input, t: syn::token::AndEq) -> syn::BinOp {
        syn::BinOp::BitAndEq(t)
    }
    fn chain_binop_bitoreq(&mut self, c: &mut Self::Input, t: syn::token::OrEq) -> syn::BinOp {
        syn::BinOp::BitOrEq(t)
    }
    fn chain_binop_shleq(&mut self, c: &mut Self::Input, t: syn::token::ShlEq) -> syn::BinOp {
        syn::BinOp::ShlEq(t)
    }
    fn chain_binop_shreq(&mut self, c: &mut Self::Input, t: syn::token::ShrEq) -> syn::BinOp {
        syn::BinOp::ShrEq(t)
    }
    fn chain_binding(&mut self, c: &mut Self::Input, t: syn::Binding) -> syn::Binding {
        t
    }
    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> syn::Block {
        t
    }
    fn chain_boundlifetimes(
        &mut self,
        c: &mut Self::Input,
        t: syn::BoundLifetimes,
    ) -> syn::BoundLifetimes {
        t
    }
    fn chain_constparam(&mut self, c: &mut Self::Input, t: syn::ConstParam) -> syn::ConstParam {
        t
    }
    fn chain_constraint(&mut self, c: &mut Self::Input, t: syn::Constraint) -> syn::Constraint {
        t
    }
    fn chain_data_struct(&mut self, c: &mut Self::Input, t: syn::DataStruct) -> syn::Data {
        syn::Data::Struct(t)
    }
    fn chain_data_enum(&mut self, c: &mut Self::Input, t: syn::DataEnum) -> syn::Data {
        syn::Data::Enum(t)
    }
    fn chain_data_union(&mut self, c: &mut Self::Input, t: syn::DataUnion) -> syn::Data {
        syn::Data::Union(t)
    }
    fn chain_dataenum(&mut self, c: &mut Self::Input, t: syn::DataEnum) -> syn::DataEnum {
        t
    }
    fn chain_datastruct(&mut self, c: &mut Self::Input, t: syn::DataStruct) -> syn::DataStruct {
        t
    }
    fn chain_dataunion(&mut self, c: &mut Self::Input, t: syn::DataUnion) -> syn::DataUnion {
        t
    }
    fn chain_deriveinput(&mut self, c: &mut Self::Input, t: syn::DeriveInput) -> syn::DeriveInput {
        t
    }
    fn chain_expr_array(&mut self, c: &mut Self::Input, t: syn::ExprArray) -> syn::Expr {
        syn::Expr::Array(t)
    }
    fn chain_expr_assign(&mut self, c: &mut Self::Input, t: syn::ExprAssign) -> syn::Expr {
        syn::Expr::Assign(t)
    }
    fn chain_expr_assignop(&mut self, c: &mut Self::Input, t: syn::ExprAssignOp) -> syn::Expr {
        syn::Expr::AssignOp(t)
    }
    fn chain_expr_async(&mut self, c: &mut Self::Input, t: syn::ExprAsync) -> syn::Expr {
        syn::Expr::Async(t)
    }
    fn chain_expr_await(&mut self, c: &mut Self::Input, t: syn::ExprAwait) -> syn::Expr {
        syn::Expr::Await(t)
    }
    fn chain_expr_binary(&mut self, c: &mut Self::Input, t: syn::ExprBinary) -> syn::Expr {
        syn::Expr::Binary(t)
    }
    fn chain_expr_block(&mut self, c: &mut Self::Input, t: syn::ExprBlock) -> syn::Expr {
        syn::Expr::Block(t)
    }
    fn chain_expr_box(&mut self, c: &mut Self::Input, t: syn::ExprBox) -> syn::Expr {
        syn::Expr::Box(t)
    }
    fn chain_expr_break(&mut self, c: &mut Self::Input, t: syn::ExprBreak) -> syn::Expr {
        syn::Expr::Break(t)
    }
    fn chain_expr_call(&mut self, c: &mut Self::Input, t: syn::ExprCall) -> syn::Expr {
        syn::Expr::Call(t)
    }
    fn chain_expr_cast(&mut self, c: &mut Self::Input, t: syn::ExprCast) -> syn::Expr {
        syn::Expr::Cast(t)
    }
    fn chain_expr_closure(&mut self, c: &mut Self::Input, t: syn::ExprClosure) -> syn::Expr {
        syn::Expr::Closure(t)
    }
    fn chain_expr_continue(&mut self, c: &mut Self::Input, t: syn::ExprContinue) -> syn::Expr {
        syn::Expr::Continue(t)
    }
    fn chain_expr_field(&mut self, c: &mut Self::Input, t: syn::ExprField) -> syn::Expr {
        syn::Expr::Field(t)
    }
    fn chain_expr_forloop(&mut self, c: &mut Self::Input, t: syn::ExprForLoop) -> syn::Expr {
        syn::Expr::ForLoop(t)
    }
    fn chain_expr_group(&mut self, c: &mut Self::Input, t: syn::ExprGroup) -> syn::Expr {
        syn::Expr::Group(t)
    }
    fn chain_expr_if(&mut self, c: &mut Self::Input, t: syn::ExprIf) -> syn::Expr {
        syn::Expr::If(t)
    }
    fn chain_expr_index(&mut self, c: &mut Self::Input, t: syn::ExprIndex) -> syn::Expr {
        syn::Expr::Index(t)
    }
    fn chain_expr_let(&mut self, c: &mut Self::Input, t: syn::ExprLet) -> syn::Expr {
        syn::Expr::Let(t)
    }
    fn chain_expr_lit(&mut self, c: &mut Self::Input, t: syn::ExprLit) -> syn::Expr {
        syn::Expr::Lit(t)
    }
    fn chain_expr_loop(&mut self, c: &mut Self::Input, t: syn::ExprLoop) -> syn::Expr {
        syn::Expr::Loop(t)
    }
    fn chain_expr_macro(&mut self, c: &mut Self::Input, t: syn::ExprMacro) -> syn::Expr {
        syn::Expr::Macro(t)
    }
    fn chain_expr_match(&mut self, c: &mut Self::Input, t: syn::ExprMatch) -> syn::Expr {
        syn::Expr::Match(t)
    }
    fn chain_expr_methodcall(&mut self, c: &mut Self::Input, t: syn::ExprMethodCall) -> syn::Expr {
        syn::Expr::MethodCall(t)
    }
    fn chain_expr_paren(&mut self, c: &mut Self::Input, t: syn::ExprParen) -> syn::Expr {
        syn::Expr::Paren(t)
    }
    fn chain_expr_path(&mut self, c: &mut Self::Input, t: syn::ExprPath) -> syn::Expr {
        syn::Expr::Path(t)
    }
    fn chain_expr_range(&mut self, c: &mut Self::Input, t: syn::ExprRange) -> syn::Expr {
        syn::Expr::Range(t)
    }
    fn chain_expr_reference(&mut self, c: &mut Self::Input, t: syn::ExprReference) -> syn::Expr {
        syn::Expr::Reference(t)
    }
    fn chain_expr_repeat(&mut self, c: &mut Self::Input, t: syn::ExprRepeat) -> syn::Expr {
        syn::Expr::Repeat(t)
    }
    fn chain_expr_return(&mut self, c: &mut Self::Input, t: syn::ExprReturn) -> syn::Expr {
        syn::Expr::Return(t)
    }
    fn chain_expr_struct(&mut self, c: &mut Self::Input, t: syn::ExprStruct) -> syn::Expr {
        syn::Expr::Struct(t)
    }
    fn chain_expr_try(&mut self, c: &mut Self::Input, t: syn::ExprTry) -> syn::Expr {
        syn::Expr::Try(t)
    }
    fn chain_expr_tryblock(&mut self, c: &mut Self::Input, t: syn::ExprTryBlock) -> syn::Expr {
        syn::Expr::TryBlock(t)
    }
    fn chain_expr_tuple(&mut self, c: &mut Self::Input, t: syn::ExprTuple) -> syn::Expr {
        syn::Expr::Tuple(t)
    }
    fn chain_expr_type(&mut self, c: &mut Self::Input, t: syn::ExprType) -> syn::Expr {
        syn::Expr::Type(t)
    }
    fn chain_expr_unary(&mut self, c: &mut Self::Input, t: syn::ExprUnary) -> syn::Expr {
        syn::Expr::Unary(t)
    }
    fn chain_expr_unsafe(&mut self, c: &mut Self::Input, t: syn::ExprUnsafe) -> syn::Expr {
        syn::Expr::Unsafe(t)
    }
    fn chain_expr_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::Expr {
        syn::Expr::Verbatim(t)
    }
    fn chain_expr_while(&mut self, c: &mut Self::Input, t: syn::ExprWhile) -> syn::Expr {
        syn::Expr::While(t)
    }
    fn chain_expr_yield(&mut self, c: &mut Self::Input, t: syn::ExprYield) -> syn::Expr {
        syn::Expr::Yield(t)
    }
    fn chain_exprarray(&mut self, c: &mut Self::Input, t: syn::ExprArray) -> syn::ExprArray {
        t
    }
    fn chain_exprassign(&mut self, c: &mut Self::Input, t: syn::ExprAssign) -> syn::ExprAssign {
        t
    }
    fn chain_exprassignop(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssignOp,
    ) -> syn::ExprAssignOp {
        t
    }
    fn chain_exprasync(&mut self, c: &mut Self::Input, t: syn::ExprAsync) -> syn::ExprAsync {
        t
    }
    fn chain_exprawait(&mut self, c: &mut Self::Input, t: syn::ExprAwait) -> syn::ExprAwait {
        t
    }
    fn chain_exprbinary(&mut self, c: &mut Self::Input, t: syn::ExprBinary) -> syn::ExprBinary {
        t
    }
    fn chain_exprblock(&mut self, c: &mut Self::Input, t: syn::ExprBlock) -> syn::ExprBlock {
        t
    }
    fn chain_exprbox(&mut self, c: &mut Self::Input, t: syn::ExprBox) -> syn::ExprBox {
        t
    }
    fn chain_exprbreak(&mut self, c: &mut Self::Input, t: syn::ExprBreak) -> syn::ExprBreak {
        t
    }
    fn chain_exprcall(&mut self, c: &mut Self::Input, t: syn::ExprCall) -> syn::ExprCall {
        t
    }
    fn chain_exprcast(&mut self, c: &mut Self::Input, t: syn::ExprCast) -> syn::ExprCast {
        t
    }
    fn chain_exprclosure(&mut self, c: &mut Self::Input, t: syn::ExprClosure) -> syn::ExprClosure {
        t
    }
    fn chain_exprcontinue(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprContinue,
    ) -> syn::ExprContinue {
        t
    }
    fn chain_exprfield(&mut self, c: &mut Self::Input, t: syn::ExprField) -> syn::ExprField {
        t
    }
    fn chain_exprforloop(&mut self, c: &mut Self::Input, t: syn::ExprForLoop) -> syn::ExprForLoop {
        t
    }
    fn chain_exprgroup(&mut self, c: &mut Self::Input, t: syn::ExprGroup) -> syn::ExprGroup {
        t
    }
    fn chain_exprif(&mut self, c: &mut Self::Input, t: syn::ExprIf) -> syn::ExprIf {
        t
    }
    fn chain_exprindex(&mut self, c: &mut Self::Input, t: syn::ExprIndex) -> syn::ExprIndex {
        t
    }
    fn chain_exprlet(&mut self, c: &mut Self::Input, t: syn::ExprLet) -> syn::ExprLet {
        t
    }
    fn chain_exprlit(&mut self, c: &mut Self::Input, t: syn::ExprLit) -> syn::ExprLit {
        t
    }
    fn chain_exprloop(&mut self, c: &mut Self::Input, t: syn::ExprLoop) -> syn::ExprLoop {
        t
    }
    fn chain_exprmacro(&mut self, c: &mut Self::Input, t: syn::ExprMacro) -> syn::ExprMacro {
        t
    }
    fn chain_exprmatch(&mut self, c: &mut Self::Input, t: syn::ExprMatch) -> syn::ExprMatch {
        t
    }
    fn chain_exprmethodcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMethodCall,
    ) -> syn::ExprMethodCall {
        t
    }
    fn chain_exprparen(&mut self, c: &mut Self::Input, t: syn::ExprParen) -> syn::ExprParen {
        t
    }
    fn chain_exprpath(&mut self, c: &mut Self::Input, t: syn::ExprPath) -> syn::ExprPath {
        t
    }
    fn chain_exprrange(&mut self, c: &mut Self::Input, t: syn::ExprRange) -> syn::ExprRange {
        t
    }
    fn chain_exprreference(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprReference,
    ) -> syn::ExprReference {
        t
    }
    fn chain_exprrepeat(&mut self, c: &mut Self::Input, t: syn::ExprRepeat) -> syn::ExprRepeat {
        t
    }
    fn chain_exprreturn(&mut self, c: &mut Self::Input, t: syn::ExprReturn) -> syn::ExprReturn {
        t
    }
    fn chain_exprstruct(&mut self, c: &mut Self::Input, t: syn::ExprStruct) -> syn::ExprStruct {
        t
    }
    fn chain_exprtry(&mut self, c: &mut Self::Input, t: syn::ExprTry) -> syn::ExprTry {
        t
    }
    fn chain_exprtryblock(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprTryBlock,
    ) -> syn::ExprTryBlock {
        t
    }
    fn chain_exprtuple(&mut self, c: &mut Self::Input, t: syn::ExprTuple) -> syn::ExprTuple {
        t
    }
    fn chain_exprtype(&mut self, c: &mut Self::Input, t: syn::ExprType) -> syn::ExprType {
        t
    }
    fn chain_exprunary(&mut self, c: &mut Self::Input, t: syn::ExprUnary) -> syn::ExprUnary {
        t
    }
    fn chain_exprunsafe(&mut self, c: &mut Self::Input, t: syn::ExprUnsafe) -> syn::ExprUnsafe {
        t
    }
    fn chain_exprwhile(&mut self, c: &mut Self::Input, t: syn::ExprWhile) -> syn::ExprWhile {
        t
    }
    fn chain_expryield(&mut self, c: &mut Self::Input, t: syn::ExprYield) -> syn::ExprYield {
        t
    }
    fn chain_field(&mut self, c: &mut Self::Input, t: syn::Field) -> syn::Field {
        t
    }
    fn chain_fieldpat(&mut self, c: &mut Self::Input, t: syn::FieldPat) -> syn::FieldPat {
        t
    }
    fn chain_fieldvalue(&mut self, c: &mut Self::Input, t: syn::FieldValue) -> syn::FieldValue {
        t
    }
    fn chain_fields_named(&mut self, c: &mut Self::Input, t: syn::FieldsNamed) -> syn::Fields {
        syn::Fields::Named(t)
    }
    fn chain_fields_unnamed(&mut self, c: &mut Self::Input, t: syn::FieldsUnnamed) -> syn::Fields {
        syn::Fields::Unnamed(t)
    }
    fn chain_fieldsnamed(&mut self, c: &mut Self::Input, t: syn::FieldsNamed) -> syn::FieldsNamed {
        t
    }
    fn chain_fieldsunnamed(
        &mut self,
        c: &mut Self::Input,
        t: syn::FieldsUnnamed,
    ) -> syn::FieldsUnnamed {
        t
    }
    fn chain_file(&mut self, c: &mut Self::Input, t: syn::File) -> syn::File {
        t
    }
    fn chain_fnarg_receiver(&mut self, c: &mut Self::Input, t: syn::Receiver) -> syn::FnArg {
        syn::FnArg::Receiver(t)
    }
    fn chain_fnarg_typed(&mut self, c: &mut Self::Input, t: syn::PatType) -> syn::FnArg {
        syn::FnArg::Typed(t)
    }
    fn chain_foreignitem_fn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemFn,
    ) -> syn::ForeignItem {
        syn::ForeignItem::Fn(t)
    }
    fn chain_foreignitem_static(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemStatic,
    ) -> syn::ForeignItem {
        syn::ForeignItem::Static(t)
    }
    fn chain_foreignitem_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemType,
    ) -> syn::ForeignItem {
        syn::ForeignItem::Type(t)
    }
    fn chain_foreignitem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemMacro,
    ) -> syn::ForeignItem {
        syn::ForeignItem::Macro(t)
    }
    fn chain_foreignitem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::ForeignItem {
        syn::ForeignItem::Verbatim(t)
    }
    fn chain_foreignitemfn(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemFn,
    ) -> syn::ForeignItemFn {
        t
    }
    fn chain_foreignitemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemMacro,
    ) -> syn::ForeignItemMacro {
        t
    }
    fn chain_foreignitemstatic(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemStatic,
    ) -> syn::ForeignItemStatic {
        t
    }
    fn chain_foreignitemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ForeignItemType,
    ) -> syn::ForeignItemType {
        t
    }
    fn chain_genericargument_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lifetime,
    ) -> syn::GenericArgument {
        syn::GenericArgument::Lifetime(t)
    }
    fn chain_genericargument_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::Type,
    ) -> syn::GenericArgument {
        syn::GenericArgument::Type(t)
    }
    fn chain_genericargument_binding(
        &mut self,
        c: &mut Self::Input,
        t: syn::Binding,
    ) -> syn::GenericArgument {
        syn::GenericArgument::Binding(t)
    }
    fn chain_genericargument_constraint(
        &mut self,
        c: &mut Self::Input,
        t: syn::Constraint,
    ) -> syn::GenericArgument {
        syn::GenericArgument::Constraint(t)
    }
    fn chain_genericargument_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> syn::GenericArgument {
        syn::GenericArgument::Const(t)
    }
    fn chain_genericmethodargument_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::Type,
    ) -> syn::GenericMethodArgument {
        syn::GenericMethodArgument::Type(t)
    }
    fn chain_genericmethodargument_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> syn::GenericMethodArgument {
        syn::GenericMethodArgument::Const(t)
    }
    fn chain_genericparam_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeParam,
    ) -> syn::GenericParam {
        syn::GenericParam::Type(t)
    }
    fn chain_genericparam_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::LifetimeDef,
    ) -> syn::GenericParam {
        syn::GenericParam::Lifetime(t)
    }
    fn chain_genericparam_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::ConstParam,
    ) -> syn::GenericParam {
        syn::GenericParam::Const(t)
    }
    fn chain_generics(&mut self, c: &mut Self::Input, t: syn::Generics) -> syn::Generics {
        t
    }
    fn chain_implitem_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemConst,
    ) -> syn::ImplItem {
        syn::ImplItem::Const(t)
    }
    fn chain_implitem_method(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMethod,
    ) -> syn::ImplItem {
        syn::ImplItem::Method(t)
    }
    fn chain_implitem_type(&mut self, c: &mut Self::Input, t: syn::ImplItemType) -> syn::ImplItem {
        syn::ImplItem::Type(t)
    }
    fn chain_implitem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMacro,
    ) -> syn::ImplItem {
        syn::ImplItem::Macro(t)
    }
    fn chain_implitem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::ImplItem {
        syn::ImplItem::Verbatim(t)
    }
    fn chain_implitemconst(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemConst,
    ) -> syn::ImplItemConst {
        t
    }
    fn chain_implitemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMacro,
    ) -> syn::ImplItemMacro {
        t
    }
    fn chain_implitemmethod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemMethod,
    ) -> syn::ImplItemMethod {
        t
    }
    fn chain_implitemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ImplItemType,
    ) -> syn::ImplItemType {
        t
    }
    fn chain_index(&mut self, c: &mut Self::Input, t: syn::Index) -> syn::Index {
        t
    }
    fn chain_item_const(&mut self, c: &mut Self::Input, t: syn::ItemConst) -> syn::Item {
        syn::Item::Const(t)
    }
    fn chain_item_enum(&mut self, c: &mut Self::Input, t: syn::ItemEnum) -> syn::Item {
        syn::Item::Enum(t)
    }
    fn chain_item_externcrate(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemExternCrate,
    ) -> syn::Item {
        syn::Item::ExternCrate(t)
    }
    fn chain_item_fn(&mut self, c: &mut Self::Input, t: syn::ItemFn) -> syn::Item {
        syn::Item::Fn(t)
    }
    fn chain_item_foreignmod(&mut self, c: &mut Self::Input, t: syn::ItemForeignMod) -> syn::Item {
        syn::Item::ForeignMod(t)
    }
    fn chain_item_impl(&mut self, c: &mut Self::Input, t: syn::ItemImpl) -> syn::Item {
        syn::Item::Impl(t)
    }
    fn chain_item_macro(&mut self, c: &mut Self::Input, t: syn::ItemMacro) -> syn::Item {
        syn::Item::Macro(t)
    }
    fn chain_item_macro2(&mut self, c: &mut Self::Input, t: syn::ItemMacro2) -> syn::Item {
        syn::Item::Macro2(t)
    }
    fn chain_item_mod(&mut self, c: &mut Self::Input, t: syn::ItemMod) -> syn::Item {
        syn::Item::Mod(t)
    }
    fn chain_item_static(&mut self, c: &mut Self::Input, t: syn::ItemStatic) -> syn::Item {
        syn::Item::Static(t)
    }
    fn chain_item_struct(&mut self, c: &mut Self::Input, t: syn::ItemStruct) -> syn::Item {
        syn::Item::Struct(t)
    }
    fn chain_item_trait(&mut self, c: &mut Self::Input, t: syn::ItemTrait) -> syn::Item {
        syn::Item::Trait(t)
    }
    fn chain_item_traitalias(&mut self, c: &mut Self::Input, t: syn::ItemTraitAlias) -> syn::Item {
        syn::Item::TraitAlias(t)
    }
    fn chain_item_type(&mut self, c: &mut Self::Input, t: syn::ItemType) -> syn::Item {
        syn::Item::Type(t)
    }
    fn chain_item_union(&mut self, c: &mut Self::Input, t: syn::ItemUnion) -> syn::Item {
        syn::Item::Union(t)
    }
    fn chain_item_use(&mut self, c: &mut Self::Input, t: syn::ItemUse) -> syn::Item {
        syn::Item::Use(t)
    }
    fn chain_item_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::Item {
        syn::Item::Verbatim(t)
    }
    fn chain_itemconst(&mut self, c: &mut Self::Input, t: syn::ItemConst) -> syn::ItemConst {
        t
    }
    fn chain_itemenum(&mut self, c: &mut Self::Input, t: syn::ItemEnum) -> syn::ItemEnum {
        t
    }
    fn chain_itemexterncrate(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemExternCrate,
    ) -> syn::ItemExternCrate {
        t
    }
    fn chain_itemfn(&mut self, c: &mut Self::Input, t: syn::ItemFn) -> syn::ItemFn {
        t
    }
    fn chain_itemforeignmod(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemForeignMod,
    ) -> syn::ItemForeignMod {
        t
    }
    fn chain_itemimpl(&mut self, c: &mut Self::Input, t: syn::ItemImpl) -> syn::ItemImpl {
        t
    }
    fn chain_itemmacro(&mut self, c: &mut Self::Input, t: syn::ItemMacro) -> syn::ItemMacro {
        t
    }
    fn chain_itemmacro2(&mut self, c: &mut Self::Input, t: syn::ItemMacro2) -> syn::ItemMacro2 {
        t
    }
    fn chain_itemmod(&mut self, c: &mut Self::Input, t: syn::ItemMod) -> syn::ItemMod {
        t
    }
    fn chain_itemstatic(&mut self, c: &mut Self::Input, t: syn::ItemStatic) -> syn::ItemStatic {
        t
    }
    fn chain_itemstruct(&mut self, c: &mut Self::Input, t: syn::ItemStruct) -> syn::ItemStruct {
        t
    }
    fn chain_itemtrait(&mut self, c: &mut Self::Input, t: syn::ItemTrait) -> syn::ItemTrait {
        t
    }
    fn chain_itemtraitalias(
        &mut self,
        c: &mut Self::Input,
        t: syn::ItemTraitAlias,
    ) -> syn::ItemTraitAlias {
        t
    }
    fn chain_itemtype(&mut self, c: &mut Self::Input, t: syn::ItemType) -> syn::ItemType {
        t
    }
    fn chain_itemunion(&mut self, c: &mut Self::Input, t: syn::ItemUnion) -> syn::ItemUnion {
        t
    }
    fn chain_itemuse(&mut self, c: &mut Self::Input, t: syn::ItemUse) -> syn::ItemUse {
        t
    }
    fn chain_label(&mut self, c: &mut Self::Input, t: syn::Label) -> syn::Label {
        t
    }
    fn chain_lifetime(&mut self, c: &mut Self::Input, t: syn::Lifetime) -> syn::Lifetime {
        t
    }
    fn chain_lifetimedef(&mut self, c: &mut Self::Input, t: syn::LifetimeDef) -> syn::LifetimeDef {
        t
    }
    fn chain_lit_str(&mut self, c: &mut Self::Input, t: syn::LitStr) -> syn::Lit {
        syn::Lit::Str(t)
    }
    fn chain_lit_bytestr(&mut self, c: &mut Self::Input, t: syn::LitByteStr) -> syn::Lit {
        syn::Lit::ByteStr(t)
    }
    fn chain_lit_byte(&mut self, c: &mut Self::Input, t: syn::LitByte) -> syn::Lit {
        syn::Lit::Byte(t)
    }
    fn chain_lit_char(&mut self, c: &mut Self::Input, t: syn::LitChar) -> syn::Lit {
        syn::Lit::Char(t)
    }
    fn chain_lit_int(&mut self, c: &mut Self::Input, t: syn::LitInt) -> syn::Lit {
        syn::Lit::Int(t)
    }
    fn chain_lit_float(&mut self, c: &mut Self::Input, t: syn::LitFloat) -> syn::Lit {
        syn::Lit::Float(t)
    }
    fn chain_lit_bool(&mut self, c: &mut Self::Input, t: syn::LitBool) -> syn::Lit {
        syn::Lit::Bool(t)
    }
    fn chain_lit_verbatim(&mut self, c: &mut Self::Input, t: proc_macro2::Literal) -> syn::Lit {
        syn::Lit::Verbatim(t)
    }
    fn chain_litbool(&mut self, c: &mut Self::Input, t: syn::LitBool) -> syn::LitBool {
        t
    }
    fn chain_litbyte(&mut self, c: &mut Self::Input, t: syn::LitByte) -> syn::LitByte {
        t
    }
    fn chain_litbytestr(&mut self, c: &mut Self::Input, t: syn::LitByteStr) -> syn::LitByteStr {
        t
    }
    fn chain_litchar(&mut self, c: &mut Self::Input, t: syn::LitChar) -> syn::LitChar {
        t
    }
    fn chain_litfloat(&mut self, c: &mut Self::Input, t: syn::LitFloat) -> syn::LitFloat {
        t
    }
    fn chain_litint(&mut self, c: &mut Self::Input, t: syn::LitInt) -> syn::LitInt {
        t
    }
    fn chain_litstr(&mut self, c: &mut Self::Input, t: syn::LitStr) -> syn::LitStr {
        t
    }
    fn chain_local(&mut self, c: &mut Self::Input, t: syn::Local) -> syn::Local {
        t
    }
    fn chain_macro(&mut self, c: &mut Self::Input, t: syn::Macro) -> syn::Macro {
        t
    }
    fn chain_macrodelimiter_paren(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Paren,
    ) -> syn::MacroDelimiter {
        syn::MacroDelimiter::Paren(t)
    }
    fn chain_macrodelimiter_brace(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Brace,
    ) -> syn::MacroDelimiter {
        syn::MacroDelimiter::Brace(t)
    }
    fn chain_macrodelimiter_bracket(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Bracket,
    ) -> syn::MacroDelimiter {
        syn::MacroDelimiter::Bracket(t)
    }
    fn chain_member_named(&mut self, c: &mut Self::Input, t: proc_macro2::Ident) -> syn::Member {
        syn::Member::Named(t)
    }
    fn chain_member_unnamed(&mut self, c: &mut Self::Input, t: syn::Index) -> syn::Member {
        syn::Member::Unnamed(t)
    }
    fn chain_meta_path(&mut self, c: &mut Self::Input, t: syn::Path) -> syn::Meta {
        syn::Meta::Path(t)
    }
    fn chain_meta_list(&mut self, c: &mut Self::Input, t: syn::MetaList) -> syn::Meta {
        syn::Meta::List(t)
    }
    fn chain_meta_namevalue(&mut self, c: &mut Self::Input, t: syn::MetaNameValue) -> syn::Meta {
        syn::Meta::NameValue(t)
    }
    fn chain_metalist(&mut self, c: &mut Self::Input, t: syn::MetaList) -> syn::MetaList {
        t
    }
    fn chain_metanamevalue(
        &mut self,
        c: &mut Self::Input,
        t: syn::MetaNameValue,
    ) -> syn::MetaNameValue {
        t
    }
    fn chain_methodturbofish(
        &mut self,
        c: &mut Self::Input,
        t: syn::MethodTurbofish,
    ) -> syn::MethodTurbofish {
        t
    }
    fn chain_nestedmeta_meta(&mut self, c: &mut Self::Input, t: syn::Meta) -> syn::NestedMeta {
        syn::NestedMeta::Meta(t)
    }
    fn chain_nestedmeta_lit(&mut self, c: &mut Self::Input, t: syn::Lit) -> syn::NestedMeta {
        syn::NestedMeta::Lit(t)
    }
    fn chain_parenthesizedgenericarguments(
        &mut self,
        c: &mut Self::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> syn::ParenthesizedGenericArguments {
        t
    }
    fn chain_pat_box(&mut self, c: &mut Self::Input, t: syn::PatBox) -> syn::Pat {
        syn::Pat::Box(t)
    }
    fn chain_pat_ident(&mut self, c: &mut Self::Input, t: syn::PatIdent) -> syn::Pat {
        syn::Pat::Ident(t)
    }
    fn chain_pat_lit(&mut self, c: &mut Self::Input, t: syn::PatLit) -> syn::Pat {
        syn::Pat::Lit(t)
    }
    fn chain_pat_macro(&mut self, c: &mut Self::Input, t: syn::PatMacro) -> syn::Pat {
        syn::Pat::Macro(t)
    }
    fn chain_pat_or(&mut self, c: &mut Self::Input, t: syn::PatOr) -> syn::Pat {
        syn::Pat::Or(t)
    }
    fn chain_pat_path(&mut self, c: &mut Self::Input, t: syn::PatPath) -> syn::Pat {
        syn::Pat::Path(t)
    }
    fn chain_pat_range(&mut self, c: &mut Self::Input, t: syn::PatRange) -> syn::Pat {
        syn::Pat::Range(t)
    }
    fn chain_pat_reference(&mut self, c: &mut Self::Input, t: syn::PatReference) -> syn::Pat {
        syn::Pat::Reference(t)
    }
    fn chain_pat_rest(&mut self, c: &mut Self::Input, t: syn::PatRest) -> syn::Pat {
        syn::Pat::Rest(t)
    }
    fn chain_pat_slice(&mut self, c: &mut Self::Input, t: syn::PatSlice) -> syn::Pat {
        syn::Pat::Slice(t)
    }
    fn chain_pat_struct(&mut self, c: &mut Self::Input, t: syn::PatStruct) -> syn::Pat {
        syn::Pat::Struct(t)
    }
    fn chain_pat_tuple(&mut self, c: &mut Self::Input, t: syn::PatTuple) -> syn::Pat {
        syn::Pat::Tuple(t)
    }
    fn chain_pat_tuplestruct(&mut self, c: &mut Self::Input, t: syn::PatTupleStruct) -> syn::Pat {
        syn::Pat::TupleStruct(t)
    }
    fn chain_pat_type(&mut self, c: &mut Self::Input, t: syn::PatType) -> syn::Pat {
        syn::Pat::Type(t)
    }
    fn chain_pat_verbatim(&mut self, c: &mut Self::Input, t: proc_macro2::TokenStream) -> syn::Pat {
        syn::Pat::Verbatim(t)
    }
    fn chain_pat_wild(&mut self, c: &mut Self::Input, t: syn::PatWild) -> syn::Pat {
        syn::Pat::Wild(t)
    }
    fn chain_patbox(&mut self, c: &mut Self::Input, t: syn::PatBox) -> syn::PatBox {
        t
    }
    fn chain_patident(&mut self, c: &mut Self::Input, t: syn::PatIdent) -> syn::PatIdent {
        t
    }
    fn chain_patlit(&mut self, c: &mut Self::Input, t: syn::PatLit) -> syn::PatLit {
        t
    }
    fn chain_patmacro(&mut self, c: &mut Self::Input, t: syn::PatMacro) -> syn::PatMacro {
        t
    }
    fn chain_pator(&mut self, c: &mut Self::Input, t: syn::PatOr) -> syn::PatOr {
        t
    }
    fn chain_patpath(&mut self, c: &mut Self::Input, t: syn::PatPath) -> syn::PatPath {
        t
    }
    fn chain_patrange(&mut self, c: &mut Self::Input, t: syn::PatRange) -> syn::PatRange {
        t
    }
    fn chain_patreference(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatReference,
    ) -> syn::PatReference {
        t
    }
    fn chain_patrest(&mut self, c: &mut Self::Input, t: syn::PatRest) -> syn::PatRest {
        t
    }
    fn chain_patslice(&mut self, c: &mut Self::Input, t: syn::PatSlice) -> syn::PatSlice {
        t
    }
    fn chain_patstruct(&mut self, c: &mut Self::Input, t: syn::PatStruct) -> syn::PatStruct {
        t
    }
    fn chain_pattuple(&mut self, c: &mut Self::Input, t: syn::PatTuple) -> syn::PatTuple {
        t
    }
    fn chain_pattuplestruct(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatTupleStruct,
    ) -> syn::PatTupleStruct {
        t
    }
    fn chain_pattype(&mut self, c: &mut Self::Input, t: syn::PatType) -> syn::PatType {
        t
    }
    fn chain_patwild(&mut self, c: &mut Self::Input, t: syn::PatWild) -> syn::PatWild {
        t
    }
    fn chain_path(&mut self, c: &mut Self::Input, t: syn::Path) -> syn::Path {
        t
    }
    fn chain_patharguments_anglebracketed(
        &mut self,
        c: &mut Self::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> syn::PathArguments {
        syn::PathArguments::AngleBracketed(t)
    }
    fn chain_patharguments_parenthesized(
        &mut self,
        c: &mut Self::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> syn::PathArguments {
        syn::PathArguments::Parenthesized(t)
    }
    fn chain_pathsegment(&mut self, c: &mut Self::Input, t: syn::PathSegment) -> syn::PathSegment {
        t
    }
    fn chain_predicateeq(&mut self, c: &mut Self::Input, t: syn::PredicateEq) -> syn::PredicateEq {
        t
    }
    fn chain_predicatelifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateLifetime,
    ) -> syn::PredicateLifetime {
        t
    }
    fn chain_predicatetype(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateType,
    ) -> syn::PredicateType {
        t
    }
    fn chain_qself(&mut self, c: &mut Self::Input, t: syn::QSelf) -> syn::QSelf {
        t
    }
    fn chain_rangelimits_halfopen(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Dot2,
    ) -> syn::RangeLimits {
        syn::RangeLimits::HalfOpen(t)
    }
    fn chain_rangelimits_closed(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::DotDotEq,
    ) -> syn::RangeLimits {
        syn::RangeLimits::Closed(t)
    }
    fn chain_receiver(&mut self, c: &mut Self::Input, t: syn::Receiver) -> syn::Receiver {
        t
    }
    fn chain_returntype_type(
        &mut self,
        c: &mut Self::Input,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> syn::ReturnType {
        syn::ReturnType::Type(t.0, t.1)
    }
    fn chain_signature(&mut self, c: &mut Self::Input, t: syn::Signature) -> syn::Signature {
        t
    }
    fn chain_stmt_local(&mut self, c: &mut Self::Input, t: syn::Local) -> syn::Stmt {
        syn::Stmt::Local(t)
    }
    fn chain_stmt_item(&mut self, c: &mut Self::Input, t: syn::Item) -> syn::Stmt {
        syn::Stmt::Item(t)
    }
    fn chain_stmt_expr(&mut self, c: &mut Self::Input, t: syn::Expr) -> syn::Stmt {
        syn::Stmt::Expr(t)
    }
    fn chain_stmt_semi(
        &mut self,
        c: &mut Self::Input,
        t: (syn::Expr, syn::token::Semi),
    ) -> syn::Stmt {
        syn::Stmt::Semi(t.0, t.1)
    }
    fn chain_traitbound(&mut self, c: &mut Self::Input, t: syn::TraitBound) -> syn::TraitBound {
        t
    }
    fn chain_traitboundmodifier_maybe(
        &mut self,
        c: &mut Self::Input,
        t: syn::token::Question,
    ) -> syn::TraitBoundModifier {
        syn::TraitBoundModifier::Maybe(t)
    }
    fn chain_traititem_const(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemConst,
    ) -> syn::TraitItem {
        syn::TraitItem::Const(t)
    }
    fn chain_traititem_method(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMethod,
    ) -> syn::TraitItem {
        syn::TraitItem::Method(t)
    }
    fn chain_traititem_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemType,
    ) -> syn::TraitItem {
        syn::TraitItem::Type(t)
    }
    fn chain_traititem_macro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMacro,
    ) -> syn::TraitItem {
        syn::TraitItem::Macro(t)
    }
    fn chain_traititem_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::TraitItem {
        syn::TraitItem::Verbatim(t)
    }
    fn chain_traititemconst(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemConst,
    ) -> syn::TraitItemConst {
        t
    }
    fn chain_traititemmacro(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMacro,
    ) -> syn::TraitItemMacro {
        t
    }
    fn chain_traititemmethod(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemMethod,
    ) -> syn::TraitItemMethod {
        t
    }
    fn chain_traititemtype(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitItemType,
    ) -> syn::TraitItemType {
        t
    }
    fn chain_type_array(&mut self, c: &mut Self::Input, t: syn::TypeArray) -> syn::Type {
        syn::Type::Array(t)
    }
    fn chain_type_barefn(&mut self, c: &mut Self::Input, t: syn::TypeBareFn) -> syn::Type {
        syn::Type::BareFn(t)
    }
    fn chain_type_group(&mut self, c: &mut Self::Input, t: syn::TypeGroup) -> syn::Type {
        syn::Type::Group(t)
    }
    fn chain_type_impltrait(&mut self, c: &mut Self::Input, t: syn::TypeImplTrait) -> syn::Type {
        syn::Type::ImplTrait(t)
    }
    fn chain_type_infer(&mut self, c: &mut Self::Input, t: syn::TypeInfer) -> syn::Type {
        syn::Type::Infer(t)
    }
    fn chain_type_macro(&mut self, c: &mut Self::Input, t: syn::TypeMacro) -> syn::Type {
        syn::Type::Macro(t)
    }
    fn chain_type_never(&mut self, c: &mut Self::Input, t: syn::TypeNever) -> syn::Type {
        syn::Type::Never(t)
    }
    fn chain_type_paren(&mut self, c: &mut Self::Input, t: syn::TypeParen) -> syn::Type {
        syn::Type::Paren(t)
    }
    fn chain_type_path(&mut self, c: &mut Self::Input, t: syn::TypePath) -> syn::Type {
        syn::Type::Path(t)
    }
    fn chain_type_ptr(&mut self, c: &mut Self::Input, t: syn::TypePtr) -> syn::Type {
        syn::Type::Ptr(t)
    }
    fn chain_type_reference(&mut self, c: &mut Self::Input, t: syn::TypeReference) -> syn::Type {
        syn::Type::Reference(t)
    }
    fn chain_type_slice(&mut self, c: &mut Self::Input, t: syn::TypeSlice) -> syn::Type {
        syn::Type::Slice(t)
    }
    fn chain_type_traitobject(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTraitObject,
    ) -> syn::Type {
        syn::Type::TraitObject(t)
    }
    fn chain_type_tuple(&mut self, c: &mut Self::Input, t: syn::TypeTuple) -> syn::Type {
        syn::Type::Tuple(t)
    }
    fn chain_type_verbatim(
        &mut self,
        c: &mut Self::Input,
        t: proc_macro2::TokenStream,
    ) -> syn::Type {
        syn::Type::Verbatim(t)
    }
    fn chain_typearray(&mut self, c: &mut Self::Input, t: syn::TypeArray) -> syn::TypeArray {
        t
    }
    fn chain_typebarefn(&mut self, c: &mut Self::Input, t: syn::TypeBareFn) -> syn::TypeBareFn {
        t
    }
    fn chain_typegroup(&mut self, c: &mut Self::Input, t: syn::TypeGroup) -> syn::TypeGroup {
        t
    }
    fn chain_typeimpltrait(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeImplTrait,
    ) -> syn::TypeImplTrait {
        t
    }
    fn chain_typeinfer(&mut self, c: &mut Self::Input, t: syn::TypeInfer) -> syn::TypeInfer {
        t
    }
    fn chain_typemacro(&mut self, c: &mut Self::Input, t: syn::TypeMacro) -> syn::TypeMacro {
        t
    }
    fn chain_typenever(&mut self, c: &mut Self::Input, t: syn::TypeNever) -> syn::TypeNever {
        t
    }
    fn chain_typeparam(&mut self, c: &mut Self::Input, t: syn::TypeParam) -> syn::TypeParam {
        t
    }
    fn chain_typeparambound_trait(
        &mut self,
        c: &mut Self::Input,
        t: syn::TraitBound,
    ) -> syn::TypeParamBound {
        syn::TypeParamBound::Trait(t)
    }
    fn chain_typeparambound_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::Lifetime,
    ) -> syn::TypeParamBound {
        syn::TypeParamBound::Lifetime(t)
    }
    fn chain_typeparen(&mut self, c: &mut Self::Input, t: syn::TypeParen) -> syn::TypeParen {
        t
    }
    fn chain_typepath(&mut self, c: &mut Self::Input, t: syn::TypePath) -> syn::TypePath {
        t
    }
    fn chain_typeptr(&mut self, c: &mut Self::Input, t: syn::TypePtr) -> syn::TypePtr {
        t
    }
    fn chain_typereference(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeReference,
    ) -> syn::TypeReference {
        t
    }
    fn chain_typeslice(&mut self, c: &mut Self::Input, t: syn::TypeSlice) -> syn::TypeSlice {
        t
    }
    fn chain_typetraitobject(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypeTraitObject,
    ) -> syn::TypeTraitObject {
        t
    }
    fn chain_typetuple(&mut self, c: &mut Self::Input, t: syn::TypeTuple) -> syn::TypeTuple {
        t
    }
    fn chain_unop_deref(&mut self, c: &mut Self::Input, t: syn::token::Star) -> syn::UnOp {
        syn::UnOp::Deref(t)
    }
    fn chain_unop_not(&mut self, c: &mut Self::Input, t: syn::token::Bang) -> syn::UnOp {
        syn::UnOp::Not(t)
    }
    fn chain_unop_neg(&mut self, c: &mut Self::Input, t: syn::token::Sub) -> syn::UnOp {
        syn::UnOp::Neg(t)
    }
    fn chain_useglob(&mut self, c: &mut Self::Input, t: syn::UseGlob) -> syn::UseGlob {
        t
    }
    fn chain_usegroup(&mut self, c: &mut Self::Input, t: syn::UseGroup) -> syn::UseGroup {
        t
    }
    fn chain_usename(&mut self, c: &mut Self::Input, t: syn::UseName) -> syn::UseName {
        t
    }
    fn chain_usepath(&mut self, c: &mut Self::Input, t: syn::UsePath) -> syn::UsePath {
        t
    }
    fn chain_userename(&mut self, c: &mut Self::Input, t: syn::UseRename) -> syn::UseRename {
        t
    }
    fn chain_usetree_path(&mut self, c: &mut Self::Input, t: syn::UsePath) -> syn::UseTree {
        syn::UseTree::Path(t)
    }
    fn chain_usetree_name(&mut self, c: &mut Self::Input, t: syn::UseName) -> syn::UseTree {
        syn::UseTree::Name(t)
    }
    fn chain_usetree_rename(&mut self, c: &mut Self::Input, t: syn::UseRename) -> syn::UseTree {
        syn::UseTree::Rename(t)
    }
    fn chain_usetree_glob(&mut self, c: &mut Self::Input, t: syn::UseGlob) -> syn::UseTree {
        syn::UseTree::Glob(t)
    }
    fn chain_usetree_group(&mut self, c: &mut Self::Input, t: syn::UseGroup) -> syn::UseTree {
        syn::UseTree::Group(t)
    }
    fn chain_variadic(&mut self, c: &mut Self::Input, t: syn::Variadic) -> syn::Variadic {
        t
    }
    fn chain_variant(&mut self, c: &mut Self::Input, t: syn::Variant) -> syn::Variant {
        t
    }
    fn chain_viscrate(&mut self, c: &mut Self::Input, t: syn::VisCrate) -> syn::VisCrate {
        t
    }
    fn chain_vispublic(&mut self, c: &mut Self::Input, t: syn::VisPublic) -> syn::VisPublic {
        t
    }
    fn chain_visrestricted(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisRestricted,
    ) -> syn::VisRestricted {
        t
    }
    fn chain_visibility_public(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisPublic,
    ) -> syn::Visibility {
        syn::Visibility::Public(t)
    }
    fn chain_visibility_crate(&mut self, c: &mut Self::Input, t: syn::VisCrate) -> syn::Visibility {
        syn::Visibility::Crate(t)
    }
    fn chain_visibility_restricted(
        &mut self,
        c: &mut Self::Input,
        t: syn::VisRestricted,
    ) -> syn::Visibility {
        syn::Visibility::Restricted(t)
    }
    fn chain_whereclause(&mut self, c: &mut Self::Input, t: syn::WhereClause) -> syn::WhereClause {
        t
    }
    fn chain_wherepredicate_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateType,
    ) -> syn::WherePredicate {
        syn::WherePredicate::Type(t)
    }
    fn chain_wherepredicate_lifetime(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateLifetime,
    ) -> syn::WherePredicate {
        syn::WherePredicate::Lifetime(t)
    }
    fn chain_wherepredicate_eq(
        &mut self,
        c: &mut Self::Input,
        t: syn::PredicateEq,
    ) -> syn::WherePredicate {
        syn::WherePredicate::Eq(t)
    }
}
