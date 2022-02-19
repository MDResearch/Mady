pub struct AstTree {
    func: Func,
}

pub struct Func {
    name: String,
}

pub struct Arg {
    name: String,
    ty: Ty,
}

pub enum Ty {
    I32,
    I64,
    F32,
    F64,
    Mat,
    Vec,
    Fn,
}
