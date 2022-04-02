// codegen file by version 0.1.0
// don't edit this

use super::ChainIter;
/// fold chain of responsibility trait
/// it is a before chain -> fold -> after chain trait
#[allow(unused)]
pub trait FoldChain
where
    Self: ChainIter,
{
    fn fold_chain_abi(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Abi,
    ) -> Result<syn::Abi, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_abi(c, t)?;
        }
        t.extern_token = t.extern_token;
        t.name = match t.name {
            Some(o) => Some(self.fold_chain_litstr(c, o)?),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_abi(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_anglebracketedgenericarguments(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::AngleBracketedGenericArguments, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_anglebracketedgenericarguments(c, t)?;
        }
        t.colon2_token = t.colon2_token;
        t.lt_token = t.lt_token;
        t.args = t.args;
        t.gt_token = t.gt_token;
        for mut i in self.after() {
            t = i.chain_anglebracketedgenericarguments(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_arm(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Arm,
    ) -> Result<syn::Arm, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_arm(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.pat = self.fold_chain_pat(c, t.pat)?;
        t.guard = match t.guard {
            Some(o) => Some((o.0, Box::new(self.fold_chain_expr(c, *o.1)?))),
            None => None,
        };
        t.fat_arrow_token = t.fat_arrow_token;
        t.body = Box::new(self.fold_chain_expr(c, *t.body)?);
        t.comma = t.comma;
        for mut i in self.after() {
            t = i.chain_arm(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_attrstyle(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::AttrStyle,
    ) -> Result<syn::AttrStyle, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_attrstyle(c, t)?;
        }
        t = match t {
            syn::AttrStyle::Inner(tmp) => self.fold_chain_attrstyle_inner(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_attrstyle(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_attrstyle_inner(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Bang,
    ) -> Result<syn::AttrStyle, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_attrstyle_inner(c, t)? {
                syn::AttrStyle::Inner(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_attrstyle_inner(c, t)? {
                syn::AttrStyle::Inner(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::AttrStyle::Inner(t))
    }
    fn fold_chain_attribute(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Attribute,
    ) -> Result<syn::Attribute, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_attribute(c, t)?;
        }
        t.pound_token = t.pound_token;
        t.style = self.fold_chain_attrstyle(c, t.style)?;
        t.bracket_token = t.bracket_token;
        t.path = self.fold_chain_path(c, t.path)?;
        t.tokens = t.tokens;
        for mut i in self.after() {
            t = i.chain_attribute(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_barefnarg(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::BareFnArg,
    ) -> Result<syn::BareFnArg, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_barefnarg(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.name = match t.name {
            Some(o) => Some((o.0, o.1)),
            None => None,
        };
        t.ty = self.fold_chain_type(c, t.ty)?;
        for mut i in self.after() {
            t = i.chain_barefnarg(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_binop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::BinOp,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_binop(c, t)?;
        }
        t = match t {
            syn::BinOp::Add(tmp) => self.fold_chain_binop_add(c, (tmp))?,
            syn::BinOp::Sub(tmp) => self.fold_chain_binop_sub(c, (tmp))?,
            syn::BinOp::Mul(tmp) => self.fold_chain_binop_mul(c, (tmp))?,
            syn::BinOp::Div(tmp) => self.fold_chain_binop_div(c, (tmp))?,
            syn::BinOp::Rem(tmp) => self.fold_chain_binop_rem(c, (tmp))?,
            syn::BinOp::And(tmp) => self.fold_chain_binop_and(c, (tmp))?,
            syn::BinOp::Or(tmp) => self.fold_chain_binop_or(c, (tmp))?,
            syn::BinOp::BitXor(tmp) => self.fold_chain_binop_bitxor(c, (tmp))?,
            syn::BinOp::BitAnd(tmp) => self.fold_chain_binop_bitand(c, (tmp))?,
            syn::BinOp::BitOr(tmp) => self.fold_chain_binop_bitor(c, (tmp))?,
            syn::BinOp::Shl(tmp) => self.fold_chain_binop_shl(c, (tmp))?,
            syn::BinOp::Shr(tmp) => self.fold_chain_binop_shr(c, (tmp))?,
            syn::BinOp::Eq(tmp) => self.fold_chain_binop_eq(c, (tmp))?,
            syn::BinOp::Lt(tmp) => self.fold_chain_binop_lt(c, (tmp))?,
            syn::BinOp::Le(tmp) => self.fold_chain_binop_le(c, (tmp))?,
            syn::BinOp::Ne(tmp) => self.fold_chain_binop_ne(c, (tmp))?,
            syn::BinOp::Ge(tmp) => self.fold_chain_binop_ge(c, (tmp))?,
            syn::BinOp::Gt(tmp) => self.fold_chain_binop_gt(c, (tmp))?,
            syn::BinOp::AddEq(tmp) => self.fold_chain_binop_addeq(c, (tmp))?,
            syn::BinOp::SubEq(tmp) => self.fold_chain_binop_subeq(c, (tmp))?,
            syn::BinOp::MulEq(tmp) => self.fold_chain_binop_muleq(c, (tmp))?,
            syn::BinOp::DivEq(tmp) => self.fold_chain_binop_diveq(c, (tmp))?,
            syn::BinOp::RemEq(tmp) => self.fold_chain_binop_remeq(c, (tmp))?,
            syn::BinOp::BitXorEq(tmp) => self.fold_chain_binop_bitxoreq(c, (tmp))?,
            syn::BinOp::BitAndEq(tmp) => self.fold_chain_binop_bitandeq(c, (tmp))?,
            syn::BinOp::BitOrEq(tmp) => self.fold_chain_binop_bitoreq(c, (tmp))?,
            syn::BinOp::ShlEq(tmp) => self.fold_chain_binop_shleq(c, (tmp))?,
            syn::BinOp::ShrEq(tmp) => self.fold_chain_binop_shreq(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_binop(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_binop_add(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Add,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_add(c, t)? {
                syn::BinOp::Add(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_add(c, t)? {
                syn::BinOp::Add(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Add(t))
    }
    fn fold_chain_binop_sub(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Sub,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_sub(c, t)? {
                syn::BinOp::Sub(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_sub(c, t)? {
                syn::BinOp::Sub(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Sub(t))
    }
    fn fold_chain_binop_mul(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Star,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_mul(c, t)? {
                syn::BinOp::Mul(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_mul(c, t)? {
                syn::BinOp::Mul(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Mul(t))
    }
    fn fold_chain_binop_div(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Div,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_div(c, t)? {
                syn::BinOp::Div(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_div(c, t)? {
                syn::BinOp::Div(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Div(t))
    }
    fn fold_chain_binop_rem(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Rem,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_rem(c, t)? {
                syn::BinOp::Rem(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_rem(c, t)? {
                syn::BinOp::Rem(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Rem(t))
    }
    fn fold_chain_binop_and(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::AndAnd,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_and(c, t)? {
                syn::BinOp::And(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_and(c, t)? {
                syn::BinOp::And(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::And(t))
    }
    fn fold_chain_binop_or(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::OrOr,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_or(c, t)? {
                syn::BinOp::Or(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_or(c, t)? {
                syn::BinOp::Or(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Or(t))
    }
    fn fold_chain_binop_bitxor(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Caret,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitxor(c, t)? {
                syn::BinOp::BitXor(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitxor(c, t)? {
                syn::BinOp::BitXor(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitXor(t))
    }
    fn fold_chain_binop_bitand(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::And,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitand(c, t)? {
                syn::BinOp::BitAnd(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitand(c, t)? {
                syn::BinOp::BitAnd(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitAnd(t))
    }
    fn fold_chain_binop_bitor(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Or,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitor(c, t)? {
                syn::BinOp::BitOr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitor(c, t)? {
                syn::BinOp::BitOr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitOr(t))
    }
    fn fold_chain_binop_shl(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Shl,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_shl(c, t)? {
                syn::BinOp::Shl(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_shl(c, t)? {
                syn::BinOp::Shl(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Shl(t))
    }
    fn fold_chain_binop_shr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Shr,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_shr(c, t)? {
                syn::BinOp::Shr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_shr(c, t)? {
                syn::BinOp::Shr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Shr(t))
    }
    fn fold_chain_binop_eq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::EqEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_eq(c, t)? {
                syn::BinOp::Eq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_eq(c, t)? {
                syn::BinOp::Eq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Eq(t))
    }
    fn fold_chain_binop_lt(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Lt,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_lt(c, t)? {
                syn::BinOp::Lt(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_lt(c, t)? {
                syn::BinOp::Lt(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Lt(t))
    }
    fn fold_chain_binop_le(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Le,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_le(c, t)? {
                syn::BinOp::Le(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_le(c, t)? {
                syn::BinOp::Le(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Le(t))
    }
    fn fold_chain_binop_ne(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Ne,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_ne(c, t)? {
                syn::BinOp::Ne(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_ne(c, t)? {
                syn::BinOp::Ne(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Ne(t))
    }
    fn fold_chain_binop_ge(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Ge,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_ge(c, t)? {
                syn::BinOp::Ge(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_ge(c, t)? {
                syn::BinOp::Ge(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Ge(t))
    }
    fn fold_chain_binop_gt(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Gt,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_gt(c, t)? {
                syn::BinOp::Gt(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_gt(c, t)? {
                syn::BinOp::Gt(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::Gt(t))
    }
    fn fold_chain_binop_addeq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::AddEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_addeq(c, t)? {
                syn::BinOp::AddEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_addeq(c, t)? {
                syn::BinOp::AddEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::AddEq(t))
    }
    fn fold_chain_binop_subeq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::SubEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_subeq(c, t)? {
                syn::BinOp::SubEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_subeq(c, t)? {
                syn::BinOp::SubEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::SubEq(t))
    }
    fn fold_chain_binop_muleq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::MulEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_muleq(c, t)? {
                syn::BinOp::MulEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_muleq(c, t)? {
                syn::BinOp::MulEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::MulEq(t))
    }
    fn fold_chain_binop_diveq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::DivEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_diveq(c, t)? {
                syn::BinOp::DivEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_diveq(c, t)? {
                syn::BinOp::DivEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::DivEq(t))
    }
    fn fold_chain_binop_remeq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::RemEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_remeq(c, t)? {
                syn::BinOp::RemEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_remeq(c, t)? {
                syn::BinOp::RemEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::RemEq(t))
    }
    fn fold_chain_binop_bitxoreq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::CaretEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitxoreq(c, t)? {
                syn::BinOp::BitXorEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitxoreq(c, t)? {
                syn::BinOp::BitXorEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitXorEq(t))
    }
    fn fold_chain_binop_bitandeq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::AndEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitandeq(c, t)? {
                syn::BinOp::BitAndEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitandeq(c, t)? {
                syn::BinOp::BitAndEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitAndEq(t))
    }
    fn fold_chain_binop_bitoreq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::OrEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_bitoreq(c, t)? {
                syn::BinOp::BitOrEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_bitoreq(c, t)? {
                syn::BinOp::BitOrEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::BitOrEq(t))
    }
    fn fold_chain_binop_shleq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::ShlEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_shleq(c, t)? {
                syn::BinOp::ShlEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_shleq(c, t)? {
                syn::BinOp::ShlEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::ShlEq(t))
    }
    fn fold_chain_binop_shreq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::ShrEq,
    ) -> Result<syn::BinOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_binop_shreq(c, t)? {
                syn::BinOp::ShrEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_binop_shreq(c, t)? {
                syn::BinOp::ShrEq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::BinOp::ShrEq(t))
    }
    fn fold_chain_binding(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Binding,
    ) -> Result<syn::Binding, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_binding(c, t)?;
        }
        t.ident = t.ident;
        t.eq_token = t.eq_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        for mut i in self.after() {
            t = i.chain_binding(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_block(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Block,
    ) -> Result<syn::Block, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_block(c, t)?;
        }
        t.brace_token = t.brace_token;
        t.stmts = {
            let mut tmp = vec![];
            for v in t.stmts {
                tmp.push(self.fold_chain_stmt(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_block(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_boundlifetimes(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::BoundLifetimes,
    ) -> Result<syn::BoundLifetimes, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_boundlifetimes(c, t)?;
        }
        t.for_token = t.for_token;
        t.lt_token = t.lt_token;
        t.lifetimes = t.lifetimes;
        t.gt_token = t.gt_token;
        for mut i in self.after() {
            t = i.chain_boundlifetimes(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_constparam(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ConstParam,
    ) -> Result<syn::ConstParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_constparam(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        t.eq_token = t.eq_token;
        t.default = match t.default {
            Some(o) => Some(self.fold_chain_expr(c, o)?),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_constparam(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_constraint(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Constraint,
    ) -> Result<syn::Constraint, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_constraint(c, t)?;
        }
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_constraint(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_data(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Data,
    ) -> Result<syn::Data, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_data(c, t)?;
        }
        t = match t {
            syn::Data::Struct(tmp) => self.fold_chain_data_struct(c, (tmp))?,
            syn::Data::Enum(tmp) => self.fold_chain_data_enum(c, (tmp))?,
            syn::Data::Union(tmp) => self.fold_chain_data_union(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_data(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_data_struct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataStruct,
    ) -> Result<syn::Data, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_data_struct(c, t)? {
                syn::Data::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_datastruct(c, t)?;
        for mut i in self.after() {
            t = match i.chain_data_struct(c, t)? {
                syn::Data::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Data::Struct(t))
    }
    fn fold_chain_data_enum(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataEnum,
    ) -> Result<syn::Data, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_data_enum(c, t)? {
                syn::Data::Enum(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_dataenum(c, t)?;
        for mut i in self.after() {
            t = match i.chain_data_enum(c, t)? {
                syn::Data::Enum(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Data::Enum(t))
    }
    fn fold_chain_data_union(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataUnion,
    ) -> Result<syn::Data, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_data_union(c, t)? {
                syn::Data::Union(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_dataunion(c, t)?;
        for mut i in self.after() {
            t = match i.chain_data_union(c, t)? {
                syn::Data::Union(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Data::Union(t))
    }
    fn fold_chain_dataenum(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataEnum,
    ) -> Result<syn::DataEnum, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_dataenum(c, t)?;
        }
        t.enum_token = t.enum_token;
        t.brace_token = t.brace_token;
        t.variants = t.variants;
        for mut i in self.after() {
            t = i.chain_dataenum(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_datastruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataStruct,
    ) -> Result<syn::DataStruct, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_datastruct(c, t)?;
        }
        t.struct_token = t.struct_token;
        t.fields = self.fold_chain_fields(c, t.fields)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_datastruct(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_dataunion(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DataUnion,
    ) -> Result<syn::DataUnion, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_dataunion(c, t)?;
        }
        t.union_token = t.union_token;
        t.fields = self.fold_chain_fieldsnamed(c, t.fields)?;
        for mut i in self.after() {
            t = i.chain_dataunion(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_deriveinput(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::DeriveInput,
    ) -> Result<syn::DeriveInput, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_deriveinput(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.data = self.fold_chain_data(c, t.data)?;
        for mut i in self.after() {
            t = i.chain_deriveinput(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_expr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Expr,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_expr(c, t)?;
        }
        t = match t {
            syn::Expr::Array(tmp) => self.fold_chain_expr_array(c, (tmp))?,
            syn::Expr::Assign(tmp) => self.fold_chain_expr_assign(c, (tmp))?,
            syn::Expr::AssignOp(tmp) => self.fold_chain_expr_assignop(c, (tmp))?,
            syn::Expr::Async(tmp) => self.fold_chain_expr_async(c, (tmp))?,
            syn::Expr::Await(tmp) => self.fold_chain_expr_await(c, (tmp))?,
            syn::Expr::Binary(tmp) => self.fold_chain_expr_binary(c, (tmp))?,
            syn::Expr::Block(tmp) => self.fold_chain_expr_block(c, (tmp))?,
            syn::Expr::Box(tmp) => self.fold_chain_expr_box(c, (tmp))?,
            syn::Expr::Break(tmp) => self.fold_chain_expr_break(c, (tmp))?,
            syn::Expr::Call(tmp) => self.fold_chain_expr_call(c, (tmp))?,
            syn::Expr::Cast(tmp) => self.fold_chain_expr_cast(c, (tmp))?,
            syn::Expr::Closure(tmp) => self.fold_chain_expr_closure(c, (tmp))?,
            syn::Expr::Continue(tmp) => self.fold_chain_expr_continue(c, (tmp))?,
            syn::Expr::Field(tmp) => self.fold_chain_expr_field(c, (tmp))?,
            syn::Expr::ForLoop(tmp) => self.fold_chain_expr_forloop(c, (tmp))?,
            syn::Expr::Group(tmp) => self.fold_chain_expr_group(c, (tmp))?,
            syn::Expr::If(tmp) => self.fold_chain_expr_if(c, (tmp))?,
            syn::Expr::Index(tmp) => self.fold_chain_expr_index(c, (tmp))?,
            syn::Expr::Let(tmp) => self.fold_chain_expr_let(c, (tmp))?,
            syn::Expr::Lit(tmp) => self.fold_chain_expr_lit(c, (tmp))?,
            syn::Expr::Loop(tmp) => self.fold_chain_expr_loop(c, (tmp))?,
            syn::Expr::Macro(tmp) => self.fold_chain_expr_macro(c, (tmp))?,
            syn::Expr::Match(tmp) => self.fold_chain_expr_match(c, (tmp))?,
            syn::Expr::MethodCall(tmp) => self.fold_chain_expr_methodcall(c, (tmp))?,
            syn::Expr::Paren(tmp) => self.fold_chain_expr_paren(c, (tmp))?,
            syn::Expr::Path(tmp) => self.fold_chain_expr_path(c, (tmp))?,
            syn::Expr::Range(tmp) => self.fold_chain_expr_range(c, (tmp))?,
            syn::Expr::Reference(tmp) => self.fold_chain_expr_reference(c, (tmp))?,
            syn::Expr::Repeat(tmp) => self.fold_chain_expr_repeat(c, (tmp))?,
            syn::Expr::Return(tmp) => self.fold_chain_expr_return(c, (tmp))?,
            syn::Expr::Struct(tmp) => self.fold_chain_expr_struct(c, (tmp))?,
            syn::Expr::Try(tmp) => self.fold_chain_expr_try(c, (tmp))?,
            syn::Expr::TryBlock(tmp) => self.fold_chain_expr_tryblock(c, (tmp))?,
            syn::Expr::Tuple(tmp) => self.fold_chain_expr_tuple(c, (tmp))?,
            syn::Expr::Type(tmp) => self.fold_chain_expr_type(c, (tmp))?,
            syn::Expr::Unary(tmp) => self.fold_chain_expr_unary(c, (tmp))?,
            syn::Expr::Unsafe(tmp) => self.fold_chain_expr_unsafe(c, (tmp))?,
            syn::Expr::Verbatim(tmp) => self.fold_chain_expr_verbatim(c, (tmp))?,
            syn::Expr::While(tmp) => self.fold_chain_expr_while(c, (tmp))?,
            syn::Expr::Yield(tmp) => self.fold_chain_expr_yield(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_expr(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_expr_array(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprArray,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_array(c, t)? {
                syn::Expr::Array(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprarray(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_array(c, t)? {
                syn::Expr::Array(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Array(t))
    }
    fn fold_chain_expr_assign(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAssign,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_assign(c, t)? {
                syn::Expr::Assign(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprassign(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_assign(c, t)? {
                syn::Expr::Assign(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Assign(t))
    }
    fn fold_chain_expr_assignop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAssignOp,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_assignop(c, t)? {
                syn::Expr::AssignOp(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprassignop(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_assignop(c, t)? {
                syn::Expr::AssignOp(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::AssignOp(t))
    }
    fn fold_chain_expr_async(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAsync,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_async(c, t)? {
                syn::Expr::Async(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprasync(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_async(c, t)? {
                syn::Expr::Async(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Async(t))
    }
    fn fold_chain_expr_await(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAwait,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_await(c, t)? {
                syn::Expr::Await(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprawait(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_await(c, t)? {
                syn::Expr::Await(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Await(t))
    }
    fn fold_chain_expr_binary(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_binary(c, t)? {
                syn::Expr::Binary(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprbinary(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_binary(c, t)? {
                syn::Expr::Binary(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Binary(t))
    }
    fn fold_chain_expr_block(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBlock,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_block(c, t)? {
                syn::Expr::Block(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprblock(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_block(c, t)? {
                syn::Expr::Block(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Block(t))
    }
    fn fold_chain_expr_box(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBox,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_box(c, t)? {
                syn::Expr::Box(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprbox(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_box(c, t)? {
                syn::Expr::Box(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Box(t))
    }
    fn fold_chain_expr_break(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBreak,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_break(c, t)? {
                syn::Expr::Break(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprbreak(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_break(c, t)? {
                syn::Expr::Break(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Break(t))
    }
    fn fold_chain_expr_call(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprCall,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_call(c, t)? {
                syn::Expr::Call(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprcall(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_call(c, t)? {
                syn::Expr::Call(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Call(t))
    }
    fn fold_chain_expr_cast(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprCast,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_cast(c, t)? {
                syn::Expr::Cast(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprcast(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_cast(c, t)? {
                syn::Expr::Cast(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Cast(t))
    }
    fn fold_chain_expr_closure(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprClosure,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_closure(c, t)? {
                syn::Expr::Closure(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprclosure(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_closure(c, t)? {
                syn::Expr::Closure(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Closure(t))
    }
    fn fold_chain_expr_continue(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprContinue,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_continue(c, t)? {
                syn::Expr::Continue(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprcontinue(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_continue(c, t)? {
                syn::Expr::Continue(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Continue(t))
    }
    fn fold_chain_expr_field(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprField,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_field(c, t)? {
                syn::Expr::Field(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprfield(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_field(c, t)? {
                syn::Expr::Field(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Field(t))
    }
    fn fold_chain_expr_forloop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprForLoop,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_forloop(c, t)? {
                syn::Expr::ForLoop(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprforloop(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_forloop(c, t)? {
                syn::Expr::ForLoop(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::ForLoop(t))
    }
    fn fold_chain_expr_group(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprGroup,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_group(c, t)? {
                syn::Expr::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprgroup(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_group(c, t)? {
                syn::Expr::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Group(t))
    }
    fn fold_chain_expr_if(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprIf,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_if(c, t)? {
                syn::Expr::If(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprif(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_if(c, t)? {
                syn::Expr::If(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::If(t))
    }
    fn fold_chain_expr_index(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprIndex,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_index(c, t)? {
                syn::Expr::Index(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprindex(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_index(c, t)? {
                syn::Expr::Index(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Index(t))
    }
    fn fold_chain_expr_let(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLet,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_let(c, t)? {
                syn::Expr::Let(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprlet(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_let(c, t)? {
                syn::Expr::Let(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Let(t))
    }
    fn fold_chain_expr_lit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLit,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_lit(c, t)? {
                syn::Expr::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprlit(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_lit(c, t)? {
                syn::Expr::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Lit(t))
    }
    fn fold_chain_expr_loop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLoop,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_loop(c, t)? {
                syn::Expr::Loop(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprloop(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_loop(c, t)? {
                syn::Expr::Loop(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Loop(t))
    }
    fn fold_chain_expr_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMacro,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_macro(c, t)? {
                syn::Expr::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_macro(c, t)? {
                syn::Expr::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Macro(t))
    }
    fn fold_chain_expr_match(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMatch,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_match(c, t)? {
                syn::Expr::Match(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprmatch(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_match(c, t)? {
                syn::Expr::Match(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Match(t))
    }
    fn fold_chain_expr_methodcall(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_methodcall(c, t)? {
                syn::Expr::MethodCall(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprmethodcall(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_methodcall(c, t)? {
                syn::Expr::MethodCall(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::MethodCall(t))
    }
    fn fold_chain_expr_paren(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprParen,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_paren(c, t)? {
                syn::Expr::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprparen(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_paren(c, t)? {
                syn::Expr::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Paren(t))
    }
    fn fold_chain_expr_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprPath,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_path(c, t)? {
                syn::Expr::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprpath(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_path(c, t)? {
                syn::Expr::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Path(t))
    }
    fn fold_chain_expr_range(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprRange,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_range(c, t)? {
                syn::Expr::Range(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprrange(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_range(c, t)? {
                syn::Expr::Range(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Range(t))
    }
    fn fold_chain_expr_reference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprReference,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_reference(c, t)? {
                syn::Expr::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprreference(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_reference(c, t)? {
                syn::Expr::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Reference(t))
    }
    fn fold_chain_expr_repeat(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprRepeat,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_repeat(c, t)? {
                syn::Expr::Repeat(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprrepeat(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_repeat(c, t)? {
                syn::Expr::Repeat(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Repeat(t))
    }
    fn fold_chain_expr_return(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprReturn,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_return(c, t)? {
                syn::Expr::Return(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprreturn(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_return(c, t)? {
                syn::Expr::Return(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Return(t))
    }
    fn fold_chain_expr_struct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprStruct,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_struct(c, t)? {
                syn::Expr::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprstruct(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_struct(c, t)? {
                syn::Expr::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Struct(t))
    }
    fn fold_chain_expr_try(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTry,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_try(c, t)? {
                syn::Expr::Try(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprtry(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_try(c, t)? {
                syn::Expr::Try(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Try(t))
    }
    fn fold_chain_expr_tryblock(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTryBlock,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_tryblock(c, t)? {
                syn::Expr::TryBlock(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprtryblock(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_tryblock(c, t)? {
                syn::Expr::TryBlock(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::TryBlock(t))
    }
    fn fold_chain_expr_tuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTuple,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_tuple(c, t)? {
                syn::Expr::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprtuple(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_tuple(c, t)? {
                syn::Expr::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Tuple(t))
    }
    fn fold_chain_expr_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprType,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_type(c, t)? {
                syn::Expr::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprtype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_type(c, t)? {
                syn::Expr::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Type(t))
    }
    fn fold_chain_expr_unary(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprUnary,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_unary(c, t)? {
                syn::Expr::Unary(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprunary(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_unary(c, t)? {
                syn::Expr::Unary(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Unary(t))
    }
    fn fold_chain_expr_unsafe(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprUnsafe,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_unsafe(c, t)? {
                syn::Expr::Unsafe(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprunsafe(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_unsafe(c, t)? {
                syn::Expr::Unsafe(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Unsafe(t))
    }
    fn fold_chain_expr_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_verbatim(c, t)? {
                syn::Expr::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_expr_verbatim(c, t)? {
                syn::Expr::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Verbatim(t))
    }
    fn fold_chain_expr_while(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprWhile,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_while(c, t)? {
                syn::Expr::While(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_exprwhile(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_while(c, t)? {
                syn::Expr::While(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::While(t))
    }
    fn fold_chain_expr_yield(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprYield,
    ) -> Result<syn::Expr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_expr_yield(c, t)? {
                syn::Expr::Yield(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_expryield(c, t)?;
        for mut i in self.after() {
            t = match i.chain_expr_yield(c, t)? {
                syn::Expr::Yield(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Expr::Yield(t))
    }
    fn fold_chain_exprarray(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprArray,
    ) -> Result<syn::ExprArray, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprarray(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t.elems = t.elems;
        for mut i in self.after() {
            t = i.chain_exprarray(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprassign(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAssign,
    ) -> Result<syn::ExprAssign, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprassign(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.left = Box::new(self.fold_chain_expr(c, *t.left)?);
        t.eq_token = t.eq_token;
        t.right = Box::new(self.fold_chain_expr(c, *t.right)?);
        for mut i in self.after() {
            t = i.chain_exprassign(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprassignop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAssignOp,
    ) -> Result<syn::ExprAssignOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprassignop(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.left = Box::new(self.fold_chain_expr(c, *t.left)?);
        t.op = self.fold_chain_binop(c, t.op)?;
        t.right = Box::new(self.fold_chain_expr(c, *t.right)?);
        for mut i in self.after() {
            t = i.chain_exprassignop(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprasync(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAsync,
    ) -> Result<syn::ExprAsync, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprasync(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.async_token = t.async_token;
        t.capture = t.capture;
        t.block = self.fold_chain_block(c, t.block)?;
        for mut i in self.after() {
            t = i.chain_exprasync(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprawait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprAwait,
    ) -> Result<syn::ExprAwait, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprawait(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.base = Box::new(self.fold_chain_expr(c, *t.base)?);
        t.dot_token = t.dot_token;
        t.await_token = t.await_token;
        for mut i in self.after() {
            t = i.chain_exprawait(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprbinary(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprbinary(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.left = Box::new(self.fold_chain_expr(c, *t.left)?);
        t.op = self.fold_chain_binop(c, t.op)?;
        t.right = Box::new(self.fold_chain_expr(c, *t.right)?);
        for mut i in self.after() {
            t = i.chain_exprbinary(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprblock(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBlock,
    ) -> Result<syn::ExprBlock, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprblock(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_label(c, o)?),
            None => None,
        };
        t.block = self.fold_chain_block(c, t.block)?;
        for mut i in self.after() {
            t = i.chain_exprblock(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprbox(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBox,
    ) -> Result<syn::ExprBox, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprbox(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.box_token = t.box_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprbox(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprbreak(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprBreak,
    ) -> Result<syn::ExprBreak, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprbreak(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.break_token = t.break_token;
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_lifetime(c, o)?),
            None => None,
        };
        t.expr = match t.expr {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprbreak(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprcall(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprCall,
    ) -> Result<syn::ExprCall, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprcall(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.func = Box::new(self.fold_chain_expr(c, *t.func)?);
        t.paren_token = t.paren_token;
        t.args = t.args;
        for mut i in self.after() {
            t = i.chain_exprcall(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprcast(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprCast,
    ) -> Result<syn::ExprCast, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprcast(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.as_token = t.as_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        for mut i in self.after() {
            t = i.chain_exprcast(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprclosure(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprClosure,
    ) -> Result<syn::ExprClosure, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprclosure(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.movability = t.movability;
        t.asyncness = t.asyncness;
        t.capture = t.capture;
        t.or1_token = t.or1_token;
        t.inputs = t.inputs;
        t.or2_token = t.or2_token;
        t.output = self.fold_chain_returntype(c, t.output)?;
        t.body = Box::new(self.fold_chain_expr(c, *t.body)?);
        for mut i in self.after() {
            t = i.chain_exprclosure(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprcontinue(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprContinue,
    ) -> Result<syn::ExprContinue, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprcontinue(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.continue_token = t.continue_token;
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_lifetime(c, o)?),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprcontinue(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprfield(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprField,
    ) -> Result<syn::ExprField, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprfield(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.base = Box::new(self.fold_chain_expr(c, *t.base)?);
        t.dot_token = t.dot_token;
        t.member = self.fold_chain_member(c, t.member)?;
        for mut i in self.after() {
            t = i.chain_exprfield(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprforloop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprForLoop,
    ) -> Result<syn::ExprForLoop, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprforloop(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_label(c, o)?),
            None => None,
        };
        t.for_token = t.for_token;
        t.pat = self.fold_chain_pat(c, t.pat)?;
        t.in_token = t.in_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.body = self.fold_chain_block(c, t.body)?;
        for mut i in self.after() {
            t = i.chain_exprforloop(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprgroup(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprGroup,
    ) -> Result<syn::ExprGroup, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprgroup(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.group_token = t.group_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprgroup(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprif(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprIf,
    ) -> Result<syn::ExprIf, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprif(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.if_token = t.if_token;
        t.cond = Box::new(self.fold_chain_expr(c, *t.cond)?);
        t.then_branch = self.fold_chain_block(c, t.then_branch)?;
        t.else_branch = match t.else_branch {
            Some(o) => Some((o.0, Box::new(self.fold_chain_expr(c, *o.1)?))),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprif(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprindex(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprIndex,
    ) -> Result<syn::ExprIndex, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprindex(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.bracket_token = t.bracket_token;
        t.index = Box::new(self.fold_chain_expr(c, *t.index)?);
        for mut i in self.after() {
            t = i.chain_exprindex(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprlet(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLet,
    ) -> Result<syn::ExprLet, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprlet(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.let_token = t.let_token;
        t.pat = self.fold_chain_pat(c, t.pat)?;
        t.eq_token = t.eq_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprlet(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprlit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLit,
    ) -> Result<syn::ExprLit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprlit(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.lit = self.fold_chain_lit(c, t.lit)?;
        for mut i in self.after() {
            t = i.chain_exprlit(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprloop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprLoop,
    ) -> Result<syn::ExprLoop, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprloop(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_label(c, o)?),
            None => None,
        };
        t.loop_token = t.loop_token;
        t.body = self.fold_chain_block(c, t.body)?;
        for mut i in self.after() {
            t = i.chain_exprloop(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMacro,
    ) -> Result<syn::ExprMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.mac = self.fold_chain_macro(c, t.mac)?;
        for mut i in self.after() {
            t = i.chain_exprmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprmatch(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMatch,
    ) -> Result<syn::ExprMatch, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprmatch(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.match_token = t.match_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.brace_token = t.brace_token;
        t.arms = {
            let mut tmp = vec![];
            for v in t.arms {
                tmp.push(self.fold_chain_arm(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_exprmatch(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprmethodcall(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::ExprMethodCall, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprmethodcall(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.receiver = Box::new(self.fold_chain_expr(c, *t.receiver)?);
        t.dot_token = t.dot_token;
        t.method = t.method;
        t.turbofish = match t.turbofish {
            Some(o) => Some(self.fold_chain_methodturbofish(c, o)?),
            None => None,
        };
        t.paren_token = t.paren_token;
        t.args = t.args;
        for mut i in self.after() {
            t = i.chain_exprmethodcall(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprparen(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprParen,
    ) -> Result<syn::ExprParen, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprparen(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprparen(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprpath(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprPath,
    ) -> Result<syn::ExprPath, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprpath(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.qself = match t.qself {
            Some(o) => Some(self.fold_chain_qself(c, o)?),
            None => None,
        };
        t.path = self.fold_chain_path(c, t.path)?;
        for mut i in self.after() {
            t = i.chain_exprpath(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprrange(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprRange,
    ) -> Result<syn::ExprRange, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprrange(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.from = match t.from {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        t.limits = self.fold_chain_rangelimits(c, t.limits)?;
        t.to = match t.to {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprrange(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprreference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprReference,
    ) -> Result<syn::ExprReference, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprreference(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.and_token = t.and_token;
        t.raw = t.raw;
        t.mutability = t.mutability;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprreference(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprrepeat(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprRepeat,
    ) -> Result<syn::ExprRepeat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprrepeat(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.semi_token = t.semi_token;
        t.len = Box::new(self.fold_chain_expr(c, *t.len)?);
        for mut i in self.after() {
            t = i.chain_exprrepeat(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprreturn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprReturn,
    ) -> Result<syn::ExprReturn, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprreturn(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.return_token = t.return_token;
        t.expr = match t.expr {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprreturn(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprstruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprStruct,
    ) -> Result<syn::ExprStruct, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprstruct(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.path = self.fold_chain_path(c, t.path)?;
        t.brace_token = t.brace_token;
        t.fields = t.fields;
        t.dot2_token = t.dot2_token;
        t.rest = match t.rest {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_exprstruct(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprtry(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTry,
    ) -> Result<syn::ExprTry, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprtry(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.question_token = t.question_token;
        for mut i in self.after() {
            t = i.chain_exprtry(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprtryblock(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTryBlock,
    ) -> Result<syn::ExprTryBlock, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprtryblock(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.try_token = t.try_token;
        t.block = self.fold_chain_block(c, t.block)?;
        for mut i in self.after() {
            t = i.chain_exprtryblock(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprtuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprTuple,
    ) -> Result<syn::ExprTuple, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprtuple(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t.elems = t.elems;
        for mut i in self.after() {
            t = i.chain_exprtuple(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprtype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprType,
    ) -> Result<syn::ExprType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprtype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.colon_token = t.colon_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        for mut i in self.after() {
            t = i.chain_exprtype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprunary(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprUnary,
    ) -> Result<syn::ExprUnary, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprunary(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.op = self.fold_chain_unop(c, t.op)?;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_exprunary(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprunsafe(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprUnsafe,
    ) -> Result<syn::ExprUnsafe, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprunsafe(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.unsafe_token = t.unsafe_token;
        t.block = self.fold_chain_block(c, t.block)?;
        for mut i in self.after() {
            t = i.chain_exprunsafe(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_exprwhile(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprWhile,
    ) -> Result<syn::ExprWhile, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_exprwhile(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.label = match t.label {
            Some(o) => Some(self.fold_chain_label(c, o)?),
            None => None,
        };
        t.while_token = t.while_token;
        t.cond = Box::new(self.fold_chain_expr(c, *t.cond)?);
        t.body = self.fold_chain_block(c, t.body)?;
        for mut i in self.after() {
            t = i.chain_exprwhile(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_expryield(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ExprYield,
    ) -> Result<syn::ExprYield, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_expryield(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.yield_token = t.yield_token;
        t.expr = match t.expr {
            Some(o) => Some(Box::new(self.fold_chain_expr(c, *o)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_expryield(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_field(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Field,
    ) -> Result<syn::Field, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_field(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        for mut i in self.after() {
            t = i.chain_field(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fieldpat(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldPat,
    ) -> Result<syn::FieldPat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fieldpat(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.member = self.fold_chain_member(c, t.member)?;
        t.colon_token = t.colon_token;
        t.pat = Box::new(self.fold_chain_pat(c, *t.pat)?);
        for mut i in self.after() {
            t = i.chain_fieldpat(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fieldvalue(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldValue,
    ) -> Result<syn::FieldValue, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fieldvalue(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.member = self.fold_chain_member(c, t.member)?;
        t.colon_token = t.colon_token;
        t.expr = self.fold_chain_expr(c, t.expr)?;
        for mut i in self.after() {
            t = i.chain_fieldvalue(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fields(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Fields,
    ) -> Result<syn::Fields, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fields(c, t)?;
        }
        t = match t {
            syn::Fields::Named(tmp) => self.fold_chain_fields_named(c, (tmp))?,
            syn::Fields::Unnamed(tmp) => self.fold_chain_fields_unnamed(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_fields(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fields_named(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldsNamed,
    ) -> Result<syn::Fields, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_fields_named(c, t)? {
                syn::Fields::Named(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_fieldsnamed(c, t)?;
        for mut i in self.after() {
            t = match i.chain_fields_named(c, t)? {
                syn::Fields::Named(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Fields::Named(t))
    }
    fn fold_chain_fields_unnamed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::Fields, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_fields_unnamed(c, t)? {
                syn::Fields::Unnamed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_fieldsunnamed(c, t)?;
        for mut i in self.after() {
            t = match i.chain_fields_unnamed(c, t)? {
                syn::Fields::Unnamed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Fields::Unnamed(t))
    }
    fn fold_chain_fieldsnamed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldsNamed,
    ) -> Result<syn::FieldsNamed, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fieldsnamed(c, t)?;
        }
        t.brace_token = t.brace_token;
        t.named = t.named;
        for mut i in self.after() {
            t = i.chain_fieldsnamed(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fieldsunnamed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::FieldsUnnamed, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fieldsunnamed(c, t)?;
        }
        t.paren_token = t.paren_token;
        t.unnamed = t.unnamed;
        for mut i in self.after() {
            t = i.chain_fieldsunnamed(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_file(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::File,
    ) -> Result<syn::File, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_file(c, t)?;
        }
        t.shebang = t.shebang;
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(self.fold_chain_item(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_file(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fnarg(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::FnArg,
    ) -> Result<syn::FnArg, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_fnarg(c, t)?;
        }
        t = match t {
            syn::FnArg::Receiver(tmp) => self.fold_chain_fnarg_receiver(c, (tmp))?,
            syn::FnArg::Typed(tmp) => self.fold_chain_fnarg_typed(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_fnarg(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_fnarg_receiver(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Receiver,
    ) -> Result<syn::FnArg, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_fnarg_receiver(c, t)? {
                syn::FnArg::Receiver(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_receiver(c, t)?;
        for mut i in self.after() {
            t = match i.chain_fnarg_receiver(c, t)? {
                syn::FnArg::Receiver(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::FnArg::Receiver(t))
    }
    fn fold_chain_fnarg_typed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatType,
    ) -> Result<syn::FnArg, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_fnarg_typed(c, t)? {
                syn::FnArg::Typed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_pattype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_fnarg_typed(c, t)? {
                syn::FnArg::Typed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::FnArg::Typed(t))
    }
    fn fold_chain_foreignitem(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItem,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_foreignitem(c, t)?;
        }
        t = match t {
            syn::ForeignItem::Fn(tmp) => self.fold_chain_foreignitem_fn(c, (tmp))?,
            syn::ForeignItem::Static(tmp) => self.fold_chain_foreignitem_static(c, (tmp))?,
            syn::ForeignItem::Type(tmp) => self.fold_chain_foreignitem_type(c, (tmp))?,
            syn::ForeignItem::Macro(tmp) => self.fold_chain_foreignitem_macro(c, (tmp))?,
            syn::ForeignItem::Verbatim(tmp) => self.fold_chain_foreignitem_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_foreignitem(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_foreignitem_fn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_foreignitem_fn(c, t)? {
                syn::ForeignItem::Fn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_foreignitemfn(c, t)?;
        for mut i in self.after() {
            t = match i.chain_foreignitem_fn(c, t)? {
                syn::ForeignItem::Fn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ForeignItem::Fn(t))
    }
    fn fold_chain_foreignitem_static(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_foreignitem_static(c, t)? {
                syn::ForeignItem::Static(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_foreignitemstatic(c, t)?;
        for mut i in self.after() {
            t = match i.chain_foreignitem_static(c, t)? {
                syn::ForeignItem::Static(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ForeignItem::Static(t))
    }
    fn fold_chain_foreignitem_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_foreignitem_type(c, t)? {
                syn::ForeignItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_foreignitemtype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_foreignitem_type(c, t)? {
                syn::ForeignItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ForeignItem::Type(t))
    }
    fn fold_chain_foreignitem_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_foreignitem_macro(c, t)? {
                syn::ForeignItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_foreignitemmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_foreignitem_macro(c, t)? {
                syn::ForeignItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ForeignItem::Macro(t))
    }
    fn fold_chain_foreignitem_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ForeignItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_foreignitem_verbatim(c, t)? {
                syn::ForeignItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_foreignitem_verbatim(c, t)? {
                syn::ForeignItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ForeignItem::Verbatim(t))
    }
    fn fold_chain_foreignitemfn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItemFn, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_foreignitemfn(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.sig = self.fold_chain_signature(c, t.sig)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_foreignitemfn(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_foreignitemmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItemMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_foreignitemmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.mac = self.fold_chain_macro(c, t.mac)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_foreignitemmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_foreignitemstatic(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItemStatic, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_foreignitemstatic(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.static_token = t.static_token;
        t.mutability = t.mutability;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_foreignitemstatic(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_foreignitemtype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItemType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_foreignitemtype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_foreignitemtype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_genericargument(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::GenericArgument,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_genericargument(c, t)?;
        }
        t = match t {
            syn::GenericArgument::Lifetime(tmp) => {
                self.fold_chain_genericargument_lifetime(c, (tmp))?
            }
            syn::GenericArgument::Type(tmp) => self.fold_chain_genericargument_type(c, (tmp))?,
            syn::GenericArgument::Binding(tmp) => {
                self.fold_chain_genericargument_binding(c, (tmp))?
            }
            syn::GenericArgument::Constraint(tmp) => {
                self.fold_chain_genericargument_constraint(c, (tmp))?
            }
            syn::GenericArgument::Const(tmp) => self.fold_chain_genericargument_const(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_genericargument(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_genericargument_lifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Lifetime,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericargument_lifetime(c, t)? {
                syn::GenericArgument::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_lifetime(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericargument_lifetime(c, t)? {
                syn::GenericArgument::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericArgument::Lifetime(t))
    }
    fn fold_chain_genericargument_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Type,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericargument_type(c, t)? {
                syn::GenericArgument::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_type(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericargument_type(c, t)? {
                syn::GenericArgument::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericArgument::Type(t))
    }
    fn fold_chain_genericargument_binding(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Binding,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericargument_binding(c, t)? {
                syn::GenericArgument::Binding(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_binding(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericargument_binding(c, t)? {
                syn::GenericArgument::Binding(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericArgument::Binding(t))
    }
    fn fold_chain_genericargument_constraint(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Constraint,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericargument_constraint(c, t)? {
                syn::GenericArgument::Constraint(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_constraint(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericargument_constraint(c, t)? {
                syn::GenericArgument::Constraint(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericArgument::Constraint(t))
    }
    fn fold_chain_genericargument_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Expr,
    ) -> Result<syn::GenericArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericargument_const(c, t)? {
                syn::GenericArgument::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_expr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericargument_const(c, t)? {
                syn::GenericArgument::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericArgument::Const(t))
    }
    fn fold_chain_genericmethodargument(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::GenericMethodArgument,
    ) -> Result<syn::GenericMethodArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_genericmethodargument(c, t)?;
        }
        t = match t {
            syn::GenericMethodArgument::Type(tmp) => {
                self.fold_chain_genericmethodargument_type(c, (tmp))?
            }
            syn::GenericMethodArgument::Const(tmp) => {
                self.fold_chain_genericmethodargument_const(c, (tmp))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_genericmethodargument(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_genericmethodargument_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Type,
    ) -> Result<syn::GenericMethodArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericmethodargument_type(c, t)? {
                syn::GenericMethodArgument::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_type(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericmethodargument_type(c, t)? {
                syn::GenericMethodArgument::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericMethodArgument::Type(t))
    }
    fn fold_chain_genericmethodargument_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Expr,
    ) -> Result<syn::GenericMethodArgument, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericmethodargument_const(c, t)? {
                syn::GenericMethodArgument::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_expr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericmethodargument_const(c, t)? {
                syn::GenericMethodArgument::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericMethodArgument::Const(t))
    }
    fn fold_chain_genericparam(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::GenericParam,
    ) -> Result<syn::GenericParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_genericparam(c, t)?;
        }
        t = match t {
            syn::GenericParam::Type(tmp) => self.fold_chain_genericparam_type(c, (tmp))?,
            syn::GenericParam::Lifetime(tmp) => self.fold_chain_genericparam_lifetime(c, (tmp))?,
            syn::GenericParam::Const(tmp) => self.fold_chain_genericparam_const(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_genericparam(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_genericparam_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeParam,
    ) -> Result<syn::GenericParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericparam_type(c, t)? {
                syn::GenericParam::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeparam(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericparam_type(c, t)? {
                syn::GenericParam::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericParam::Type(t))
    }
    fn fold_chain_genericparam_lifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LifetimeDef,
    ) -> Result<syn::GenericParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericparam_lifetime(c, t)? {
                syn::GenericParam::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_lifetimedef(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericparam_lifetime(c, t)? {
                syn::GenericParam::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericParam::Lifetime(t))
    }
    fn fold_chain_genericparam_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ConstParam,
    ) -> Result<syn::GenericParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_genericparam_const(c, t)? {
                syn::GenericParam::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_constparam(c, t)?;
        for mut i in self.after() {
            t = match i.chain_genericparam_const(c, t)? {
                syn::GenericParam::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::GenericParam::Const(t))
    }
    fn fold_chain_generics(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Generics,
    ) -> Result<syn::Generics, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_generics(c, t)?;
        }
        t.lt_token = t.lt_token;
        t.params = t.params;
        t.gt_token = t.gt_token;
        t.where_clause = match t.where_clause {
            Some(o) => Some(self.fold_chain_whereclause(c, o)?),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_generics(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_implitem(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItem,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_implitem(c, t)?;
        }
        t = match t {
            syn::ImplItem::Const(tmp) => self.fold_chain_implitem_const(c, (tmp))?,
            syn::ImplItem::Method(tmp) => self.fold_chain_implitem_method(c, (tmp))?,
            syn::ImplItem::Type(tmp) => self.fold_chain_implitem_type(c, (tmp))?,
            syn::ImplItem::Macro(tmp) => self.fold_chain_implitem_macro(c, (tmp))?,
            syn::ImplItem::Verbatim(tmp) => self.fold_chain_implitem_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_implitem(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_implitem_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_implitem_const(c, t)? {
                syn::ImplItem::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_implitemconst(c, t)?;
        for mut i in self.after() {
            t = match i.chain_implitem_const(c, t)? {
                syn::ImplItem::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ImplItem::Const(t))
    }
    fn fold_chain_implitem_method(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_implitem_method(c, t)? {
                syn::ImplItem::Method(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_implitemmethod(c, t)?;
        for mut i in self.after() {
            t = match i.chain_implitem_method(c, t)? {
                syn::ImplItem::Method(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ImplItem::Method(t))
    }
    fn fold_chain_implitem_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_implitem_type(c, t)? {
                syn::ImplItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_implitemtype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_implitem_type(c, t)? {
                syn::ImplItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ImplItem::Type(t))
    }
    fn fold_chain_implitem_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_implitem_macro(c, t)? {
                syn::ImplItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_implitemmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_implitem_macro(c, t)? {
                syn::ImplItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ImplItem::Macro(t))
    }
    fn fold_chain_implitem_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::ImplItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_implitem_verbatim(c, t)? {
                syn::ImplItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_implitem_verbatim(c, t)? {
                syn::ImplItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ImplItem::Verbatim(t))
    }
    fn fold_chain_implitemconst(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItemConst, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_implitemconst(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.defaultness = t.defaultness;
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        t.eq_token = t.eq_token;
        t.expr = self.fold_chain_expr(c, t.expr)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_implitemconst(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_implitemmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItemMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_implitemmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.mac = self.fold_chain_macro(c, t.mac)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_implitemmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_implitemmethod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemMethod,
    ) -> Result<syn::ImplItemMethod, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_implitemmethod(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.defaultness = t.defaultness;
        t.sig = self.fold_chain_signature(c, t.sig)?;
        t.block = self.fold_chain_block(c, t.block)?;
        for mut i in self.after() {
            t = i.chain_implitemmethod(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_implitemtype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItemType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_implitemtype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.defaultness = t.defaultness;
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.eq_token = t.eq_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_implitemtype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_index(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Index,
    ) -> Result<syn::Index, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_index(c, t)?;
        }
        t.index = t.index;
        t.span = t.span;
        for mut i in self.after() {
            t = i.chain_index(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_item(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Item,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_item(c, t)?;
        }
        t = match t {
            syn::Item::Const(tmp) => self.fold_chain_item_const(c, (tmp))?,
            syn::Item::Enum(tmp) => self.fold_chain_item_enum(c, (tmp))?,
            syn::Item::ExternCrate(tmp) => self.fold_chain_item_externcrate(c, (tmp))?,
            syn::Item::Fn(tmp) => self.fold_chain_item_fn(c, (tmp))?,
            syn::Item::ForeignMod(tmp) => self.fold_chain_item_foreignmod(c, (tmp))?,
            syn::Item::Impl(tmp) => self.fold_chain_item_impl(c, (tmp))?,
            syn::Item::Macro(tmp) => self.fold_chain_item_macro(c, (tmp))?,
            syn::Item::Macro2(tmp) => self.fold_chain_item_macro2(c, (tmp))?,
            syn::Item::Mod(tmp) => self.fold_chain_item_mod(c, (tmp))?,
            syn::Item::Static(tmp) => self.fold_chain_item_static(c, (tmp))?,
            syn::Item::Struct(tmp) => self.fold_chain_item_struct(c, (tmp))?,
            syn::Item::Trait(tmp) => self.fold_chain_item_trait(c, (tmp))?,
            syn::Item::TraitAlias(tmp) => self.fold_chain_item_traitalias(c, (tmp))?,
            syn::Item::Type(tmp) => self.fold_chain_item_type(c, (tmp))?,
            syn::Item::Union(tmp) => self.fold_chain_item_union(c, (tmp))?,
            syn::Item::Use(tmp) => self.fold_chain_item_use(c, (tmp))?,
            syn::Item::Verbatim(tmp) => self.fold_chain_item_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_item(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_item_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemConst,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_const(c, t)? {
                syn::Item::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemconst(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_const(c, t)? {
                syn::Item::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Const(t))
    }
    fn fold_chain_item_enum(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemEnum,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_enum(c, t)? {
                syn::Item::Enum(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemenum(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_enum(c, t)? {
                syn::Item::Enum(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Enum(t))
    }
    fn fold_chain_item_externcrate(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemExternCrate,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_externcrate(c, t)? {
                syn::Item::ExternCrate(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemexterncrate(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_externcrate(c, t)? {
                syn::Item::ExternCrate(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::ExternCrate(t))
    }
    fn fold_chain_item_fn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemFn,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_fn(c, t)? {
                syn::Item::Fn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemfn(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_fn(c, t)? {
                syn::Item::Fn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Fn(t))
    }
    fn fold_chain_item_foreignmod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemForeignMod,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_foreignmod(c, t)? {
                syn::Item::ForeignMod(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemforeignmod(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_foreignmod(c, t)? {
                syn::Item::ForeignMod(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::ForeignMod(t))
    }
    fn fold_chain_item_impl(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemImpl,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_impl(c, t)? {
                syn::Item::Impl(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemimpl(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_impl(c, t)? {
                syn::Item::Impl(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Impl(t))
    }
    fn fold_chain_item_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMacro,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_macro(c, t)? {
                syn::Item::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_macro(c, t)? {
                syn::Item::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Macro(t))
    }
    fn fold_chain_item_macro2(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMacro2,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_macro2(c, t)? {
                syn::Item::Macro2(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemmacro2(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_macro2(c, t)? {
                syn::Item::Macro2(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Macro2(t))
    }
    fn fold_chain_item_mod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMod,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_mod(c, t)? {
                syn::Item::Mod(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemmod(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_mod(c, t)? {
                syn::Item::Mod(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Mod(t))
    }
    fn fold_chain_item_static(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemStatic,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_static(c, t)? {
                syn::Item::Static(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemstatic(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_static(c, t)? {
                syn::Item::Static(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Static(t))
    }
    fn fold_chain_item_struct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemStruct,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_struct(c, t)? {
                syn::Item::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemstruct(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_struct(c, t)? {
                syn::Item::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Struct(t))
    }
    fn fold_chain_item_trait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemTrait,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_trait(c, t)? {
                syn::Item::Trait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemtrait(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_trait(c, t)? {
                syn::Item::Trait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Trait(t))
    }
    fn fold_chain_item_traitalias(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_traitalias(c, t)? {
                syn::Item::TraitAlias(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemtraitalias(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_traitalias(c, t)? {
                syn::Item::TraitAlias(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::TraitAlias(t))
    }
    fn fold_chain_item_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemType,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_type(c, t)? {
                syn::Item::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemtype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_type(c, t)? {
                syn::Item::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Type(t))
    }
    fn fold_chain_item_union(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemUnion,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_union(c, t)? {
                syn::Item::Union(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemunion(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_union(c, t)? {
                syn::Item::Union(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Union(t))
    }
    fn fold_chain_item_use(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemUse,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_use(c, t)? {
                syn::Item::Use(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_itemuse(c, t)?;
        for mut i in self.after() {
            t = match i.chain_item_use(c, t)? {
                syn::Item::Use(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Use(t))
    }
    fn fold_chain_item_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Item, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_item_verbatim(c, t)? {
                syn::Item::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_item_verbatim(c, t)? {
                syn::Item::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Item::Verbatim(t))
    }
    fn fold_chain_itemconst(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemConst,
    ) -> Result<syn::ItemConst, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemconst(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        t.eq_token = t.eq_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemconst(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemenum(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemEnum,
    ) -> Result<syn::ItemEnum, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemenum(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.enum_token = t.enum_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.brace_token = t.brace_token;
        t.variants = t.variants;
        for mut i in self.after() {
            t = i.chain_itemenum(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemexterncrate(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemExternCrate,
    ) -> Result<syn::ItemExternCrate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemexterncrate(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.extern_token = t.extern_token;
        t.crate_token = t.crate_token;
        t.ident = t.ident;
        t.rename = match t.rename {
            Some(o) => Some((o.0, o.1)),
            None => None,
        };
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemexterncrate(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemfn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemFn,
    ) -> Result<syn::ItemFn, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemfn(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.sig = self.fold_chain_signature(c, t.sig)?;
        t.block = Box::new(self.fold_chain_block(c, *t.block)?);
        for mut i in self.after() {
            t = i.chain_itemfn(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemforeignmod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemForeignMod,
    ) -> Result<syn::ItemForeignMod, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemforeignmod(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.abi = self.fold_chain_abi(c, t.abi)?;
        t.brace_token = t.brace_token;
        t.items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(self.fold_chain_foreignitem(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_itemforeignmod(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemimpl(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemImpl,
    ) -> Result<syn::ItemImpl, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemimpl(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.defaultness = t.defaultness;
        t.unsafety = t.unsafety;
        t.impl_token = t.impl_token;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.trait_ = match t.trait_ {
            Some(o) => Some((o.0, self.fold_chain_path(c, o.1)?, o.2)),
            None => None,
        };
        t.self_ty = Box::new(self.fold_chain_type(c, *t.self_ty)?);
        t.brace_token = t.brace_token;
        t.items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(self.fold_chain_implitem(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_itemimpl(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMacro,
    ) -> Result<syn::ItemMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.ident = t.ident;
        t.mac = self.fold_chain_macro(c, t.mac)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemmacro2(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMacro2,
    ) -> Result<syn::ItemMacro2, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemmacro2(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.macro_token = t.macro_token;
        t.ident = t.ident;
        t.rules = t.rules;
        for mut i in self.after() {
            t = i.chain_itemmacro2(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemmod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemMod,
    ) -> Result<syn::ItemMod, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemmod(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.mod_token = t.mod_token;
        t.ident = t.ident;
        t.content = match t.content {
            Some(o) => Some((o.0, {
                let mut tmp = vec![];
                for v in o.1 {
                    tmp.push(self.fold_chain_item(c, v)?);
                }
                tmp
            })),
            None => None,
        };
        t.semi = t.semi;
        for mut i in self.after() {
            t = i.chain_itemmod(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemstatic(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemStatic,
    ) -> Result<syn::ItemStatic, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemstatic(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.static_token = t.static_token;
        t.mutability = t.mutability;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        t.eq_token = t.eq_token;
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemstatic(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemstruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemStruct,
    ) -> Result<syn::ItemStruct, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemstruct(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.struct_token = t.struct_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.fields = self.fold_chain_fields(c, t.fields)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemstruct(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemtrait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemTrait,
    ) -> Result<syn::ItemTrait, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemtrait(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.unsafety = t.unsafety;
        t.auto_token = t.auto_token;
        t.trait_token = t.trait_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.colon_token = t.colon_token;
        t.supertraits = t.supertraits;
        t.brace_token = t.brace_token;
        t.items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(self.fold_chain_traititem(c, v)?);
            }
            tmp
        };
        for mut i in self.after() {
            t = i.chain_itemtrait(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemtraitalias(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::ItemTraitAlias, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemtraitalias(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.trait_token = t.trait_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.eq_token = t.eq_token;
        t.bounds = t.bounds;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemtraitalias(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemtype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemType,
    ) -> Result<syn::ItemType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemtype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.eq_token = t.eq_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemtype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemunion(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemUnion,
    ) -> Result<syn::ItemUnion, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemunion(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.union_token = t.union_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.fields = self.fold_chain_fieldsnamed(c, t.fields)?;
        for mut i in self.after() {
            t = i.chain_itemunion(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_itemuse(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ItemUse,
    ) -> Result<syn::ItemUse, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_itemuse(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.vis = self.fold_chain_visibility(c, t.vis)?;
        t.use_token = t.use_token;
        t.leading_colon = t.leading_colon;
        t.tree = self.fold_chain_usetree(c, t.tree)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_itemuse(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_label(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Label,
    ) -> Result<syn::Label, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_label(c, t)?;
        }
        t.name = self.fold_chain_lifetime(c, t.name)?;
        t.colon_token = t.colon_token;
        for mut i in self.after() {
            t = i.chain_label(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_lifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Lifetime,
    ) -> Result<syn::Lifetime, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_lifetime(c, t)?;
        }
        t.apostrophe = t.apostrophe;
        t.ident = t.ident;
        for mut i in self.after() {
            t = i.chain_lifetime(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_lifetimedef(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LifetimeDef,
    ) -> Result<syn::LifetimeDef, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_lifetimedef(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.lifetime = self.fold_chain_lifetime(c, t.lifetime)?;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_lifetimedef(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_lit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Lit,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_lit(c, t)?;
        }
        t = match t {
            syn::Lit::Str(tmp) => self.fold_chain_lit_str(c, (tmp))?,
            syn::Lit::ByteStr(tmp) => self.fold_chain_lit_bytestr(c, (tmp))?,
            syn::Lit::Byte(tmp) => self.fold_chain_lit_byte(c, (tmp))?,
            syn::Lit::Char(tmp) => self.fold_chain_lit_char(c, (tmp))?,
            syn::Lit::Int(tmp) => self.fold_chain_lit_int(c, (tmp))?,
            syn::Lit::Float(tmp) => self.fold_chain_lit_float(c, (tmp))?,
            syn::Lit::Bool(tmp) => self.fold_chain_lit_bool(c, (tmp))?,
            syn::Lit::Verbatim(tmp) => self.fold_chain_lit_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_lit(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_lit_str(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitStr,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_str(c, t)? {
                syn::Lit::Str(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litstr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_str(c, t)? {
                syn::Lit::Str(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Str(t))
    }
    fn fold_chain_lit_bytestr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitByteStr,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_bytestr(c, t)? {
                syn::Lit::ByteStr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litbytestr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_bytestr(c, t)? {
                syn::Lit::ByteStr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::ByteStr(t))
    }
    fn fold_chain_lit_byte(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitByte,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_byte(c, t)? {
                syn::Lit::Byte(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litbyte(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_byte(c, t)? {
                syn::Lit::Byte(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Byte(t))
    }
    fn fold_chain_lit_char(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitChar,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_char(c, t)? {
                syn::Lit::Char(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litchar(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_char(c, t)? {
                syn::Lit::Char(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Char(t))
    }
    fn fold_chain_lit_int(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitInt,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_int(c, t)? {
                syn::Lit::Int(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litint(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_int(c, t)? {
                syn::Lit::Int(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Int(t))
    }
    fn fold_chain_lit_float(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitFloat,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_float(c, t)? {
                syn::Lit::Float(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litfloat(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_float(c, t)? {
                syn::Lit::Float(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Float(t))
    }
    fn fold_chain_lit_bool(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitBool,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_bool(c, t)? {
                syn::Lit::Bool(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_litbool(c, t)?;
        for mut i in self.after() {
            t = match i.chain_lit_bool(c, t)? {
                syn::Lit::Bool(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Bool(t))
    }
    fn fold_chain_lit_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::Literal,
    ) -> Result<syn::Lit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_lit_verbatim(c, t)? {
                syn::Lit::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_lit_verbatim(c, t)? {
                syn::Lit::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Lit::Verbatim(t))
    }
    fn fold_chain_litbool(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitBool,
    ) -> Result<syn::LitBool, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litbool(c, t)?;
        }
        t.value = t.value;
        t.span = t.span;
        for mut i in self.after() {
            t = i.chain_litbool(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litbyte(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitByte,
    ) -> Result<syn::LitByte, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litbyte(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litbyte(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litbytestr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitByteStr,
    ) -> Result<syn::LitByteStr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litbytestr(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litbytestr(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litchar(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitChar,
    ) -> Result<syn::LitChar, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litchar(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litchar(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litfloat(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitFloat,
    ) -> Result<syn::LitFloat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litfloat(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litfloat(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litint(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitInt,
    ) -> Result<syn::LitInt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litint(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litint(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_litstr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::LitStr,
    ) -> Result<syn::LitStr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_litstr(c, t)?;
        }
        for mut i in self.after() {
            t = i.chain_litstr(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_local(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Local,
    ) -> Result<syn::Local, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_local(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.let_token = t.let_token;
        t.pat = self.fold_chain_pat(c, t.pat)?;
        t.init = match t.init {
            Some(o) => Some((o.0, Box::new(self.fold_chain_expr(c, *o.1)?))),
            None => None,
        };
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_local(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Macro,
    ) -> Result<syn::Macro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_macro(c, t)?;
        }
        t.path = self.fold_chain_path(c, t.path)?;
        t.bang_token = t.bang_token;
        t.delimiter = self.fold_chain_macrodelimiter(c, t.delimiter)?;
        t.tokens = t.tokens;
        for mut i in self.after() {
            t = i.chain_macro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_macrodelimiter(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MacroDelimiter,
    ) -> Result<syn::MacroDelimiter, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_macrodelimiter(c, t)?;
        }
        t = match t {
            syn::MacroDelimiter::Paren(tmp) => self.fold_chain_macrodelimiter_paren(c, (tmp))?,
            syn::MacroDelimiter::Brace(tmp) => self.fold_chain_macrodelimiter_brace(c, (tmp))?,
            syn::MacroDelimiter::Bracket(tmp) => {
                self.fold_chain_macrodelimiter_bracket(c, (tmp))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_macrodelimiter(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_macrodelimiter_paren(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Paren,
    ) -> Result<syn::MacroDelimiter, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_macrodelimiter_paren(c, t)? {
                syn::MacroDelimiter::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_macrodelimiter_paren(c, t)? {
                syn::MacroDelimiter::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::MacroDelimiter::Paren(t))
    }
    fn fold_chain_macrodelimiter_brace(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Brace,
    ) -> Result<syn::MacroDelimiter, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_macrodelimiter_brace(c, t)? {
                syn::MacroDelimiter::Brace(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_macrodelimiter_brace(c, t)? {
                syn::MacroDelimiter::Brace(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::MacroDelimiter::Brace(t))
    }
    fn fold_chain_macrodelimiter_bracket(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Bracket,
    ) -> Result<syn::MacroDelimiter, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_macrodelimiter_bracket(c, t)? {
                syn::MacroDelimiter::Bracket(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_macrodelimiter_bracket(c, t)? {
                syn::MacroDelimiter::Bracket(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::MacroDelimiter::Bracket(t))
    }
    fn fold_chain_member(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Member,
    ) -> Result<syn::Member, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_member(c, t)?;
        }
        t = match t {
            syn::Member::Named(tmp) => self.fold_chain_member_named(c, (tmp))?,
            syn::Member::Unnamed(tmp) => self.fold_chain_member_unnamed(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_member(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_member_named(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::Ident,
    ) -> Result<syn::Member, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_member_named(c, t)? {
                syn::Member::Named(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_member_named(c, t)? {
                syn::Member::Named(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Member::Named(t))
    }
    fn fold_chain_member_unnamed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Index,
    ) -> Result<syn::Member, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_member_unnamed(c, t)? {
                syn::Member::Unnamed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_index(c, t)?;
        for mut i in self.after() {
            t = match i.chain_member_unnamed(c, t)? {
                syn::Member::Unnamed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Member::Unnamed(t))
    }
    fn fold_chain_meta(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Meta,
    ) -> Result<syn::Meta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_meta(c, t)?;
        }
        t = match t {
            syn::Meta::Path(tmp) => self.fold_chain_meta_path(c, (tmp))?,
            syn::Meta::List(tmp) => self.fold_chain_meta_list(c, (tmp))?,
            syn::Meta::NameValue(tmp) => self.fold_chain_meta_namevalue(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_meta(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_meta_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Path,
    ) -> Result<syn::Meta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_meta_path(c, t)? {
                syn::Meta::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_path(c, t)?;
        for mut i in self.after() {
            t = match i.chain_meta_path(c, t)? {
                syn::Meta::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Meta::Path(t))
    }
    fn fold_chain_meta_list(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MetaList,
    ) -> Result<syn::Meta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_meta_list(c, t)? {
                syn::Meta::List(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_metalist(c, t)?;
        for mut i in self.after() {
            t = match i.chain_meta_list(c, t)? {
                syn::Meta::List(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Meta::List(t))
    }
    fn fold_chain_meta_namevalue(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MetaNameValue,
    ) -> Result<syn::Meta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_meta_namevalue(c, t)? {
                syn::Meta::NameValue(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_metanamevalue(c, t)?;
        for mut i in self.after() {
            t = match i.chain_meta_namevalue(c, t)? {
                syn::Meta::NameValue(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Meta::NameValue(t))
    }
    fn fold_chain_metalist(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MetaList,
    ) -> Result<syn::MetaList, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_metalist(c, t)?;
        }
        t.path = self.fold_chain_path(c, t.path)?;
        t.paren_token = t.paren_token;
        t.nested = t.nested;
        for mut i in self.after() {
            t = i.chain_metalist(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_metanamevalue(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MetaNameValue,
    ) -> Result<syn::MetaNameValue, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_metanamevalue(c, t)?;
        }
        t.path = self.fold_chain_path(c, t.path)?;
        t.eq_token = t.eq_token;
        t.lit = self.fold_chain_lit(c, t.lit)?;
        for mut i in self.after() {
            t = i.chain_metanamevalue(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_methodturbofish(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::MethodTurbofish,
    ) -> Result<syn::MethodTurbofish, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_methodturbofish(c, t)?;
        }
        t.colon2_token = t.colon2_token;
        t.lt_token = t.lt_token;
        t.args = t.args;
        t.gt_token = t.gt_token;
        for mut i in self.after() {
            t = i.chain_methodturbofish(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_nestedmeta(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::NestedMeta,
    ) -> Result<syn::NestedMeta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_nestedmeta(c, t)?;
        }
        t = match t {
            syn::NestedMeta::Meta(tmp) => self.fold_chain_nestedmeta_meta(c, (tmp))?,
            syn::NestedMeta::Lit(tmp) => self.fold_chain_nestedmeta_lit(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_nestedmeta(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_nestedmeta_meta(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Meta,
    ) -> Result<syn::NestedMeta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_nestedmeta_meta(c, t)? {
                syn::NestedMeta::Meta(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_meta(c, t)?;
        for mut i in self.after() {
            t = match i.chain_nestedmeta_meta(c, t)? {
                syn::NestedMeta::Meta(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::NestedMeta::Meta(t))
    }
    fn fold_chain_nestedmeta_lit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Lit,
    ) -> Result<syn::NestedMeta, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_nestedmeta_lit(c, t)? {
                syn::NestedMeta::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_lit(c, t)?;
        for mut i in self.after() {
            t = match i.chain_nestedmeta_lit(c, t)? {
                syn::NestedMeta::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::NestedMeta::Lit(t))
    }
    fn fold_chain_parenthesizedgenericarguments(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::ParenthesizedGenericArguments, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_parenthesizedgenericarguments(c, t)?;
        }
        t.paren_token = t.paren_token;
        t.inputs = t.inputs;
        t.output = self.fold_chain_returntype(c, t.output)?;
        for mut i in self.after() {
            t = i.chain_parenthesizedgenericarguments(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pat(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Pat,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pat(c, t)?;
        }
        t = match t {
            syn::Pat::Box(tmp) => self.fold_chain_pat_box(c, (tmp))?,
            syn::Pat::Ident(tmp) => self.fold_chain_pat_ident(c, (tmp))?,
            syn::Pat::Lit(tmp) => self.fold_chain_pat_lit(c, (tmp))?,
            syn::Pat::Macro(tmp) => self.fold_chain_pat_macro(c, (tmp))?,
            syn::Pat::Or(tmp) => self.fold_chain_pat_or(c, (tmp))?,
            syn::Pat::Path(tmp) => self.fold_chain_pat_path(c, (tmp))?,
            syn::Pat::Range(tmp) => self.fold_chain_pat_range(c, (tmp))?,
            syn::Pat::Reference(tmp) => self.fold_chain_pat_reference(c, (tmp))?,
            syn::Pat::Rest(tmp) => self.fold_chain_pat_rest(c, (tmp))?,
            syn::Pat::Slice(tmp) => self.fold_chain_pat_slice(c, (tmp))?,
            syn::Pat::Struct(tmp) => self.fold_chain_pat_struct(c, (tmp))?,
            syn::Pat::Tuple(tmp) => self.fold_chain_pat_tuple(c, (tmp))?,
            syn::Pat::TupleStruct(tmp) => self.fold_chain_pat_tuplestruct(c, (tmp))?,
            syn::Pat::Type(tmp) => self.fold_chain_pat_type(c, (tmp))?,
            syn::Pat::Verbatim(tmp) => self.fold_chain_pat_verbatim(c, (tmp))?,
            syn::Pat::Wild(tmp) => self.fold_chain_pat_wild(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_pat(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pat_box(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatBox,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_box(c, t)? {
                syn::Pat::Box(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patbox(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_box(c, t)? {
                syn::Pat::Box(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Box(t))
    }
    fn fold_chain_pat_ident(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatIdent,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_ident(c, t)? {
                syn::Pat::Ident(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patident(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_ident(c, t)? {
                syn::Pat::Ident(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Ident(t))
    }
    fn fold_chain_pat_lit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatLit,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_lit(c, t)? {
                syn::Pat::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patlit(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_lit(c, t)? {
                syn::Pat::Lit(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Lit(t))
    }
    fn fold_chain_pat_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatMacro,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_macro(c, t)? {
                syn::Pat::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_macro(c, t)? {
                syn::Pat::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Macro(t))
    }
    fn fold_chain_pat_or(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatOr,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_or(c, t)? {
                syn::Pat::Or(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_pator(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_or(c, t)? {
                syn::Pat::Or(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Or(t))
    }
    fn fold_chain_pat_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatPath,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_path(c, t)? {
                syn::Pat::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patpath(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_path(c, t)? {
                syn::Pat::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Path(t))
    }
    fn fold_chain_pat_range(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatRange,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_range(c, t)? {
                syn::Pat::Range(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patrange(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_range(c, t)? {
                syn::Pat::Range(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Range(t))
    }
    fn fold_chain_pat_reference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatReference,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_reference(c, t)? {
                syn::Pat::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patreference(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_reference(c, t)? {
                syn::Pat::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Reference(t))
    }
    fn fold_chain_pat_rest(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatRest,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_rest(c, t)? {
                syn::Pat::Rest(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patrest(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_rest(c, t)? {
                syn::Pat::Rest(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Rest(t))
    }
    fn fold_chain_pat_slice(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatSlice,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_slice(c, t)? {
                syn::Pat::Slice(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patslice(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_slice(c, t)? {
                syn::Pat::Slice(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Slice(t))
    }
    fn fold_chain_pat_struct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatStruct,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_struct(c, t)? {
                syn::Pat::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patstruct(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_struct(c, t)? {
                syn::Pat::Struct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Struct(t))
    }
    fn fold_chain_pat_tuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatTuple,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_tuple(c, t)? {
                syn::Pat::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_pattuple(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_tuple(c, t)? {
                syn::Pat::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Tuple(t))
    }
    fn fold_chain_pat_tuplestruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatTupleStruct,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_tuplestruct(c, t)? {
                syn::Pat::TupleStruct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_pattuplestruct(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_tuplestruct(c, t)? {
                syn::Pat::TupleStruct(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::TupleStruct(t))
    }
    fn fold_chain_pat_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatType,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_type(c, t)? {
                syn::Pat::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_pattype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_type(c, t)? {
                syn::Pat::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Type(t))
    }
    fn fold_chain_pat_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_verbatim(c, t)? {
                syn::Pat::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_pat_verbatim(c, t)? {
                syn::Pat::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Verbatim(t))
    }
    fn fold_chain_pat_wild(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatWild,
    ) -> Result<syn::Pat, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_pat_wild(c, t)? {
                syn::Pat::Wild(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_patwild(c, t)?;
        for mut i in self.after() {
            t = match i.chain_pat_wild(c, t)? {
                syn::Pat::Wild(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Pat::Wild(t))
    }
    fn fold_chain_patbox(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatBox,
    ) -> Result<syn::PatBox, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patbox(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.box_token = t.box_token;
        t.pat = Box::new(self.fold_chain_pat(c, *t.pat)?);
        for mut i in self.after() {
            t = i.chain_patbox(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patident(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatIdent,
    ) -> Result<syn::PatIdent, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patident(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.by_ref = t.by_ref;
        t.mutability = t.mutability;
        t.ident = t.ident;
        t.subpat = match t.subpat {
            Some(o) => Some((o.0, Box::new(self.fold_chain_pat(c, *o.1)?))),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_patident(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patlit(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatLit,
    ) -> Result<syn::PatLit, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patlit(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.expr = Box::new(self.fold_chain_expr(c, *t.expr)?);
        for mut i in self.after() {
            t = i.chain_patlit(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatMacro,
    ) -> Result<syn::PatMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.mac = self.fold_chain_macro(c, t.mac)?;
        for mut i in self.after() {
            t = i.chain_patmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pator(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatOr,
    ) -> Result<syn::PatOr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pator(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.leading_vert = t.leading_vert;
        t.cases = t.cases;
        for mut i in self.after() {
            t = i.chain_pator(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patpath(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatPath,
    ) -> Result<syn::PatPath, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patpath(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.qself = match t.qself {
            Some(o) => Some(self.fold_chain_qself(c, o)?),
            None => None,
        };
        t.path = self.fold_chain_path(c, t.path)?;
        for mut i in self.after() {
            t = i.chain_patpath(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patrange(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatRange,
    ) -> Result<syn::PatRange, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patrange(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.lo = Box::new(self.fold_chain_expr(c, *t.lo)?);
        t.limits = self.fold_chain_rangelimits(c, t.limits)?;
        t.hi = Box::new(self.fold_chain_expr(c, *t.hi)?);
        for mut i in self.after() {
            t = i.chain_patrange(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patreference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatReference,
    ) -> Result<syn::PatReference, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patreference(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.and_token = t.and_token;
        t.mutability = t.mutability;
        t.pat = Box::new(self.fold_chain_pat(c, *t.pat)?);
        for mut i in self.after() {
            t = i.chain_patreference(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patrest(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatRest,
    ) -> Result<syn::PatRest, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patrest(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.dot2_token = t.dot2_token;
        for mut i in self.after() {
            t = i.chain_patrest(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patslice(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatSlice,
    ) -> Result<syn::PatSlice, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patslice(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t.elems = t.elems;
        for mut i in self.after() {
            t = i.chain_patslice(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patstruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatStruct,
    ) -> Result<syn::PatStruct, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patstruct(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.path = self.fold_chain_path(c, t.path)?;
        t.brace_token = t.brace_token;
        t.fields = t.fields;
        t.dot2_token = t.dot2_token;
        for mut i in self.after() {
            t = i.chain_patstruct(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pattuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatTuple,
    ) -> Result<syn::PatTuple, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pattuple(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t.elems = t.elems;
        for mut i in self.after() {
            t = i.chain_pattuple(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pattuplestruct(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatTupleStruct,
    ) -> Result<syn::PatTupleStruct, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pattuplestruct(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.path = self.fold_chain_path(c, t.path)?;
        t.pat = self.fold_chain_pattuple(c, t.pat)?;
        for mut i in self.after() {
            t = i.chain_pattuplestruct(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_pattype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatType,
    ) -> Result<syn::PatType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pattype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.pat = Box::new(self.fold_chain_pat(c, *t.pat)?);
        t.colon_token = t.colon_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        for mut i in self.after() {
            t = i.chain_pattype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patwild(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PatWild,
    ) -> Result<syn::PatWild, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patwild(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.underscore_token = t.underscore_token;
        for mut i in self.after() {
            t = i.chain_patwild(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Path,
    ) -> Result<syn::Path, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_path(c, t)?;
        }
        t.leading_colon = t.leading_colon;
        t.segments = t.segments;
        for mut i in self.after() {
            t = i.chain_path(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patharguments(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PathArguments,
    ) -> Result<syn::PathArguments, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_patharguments(c, t)?;
        }
        t = match t {
            syn::PathArguments::AngleBracketed(tmp) => {
                self.fold_chain_patharguments_anglebracketed(c, (tmp))?
            }
            syn::PathArguments::Parenthesized(tmp) => {
                self.fold_chain_patharguments_parenthesized(c, (tmp))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_patharguments(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_patharguments_anglebracketed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::PathArguments, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_patharguments_anglebracketed(c, t)? {
                syn::PathArguments::AngleBracketed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_anglebracketedgenericarguments(c, t)?;
        for mut i in self.after() {
            t = match i.chain_patharguments_anglebracketed(c, t)? {
                syn::PathArguments::AngleBracketed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::PathArguments::AngleBracketed(t))
    }
    fn fold_chain_patharguments_parenthesized(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::PathArguments, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_patharguments_parenthesized(c, t)? {
                syn::PathArguments::Parenthesized(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_parenthesizedgenericarguments(c, t)?;
        for mut i in self.after() {
            t = match i.chain_patharguments_parenthesized(c, t)? {
                syn::PathArguments::Parenthesized(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::PathArguments::Parenthesized(t))
    }
    fn fold_chain_pathsegment(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PathSegment,
    ) -> Result<syn::PathSegment, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_pathsegment(c, t)?;
        }
        t.ident = t.ident;
        t.arguments = self.fold_chain_patharguments(c, t.arguments)?;
        for mut i in self.after() {
            t = i.chain_pathsegment(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_predicateeq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateEq,
    ) -> Result<syn::PredicateEq, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_predicateeq(c, t)?;
        }
        t.lhs_ty = self.fold_chain_type(c, t.lhs_ty)?;
        t.eq_token = t.eq_token;
        t.rhs_ty = self.fold_chain_type(c, t.rhs_ty)?;
        for mut i in self.after() {
            t = i.chain_predicateeq(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_predicatelifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateLifetime,
    ) -> Result<syn::PredicateLifetime, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_predicatelifetime(c, t)?;
        }
        t.lifetime = self.fold_chain_lifetime(c, t.lifetime)?;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_predicatelifetime(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_predicatetype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateType,
    ) -> Result<syn::PredicateType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_predicatetype(c, t)?;
        }
        t.lifetimes = match t.lifetimes {
            Some(o) => Some(self.fold_chain_boundlifetimes(c, o)?),
            None => None,
        };
        t.bounded_ty = self.fold_chain_type(c, t.bounded_ty)?;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_predicatetype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_qself(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::QSelf,
    ) -> Result<syn::QSelf, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_qself(c, t)?;
        }
        t.lt_token = t.lt_token;
        t.ty = Box::new(self.fold_chain_type(c, *t.ty)?);
        t.position = t.position;
        t.as_token = t.as_token;
        t.gt_token = t.gt_token;
        for mut i in self.after() {
            t = i.chain_qself(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_rangelimits(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::RangeLimits,
    ) -> Result<syn::RangeLimits, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_rangelimits(c, t)?;
        }
        t = match t {
            syn::RangeLimits::HalfOpen(tmp) => self.fold_chain_rangelimits_halfopen(c, (tmp))?,
            syn::RangeLimits::Closed(tmp) => self.fold_chain_rangelimits_closed(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_rangelimits(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_rangelimits_halfopen(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Dot2,
    ) -> Result<syn::RangeLimits, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_rangelimits_halfopen(c, t)? {
                syn::RangeLimits::HalfOpen(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_rangelimits_halfopen(c, t)? {
                syn::RangeLimits::HalfOpen(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::RangeLimits::HalfOpen(t))
    }
    fn fold_chain_rangelimits_closed(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::DotDotEq,
    ) -> Result<syn::RangeLimits, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_rangelimits_closed(c, t)? {
                syn::RangeLimits::Closed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_rangelimits_closed(c, t)? {
                syn::RangeLimits::Closed(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::RangeLimits::Closed(t))
    }
    fn fold_chain_receiver(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Receiver,
    ) -> Result<syn::Receiver, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_receiver(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.reference = match t.reference {
            Some(o) => Some((
                o.0,
                match o.1 {
                    Some(o) => Some(self.fold_chain_lifetime(c, o)?),
                    None => None,
                },
            )),
            None => None,
        };
        t.mutability = t.mutability;
        t.self_token = t.self_token;
        for mut i in self.after() {
            t = i.chain_receiver(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_returntype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::ReturnType,
    ) -> Result<syn::ReturnType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_returntype(c, t)?;
        }
        t = match t {
            syn::ReturnType::Type(tmp_0, tmp_1) => {
                self.fold_chain_returntype_type(c, (tmp_0, tmp_1))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_returntype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_returntype_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<syn::ReturnType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_returntype_type(c, t)? {
                syn::ReturnType::Type(tmp_0, tmp_1) => (tmp_0, tmp_1),
                tmp => return Ok(tmp),
            };
        }
        t.0 = t.0;
        t.1 = Box::new(self.fold_chain_type(c, *t.1)?);
        for mut i in self.after() {
            t = match i.chain_returntype_type(c, t)? {
                syn::ReturnType::Type(tmp_0, tmp_1) => (tmp_0, tmp_1),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::ReturnType::Type(t.0, t.1))
    }
    fn fold_chain_signature(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Signature,
    ) -> Result<syn::Signature, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_signature(c, t)?;
        }
        t.constness = t.constness;
        t.asyncness = t.asyncness;
        t.unsafety = t.unsafety;
        t.abi = match t.abi {
            Some(o) => Some(self.fold_chain_abi(c, o)?),
            None => None,
        };
        t.fn_token = t.fn_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.paren_token = t.paren_token;
        t.inputs = t.inputs;
        t.variadic = match t.variadic {
            Some(o) => Some(self.fold_chain_variadic(c, o)?),
            None => None,
        };
        t.output = self.fold_chain_returntype(c, t.output)?;
        for mut i in self.after() {
            t = i.chain_signature(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_stmt(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Stmt,
    ) -> Result<syn::Stmt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_stmt(c, t)?;
        }
        t = match t {
            syn::Stmt::Local(tmp) => self.fold_chain_stmt_local(c, (tmp))?,
            syn::Stmt::Item(tmp) => self.fold_chain_stmt_item(c, (tmp))?,
            syn::Stmt::Expr(tmp) => self.fold_chain_stmt_expr(c, (tmp))?,
            syn::Stmt::Semi(tmp_0, tmp_1) => self.fold_chain_stmt_semi(c, (tmp_0, tmp_1))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_stmt(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_stmt_local(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Local,
    ) -> Result<syn::Stmt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_stmt_local(c, t)? {
                syn::Stmt::Local(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_local(c, t)?;
        for mut i in self.after() {
            t = match i.chain_stmt_local(c, t)? {
                syn::Stmt::Local(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Stmt::Local(t))
    }
    fn fold_chain_stmt_item(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Item,
    ) -> Result<syn::Stmt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_stmt_item(c, t)? {
                syn::Stmt::Item(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_item(c, t)?;
        for mut i in self.after() {
            t = match i.chain_stmt_item(c, t)? {
                syn::Stmt::Item(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Stmt::Item(t))
    }
    fn fold_chain_stmt_expr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Expr,
    ) -> Result<syn::Stmt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_stmt_expr(c, t)? {
                syn::Stmt::Expr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_expr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_stmt_expr(c, t)? {
                syn::Stmt::Expr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Stmt::Expr(t))
    }
    fn fold_chain_stmt_semi(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: (syn::Expr, syn::token::Semi),
    ) -> Result<syn::Stmt, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_stmt_semi(c, t)? {
                syn::Stmt::Semi(tmp_0, tmp_1) => (tmp_0, tmp_1),
                tmp => return Ok(tmp),
            };
        }
        t.0 = self.fold_chain_expr(c, t.0)?;
        t.1 = t.1;
        for mut i in self.after() {
            t = match i.chain_stmt_semi(c, t)? {
                syn::Stmt::Semi(tmp_0, tmp_1) => (tmp_0, tmp_1),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Stmt::Semi(t.0, t.1))
    }
    fn fold_chain_traitbound(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitBound,
    ) -> Result<syn::TraitBound, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traitbound(c, t)?;
        }
        t.paren_token = t.paren_token;
        t.modifier = self.fold_chain_traitboundmodifier(c, t.modifier)?;
        t.lifetimes = match t.lifetimes {
            Some(o) => Some(self.fold_chain_boundlifetimes(c, o)?),
            None => None,
        };
        t.path = self.fold_chain_path(c, t.path)?;
        for mut i in self.after() {
            t = i.chain_traitbound(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traitboundmodifier(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitBoundModifier,
    ) -> Result<syn::TraitBoundModifier, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traitboundmodifier(c, t)?;
        }
        t = match t {
            syn::TraitBoundModifier::Maybe(tmp) => {
                self.fold_chain_traitboundmodifier_maybe(c, (tmp))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_traitboundmodifier(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traitboundmodifier_maybe(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Question,
    ) -> Result<syn::TraitBoundModifier, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traitboundmodifier_maybe(c, t)? {
                syn::TraitBoundModifier::Maybe(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_traitboundmodifier_maybe(c, t)? {
                syn::TraitBoundModifier::Maybe(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitBoundModifier::Maybe(t))
    }
    fn fold_chain_traititem(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItem,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traititem(c, t)?;
        }
        t = match t {
            syn::TraitItem::Const(tmp) => self.fold_chain_traititem_const(c, (tmp))?,
            syn::TraitItem::Method(tmp) => self.fold_chain_traititem_method(c, (tmp))?,
            syn::TraitItem::Type(tmp) => self.fold_chain_traititem_type(c, (tmp))?,
            syn::TraitItem::Macro(tmp) => self.fold_chain_traititem_macro(c, (tmp))?,
            syn::TraitItem::Verbatim(tmp) => self.fold_chain_traititem_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_traititem(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traititem_const(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traititem_const(c, t)? {
                syn::TraitItem::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_traititemconst(c, t)?;
        for mut i in self.after() {
            t = match i.chain_traititem_const(c, t)? {
                syn::TraitItem::Const(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitItem::Const(t))
    }
    fn fold_chain_traititem_method(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traititem_method(c, t)? {
                syn::TraitItem::Method(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_traititemmethod(c, t)?;
        for mut i in self.after() {
            t = match i.chain_traititem_method(c, t)? {
                syn::TraitItem::Method(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitItem::Method(t))
    }
    fn fold_chain_traititem_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traititem_type(c, t)? {
                syn::TraitItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_traititemtype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_traititem_type(c, t)? {
                syn::TraitItem::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitItem::Type(t))
    }
    fn fold_chain_traititem_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traititem_macro(c, t)? {
                syn::TraitItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_traititemmacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_traititem_macro(c, t)? {
                syn::TraitItem::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitItem::Macro(t))
    }
    fn fold_chain_traititem_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::TraitItem, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_traititem_verbatim(c, t)? {
                syn::TraitItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_traititem_verbatim(c, t)? {
                syn::TraitItem::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TraitItem::Verbatim(t))
    }
    fn fold_chain_traititemconst(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItemConst, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traititemconst(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = self.fold_chain_type(c, t.ty)?;
        t.default = match t.default {
            Some(o) => Some((o.0, self.fold_chain_expr(c, o.1)?)),
            None => None,
        };
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_traititemconst(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traititemmacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItemMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traititemmacro(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.mac = self.fold_chain_macro(c, t.mac)?;
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_traititemmacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traititemmethod(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemMethod,
    ) -> Result<syn::TraitItemMethod, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traititemmethod(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.sig = self.fold_chain_signature(c, t.sig)?;
        t.default = match t.default {
            Some(o) => Some(self.fold_chain_block(c, o)?),
            None => None,
        };
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_traititemmethod(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_traititemtype(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItemType, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_traititemtype(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = self.fold_chain_generics(c, t.generics)?;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        t.default = match t.default {
            Some(o) => Some((o.0, self.fold_chain_type(c, o.1)?)),
            None => None,
        };
        t.semi_token = t.semi_token;
        for mut i in self.after() {
            t = i.chain_traititemtype(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Type,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_type(c, t)?;
        }
        t = match t {
            syn::Type::Array(tmp) => self.fold_chain_type_array(c, (tmp))?,
            syn::Type::BareFn(tmp) => self.fold_chain_type_barefn(c, (tmp))?,
            syn::Type::Group(tmp) => self.fold_chain_type_group(c, (tmp))?,
            syn::Type::ImplTrait(tmp) => self.fold_chain_type_impltrait(c, (tmp))?,
            syn::Type::Infer(tmp) => self.fold_chain_type_infer(c, (tmp))?,
            syn::Type::Macro(tmp) => self.fold_chain_type_macro(c, (tmp))?,
            syn::Type::Never(tmp) => self.fold_chain_type_never(c, (tmp))?,
            syn::Type::Paren(tmp) => self.fold_chain_type_paren(c, (tmp))?,
            syn::Type::Path(tmp) => self.fold_chain_type_path(c, (tmp))?,
            syn::Type::Ptr(tmp) => self.fold_chain_type_ptr(c, (tmp))?,
            syn::Type::Reference(tmp) => self.fold_chain_type_reference(c, (tmp))?,
            syn::Type::Slice(tmp) => self.fold_chain_type_slice(c, (tmp))?,
            syn::Type::TraitObject(tmp) => self.fold_chain_type_traitobject(c, (tmp))?,
            syn::Type::Tuple(tmp) => self.fold_chain_type_tuple(c, (tmp))?,
            syn::Type::Verbatim(tmp) => self.fold_chain_type_verbatim(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_type(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_type_array(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeArray,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_array(c, t)? {
                syn::Type::Array(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typearray(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_array(c, t)? {
                syn::Type::Array(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Array(t))
    }
    fn fold_chain_type_barefn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeBareFn,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_barefn(c, t)? {
                syn::Type::BareFn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typebarefn(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_barefn(c, t)? {
                syn::Type::BareFn(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::BareFn(t))
    }
    fn fold_chain_type_group(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeGroup,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_group(c, t)? {
                syn::Type::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typegroup(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_group(c, t)? {
                syn::Type::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Group(t))
    }
    fn fold_chain_type_impltrait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeImplTrait,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_impltrait(c, t)? {
                syn::Type::ImplTrait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeimpltrait(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_impltrait(c, t)? {
                syn::Type::ImplTrait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::ImplTrait(t))
    }
    fn fold_chain_type_infer(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeInfer,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_infer(c, t)? {
                syn::Type::Infer(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeinfer(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_infer(c, t)? {
                syn::Type::Infer(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Infer(t))
    }
    fn fold_chain_type_macro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeMacro,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_macro(c, t)? {
                syn::Type::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typemacro(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_macro(c, t)? {
                syn::Type::Macro(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Macro(t))
    }
    fn fold_chain_type_never(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeNever,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_never(c, t)? {
                syn::Type::Never(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typenever(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_never(c, t)? {
                syn::Type::Never(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Never(t))
    }
    fn fold_chain_type_paren(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeParen,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_paren(c, t)? {
                syn::Type::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeparen(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_paren(c, t)? {
                syn::Type::Paren(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Paren(t))
    }
    fn fold_chain_type_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypePath,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_path(c, t)? {
                syn::Type::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typepath(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_path(c, t)? {
                syn::Type::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Path(t))
    }
    fn fold_chain_type_ptr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypePtr,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_ptr(c, t)? {
                syn::Type::Ptr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeptr(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_ptr(c, t)? {
                syn::Type::Ptr(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Ptr(t))
    }
    fn fold_chain_type_reference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeReference,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_reference(c, t)? {
                syn::Type::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typereference(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_reference(c, t)? {
                syn::Type::Reference(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Reference(t))
    }
    fn fold_chain_type_slice(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeSlice,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_slice(c, t)? {
                syn::Type::Slice(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typeslice(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_slice(c, t)? {
                syn::Type::Slice(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Slice(t))
    }
    fn fold_chain_type_traitobject(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeTraitObject,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_traitobject(c, t)? {
                syn::Type::TraitObject(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typetraitobject(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_traitobject(c, t)? {
                syn::Type::TraitObject(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::TraitObject(t))
    }
    fn fold_chain_type_tuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeTuple,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_tuple(c, t)? {
                syn::Type::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_typetuple(c, t)?;
        for mut i in self.after() {
            t = match i.chain_type_tuple(c, t)? {
                syn::Type::Tuple(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Tuple(t))
    }
    fn fold_chain_type_verbatim(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: proc_macro2::TokenStream,
    ) -> Result<syn::Type, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_type_verbatim(c, t)? {
                syn::Type::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_type_verbatim(c, t)? {
                syn::Type::Verbatim(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Type::Verbatim(t))
    }
    fn fold_chain_typearray(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeArray,
    ) -> Result<syn::TypeArray, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typearray(c, t)?;
        }
        t.bracket_token = t.bracket_token;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        t.semi_token = t.semi_token;
        t.len = self.fold_chain_expr(c, t.len)?;
        for mut i in self.after() {
            t = i.chain_typearray(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typebarefn(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeBareFn,
    ) -> Result<syn::TypeBareFn, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typebarefn(c, t)?;
        }
        t.lifetimes = match t.lifetimes {
            Some(o) => Some(self.fold_chain_boundlifetimes(c, o)?),
            None => None,
        };
        t.unsafety = t.unsafety;
        t.abi = match t.abi {
            Some(o) => Some(self.fold_chain_abi(c, o)?),
            None => None,
        };
        t.fn_token = t.fn_token;
        t.paren_token = t.paren_token;
        t.inputs = t.inputs;
        t.variadic = match t.variadic {
            Some(o) => Some(self.fold_chain_variadic(c, o)?),
            None => None,
        };
        t.output = self.fold_chain_returntype(c, t.output)?;
        for mut i in self.after() {
            t = i.chain_typebarefn(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typegroup(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeGroup,
    ) -> Result<syn::TypeGroup, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typegroup(c, t)?;
        }
        t.group_token = t.group_token;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        for mut i in self.after() {
            t = i.chain_typegroup(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeimpltrait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeImplTrait,
    ) -> Result<syn::TypeImplTrait, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeimpltrait(c, t)?;
        }
        t.impl_token = t.impl_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_typeimpltrait(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeinfer(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeInfer,
    ) -> Result<syn::TypeInfer, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeinfer(c, t)?;
        }
        t.underscore_token = t.underscore_token;
        for mut i in self.after() {
            t = i.chain_typeinfer(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typemacro(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeMacro,
    ) -> Result<syn::TypeMacro, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typemacro(c, t)?;
        }
        t.mac = self.fold_chain_macro(c, t.mac)?;
        for mut i in self.after() {
            t = i.chain_typemacro(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typenever(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeNever,
    ) -> Result<syn::TypeNever, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typenever(c, t)?;
        }
        t.bang_token = t.bang_token;
        for mut i in self.after() {
            t = i.chain_typenever(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeparam(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeParam,
    ) -> Result<syn::TypeParam, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeparam(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.bounds = t.bounds;
        t.eq_token = t.eq_token;
        t.default = match t.default {
            Some(o) => Some(self.fold_chain_type(c, o)?),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_typeparam(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeparambound(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeParamBound,
    ) -> Result<syn::TypeParamBound, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeparambound(c, t)?;
        }
        t = match t {
            syn::TypeParamBound::Trait(tmp) => self.fold_chain_typeparambound_trait(c, (tmp))?,
            syn::TypeParamBound::Lifetime(tmp) => {
                self.fold_chain_typeparambound_lifetime(c, (tmp))?
            }
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_typeparambound(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeparambound_trait(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TraitBound,
    ) -> Result<syn::TypeParamBound, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_typeparambound_trait(c, t)? {
                syn::TypeParamBound::Trait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_traitbound(c, t)?;
        for mut i in self.after() {
            t = match i.chain_typeparambound_trait(c, t)? {
                syn::TypeParamBound::Trait(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TypeParamBound::Trait(t))
    }
    fn fold_chain_typeparambound_lifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Lifetime,
    ) -> Result<syn::TypeParamBound, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_typeparambound_lifetime(c, t)? {
                syn::TypeParamBound::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_lifetime(c, t)?;
        for mut i in self.after() {
            t = match i.chain_typeparambound_lifetime(c, t)? {
                syn::TypeParamBound::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::TypeParamBound::Lifetime(t))
    }
    fn fold_chain_typeparen(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeParen,
    ) -> Result<syn::TypeParen, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeparen(c, t)?;
        }
        t.paren_token = t.paren_token;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        for mut i in self.after() {
            t = i.chain_typeparen(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typepath(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypePath,
    ) -> Result<syn::TypePath, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typepath(c, t)?;
        }
        t.qself = match t.qself {
            Some(o) => Some(self.fold_chain_qself(c, o)?),
            None => None,
        };
        t.path = self.fold_chain_path(c, t.path)?;
        for mut i in self.after() {
            t = i.chain_typepath(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeptr(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypePtr,
    ) -> Result<syn::TypePtr, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeptr(c, t)?;
        }
        t.star_token = t.star_token;
        t.const_token = t.const_token;
        t.mutability = t.mutability;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        for mut i in self.after() {
            t = i.chain_typeptr(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typereference(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeReference,
    ) -> Result<syn::TypeReference, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typereference(c, t)?;
        }
        t.and_token = t.and_token;
        t.lifetime = match t.lifetime {
            Some(o) => Some(self.fold_chain_lifetime(c, o)?),
            None => None,
        };
        t.mutability = t.mutability;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        for mut i in self.after() {
            t = i.chain_typereference(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typeslice(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeSlice,
    ) -> Result<syn::TypeSlice, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typeslice(c, t)?;
        }
        t.bracket_token = t.bracket_token;
        t.elem = Box::new(self.fold_chain_type(c, *t.elem)?);
        for mut i in self.after() {
            t = i.chain_typeslice(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typetraitobject(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeTraitObject,
    ) -> Result<syn::TypeTraitObject, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typetraitobject(c, t)?;
        }
        t.dyn_token = t.dyn_token;
        t.bounds = t.bounds;
        for mut i in self.after() {
            t = i.chain_typetraitobject(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_typetuple(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::TypeTuple,
    ) -> Result<syn::TypeTuple, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_typetuple(c, t)?;
        }
        t.paren_token = t.paren_token;
        t.elems = t.elems;
        for mut i in self.after() {
            t = i.chain_typetuple(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_unop(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UnOp,
    ) -> Result<syn::UnOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_unop(c, t)?;
        }
        t = match t {
            syn::UnOp::Deref(tmp) => self.fold_chain_unop_deref(c, (tmp))?,
            syn::UnOp::Not(tmp) => self.fold_chain_unop_not(c, (tmp))?,
            syn::UnOp::Neg(tmp) => self.fold_chain_unop_neg(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_unop(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_unop_deref(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Star,
    ) -> Result<syn::UnOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_unop_deref(c, t)? {
                syn::UnOp::Deref(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_unop_deref(c, t)? {
                syn::UnOp::Deref(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UnOp::Deref(t))
    }
    fn fold_chain_unop_not(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Bang,
    ) -> Result<syn::UnOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_unop_not(c, t)? {
                syn::UnOp::Not(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_unop_not(c, t)? {
                syn::UnOp::Not(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UnOp::Not(t))
    }
    fn fold_chain_unop_neg(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::token::Sub,
    ) -> Result<syn::UnOp, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_unop_neg(c, t)? {
                syn::UnOp::Neg(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = t;
        for mut i in self.after() {
            t = match i.chain_unop_neg(c, t)? {
                syn::UnOp::Neg(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UnOp::Neg(t))
    }
    fn fold_chain_useglob(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseGlob,
    ) -> Result<syn::UseGlob, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_useglob(c, t)?;
        }
        t.star_token = t.star_token;
        for mut i in self.after() {
            t = i.chain_useglob(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_usegroup(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseGroup,
    ) -> Result<syn::UseGroup, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_usegroup(c, t)?;
        }
        t.brace_token = t.brace_token;
        t.items = t.items;
        for mut i in self.after() {
            t = i.chain_usegroup(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_usename(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseName,
    ) -> Result<syn::UseName, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_usename(c, t)?;
        }
        t.ident = t.ident;
        for mut i in self.after() {
            t = i.chain_usename(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_usepath(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UsePath,
    ) -> Result<syn::UsePath, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_usepath(c, t)?;
        }
        t.ident = t.ident;
        t.colon2_token = t.colon2_token;
        t.tree = Box::new(self.fold_chain_usetree(c, *t.tree)?);
        for mut i in self.after() {
            t = i.chain_usepath(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_userename(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseRename,
    ) -> Result<syn::UseRename, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_userename(c, t)?;
        }
        t.ident = t.ident;
        t.as_token = t.as_token;
        t.rename = t.rename;
        for mut i in self.after() {
            t = i.chain_userename(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_usetree(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseTree,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_usetree(c, t)?;
        }
        t = match t {
            syn::UseTree::Path(tmp) => self.fold_chain_usetree_path(c, (tmp))?,
            syn::UseTree::Name(tmp) => self.fold_chain_usetree_name(c, (tmp))?,
            syn::UseTree::Rename(tmp) => self.fold_chain_usetree_rename(c, (tmp))?,
            syn::UseTree::Glob(tmp) => self.fold_chain_usetree_glob(c, (tmp))?,
            syn::UseTree::Group(tmp) => self.fold_chain_usetree_group(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_usetree(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_usetree_path(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UsePath,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_usetree_path(c, t)? {
                syn::UseTree::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_usepath(c, t)?;
        for mut i in self.after() {
            t = match i.chain_usetree_path(c, t)? {
                syn::UseTree::Path(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UseTree::Path(t))
    }
    fn fold_chain_usetree_name(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseName,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_usetree_name(c, t)? {
                syn::UseTree::Name(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_usename(c, t)?;
        for mut i in self.after() {
            t = match i.chain_usetree_name(c, t)? {
                syn::UseTree::Name(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UseTree::Name(t))
    }
    fn fold_chain_usetree_rename(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseRename,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_usetree_rename(c, t)? {
                syn::UseTree::Rename(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_userename(c, t)?;
        for mut i in self.after() {
            t = match i.chain_usetree_rename(c, t)? {
                syn::UseTree::Rename(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UseTree::Rename(t))
    }
    fn fold_chain_usetree_glob(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseGlob,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_usetree_glob(c, t)? {
                syn::UseTree::Glob(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_useglob(c, t)?;
        for mut i in self.after() {
            t = match i.chain_usetree_glob(c, t)? {
                syn::UseTree::Glob(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UseTree::Glob(t))
    }
    fn fold_chain_usetree_group(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::UseGroup,
    ) -> Result<syn::UseTree, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_usetree_group(c, t)? {
                syn::UseTree::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_usegroup(c, t)?;
        for mut i in self.after() {
            t = match i.chain_usetree_group(c, t)? {
                syn::UseTree::Group(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::UseTree::Group(t))
    }
    fn fold_chain_variadic(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Variadic,
    ) -> Result<syn::Variadic, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_variadic(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.dots = t.dots;
        for mut i in self.after() {
            t = i.chain_variadic(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_variant(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Variant,
    ) -> Result<syn::Variant, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_variant(c, t)?;
        }
        t.attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(self.fold_chain_attribute(c, v)?);
            }
            tmp
        };
        t.ident = t.ident;
        t.fields = self.fold_chain_fields(c, t.fields)?;
        t.discriminant = match t.discriminant {
            Some(o) => Some((o.0, self.fold_chain_expr(c, o.1)?)),
            None => None,
        };
        for mut i in self.after() {
            t = i.chain_variant(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_viscrate(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisCrate,
    ) -> Result<syn::VisCrate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_viscrate(c, t)?;
        }
        t.crate_token = t.crate_token;
        for mut i in self.after() {
            t = i.chain_viscrate(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_vispublic(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisPublic,
    ) -> Result<syn::VisPublic, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_vispublic(c, t)?;
        }
        t.pub_token = t.pub_token;
        for mut i in self.after() {
            t = i.chain_vispublic(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_visrestricted(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisRestricted,
    ) -> Result<syn::VisRestricted, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_visrestricted(c, t)?;
        }
        t.pub_token = t.pub_token;
        t.paren_token = t.paren_token;
        t.in_token = t.in_token;
        t.path = Box::new(self.fold_chain_path(c, *t.path)?);
        for mut i in self.after() {
            t = i.chain_visrestricted(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_visibility(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::Visibility,
    ) -> Result<syn::Visibility, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_visibility(c, t)?;
        }
        t = match t {
            syn::Visibility::Public(tmp) => self.fold_chain_visibility_public(c, (tmp))?,
            syn::Visibility::Crate(tmp) => self.fold_chain_visibility_crate(c, (tmp))?,
            syn::Visibility::Restricted(tmp) => self.fold_chain_visibility_restricted(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_visibility(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_visibility_public(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisPublic,
    ) -> Result<syn::Visibility, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_visibility_public(c, t)? {
                syn::Visibility::Public(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_vispublic(c, t)?;
        for mut i in self.after() {
            t = match i.chain_visibility_public(c, t)? {
                syn::Visibility::Public(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Visibility::Public(t))
    }
    fn fold_chain_visibility_crate(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisCrate,
    ) -> Result<syn::Visibility, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_visibility_crate(c, t)? {
                syn::Visibility::Crate(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_viscrate(c, t)?;
        for mut i in self.after() {
            t = match i.chain_visibility_crate(c, t)? {
                syn::Visibility::Crate(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Visibility::Crate(t))
    }
    fn fold_chain_visibility_restricted(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::VisRestricted,
    ) -> Result<syn::Visibility, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_visibility_restricted(c, t)? {
                syn::Visibility::Restricted(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_visrestricted(c, t)?;
        for mut i in self.after() {
            t = match i.chain_visibility_restricted(c, t)? {
                syn::Visibility::Restricted(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::Visibility::Restricted(t))
    }
    fn fold_chain_whereclause(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::WhereClause,
    ) -> Result<syn::WhereClause, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_whereclause(c, t)?;
        }
        t.where_token = t.where_token;
        t.predicates = t.predicates;
        for mut i in self.after() {
            t = i.chain_whereclause(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_wherepredicate(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::WherePredicate,
    ) -> Result<syn::WherePredicate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = i.chain_wherepredicate(c, t)?;
        }
        t = match t {
            syn::WherePredicate::Type(tmp) => self.fold_chain_wherepredicate_type(c, (tmp))?,
            syn::WherePredicate::Lifetime(tmp) => {
                self.fold_chain_wherepredicate_lifetime(c, (tmp))?
            }
            syn::WherePredicate::Eq(tmp) => self.fold_chain_wherepredicate_eq(c, (tmp))?,
            _ => t,
        };
        for mut i in self.after() {
            t = i.chain_wherepredicate(c, t)?;
        }
        Ok(t)
    }
    fn fold_chain_wherepredicate_type(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateType,
    ) -> Result<syn::WherePredicate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_wherepredicate_type(c, t)? {
                syn::WherePredicate::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_predicatetype(c, t)?;
        for mut i in self.after() {
            t = match i.chain_wherepredicate_type(c, t)? {
                syn::WherePredicate::Type(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::WherePredicate::Type(t))
    }
    fn fold_chain_wherepredicate_lifetime(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateLifetime,
    ) -> Result<syn::WherePredicate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_wherepredicate_lifetime(c, t)? {
                syn::WherePredicate::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_predicatelifetime(c, t)?;
        for mut i in self.after() {
            t = match i.chain_wherepredicate_lifetime(c, t)? {
                syn::WherePredicate::Lifetime(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::WherePredicate::Lifetime(t))
    }
    fn fold_chain_wherepredicate_eq(
        &mut self,
        c: &mut <Self as ChainIter>::Input,
        t: syn::PredicateEq,
    ) -> Result<syn::WherePredicate, <Self as ChainIter>::Err> {
        let mut t = t;
        for mut i in self.before() {
            t = match i.chain_wherepredicate_eq(c, t)? {
                syn::WherePredicate::Eq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        t = self.fold_chain_predicateeq(c, t)?;
        for mut i in self.after() {
            t = match i.chain_wherepredicate_eq(c, t)? {
                syn::WherePredicate::Eq(tmp) => (tmp),
                tmp => return Ok(tmp),
            };
        }
        Ok(syn::WherePredicate::Eq(t))
    }
}
