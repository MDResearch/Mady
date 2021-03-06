//! a internal library
//! most of macros define here

#[macro_export]
macro_rules! impl_ops_all {
    ($opstype:tt[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        $crate::impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:$at,$b:$bt)->$r$code);
        $crate::impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:&$at,$b:$bt)->$r$code);
        $crate::impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:$at,$b:&$bt)->$r$code);
        $crate::impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:&$at,$b:&$bt)->$r$code);
    };
}

#[macro_export]
macro_rules! impl_ops {
    (+[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        $crate::impl_ops_trait!(Add,add,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (-[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        $crate::impl_ops_trait!(Sub,sub,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (*[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        $crate::impl_ops_trait!(Mul,mul,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (/[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        $crate::impl_ops_trait!(Div,div,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };
}

#[macro_export]
macro_rules! impl_ops_trait {
    ($t:ident,$f:ident,[$($ge:tt),+],[$($wh:tt)+],$a:ident,$at:ty,$b:ident,$bt:ty,$r:ty,$code:block) => {
        impl<$($ge),+> $t<$bt> for $at where $($wh)+
        {
            type Output = $r;
            fn $f(self, $b: $bt) -> Self::Output {
                let $a = self;
                $code
            }
        }
    };
}

#[macro_export]
macro_rules! test_ops_all {
    ($cmp:tt,$ops:tt,$a:expr,$b:expr,$r:expr) => {
        test_ops!($cmp, $ops, $a, $b, $r);
        test_ops!($cmp, $ops, &$a, $b, $r);
        test_ops!($cmp, $ops, $a, &$b, $r);
        test_ops!($cmp, $ops, &$a, &$b, $r);
    };
}

#[macro_export]
macro_rules! test_ops {
    (=,$ops:tt,$a:expr,$b:expr,$r:expr) => {
        assert_eq!($r, $a $ops $b);
    };
}

#[macro_export]
macro_rules! ten {
    ($elem:expr; $n:expr) => (
        $crate::tensor::Tensor::new($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        $crate::tensor::Tensor::from(vec![$($x),+])
    );
}

#[macro_export]
macro_rules! mat {
    ($elem:expr; $r:expr, $c:expr) => (
        $crate::matrix::Matrix::new($elem, [$r, $c])
    );
    // $(,)? , maybe appear
    ($($x:expr),+ $(,)?;$l:expr) => (
        $crate::matrix::Matrix::from(($crate::ten![$($x),+], [$l, $l]))
    );
    ($($x:expr),+ $(,)?;$r:expr, $c:expr) => (
        $crate::matrix::Matrix::from(($crate::ten![$($x),+], [$r, $c]))
    );
}

#[macro_export]
macro_rules! impl_trait {
    ($mac:ident,$($ty:ident),+)=>{
        $(
            $mac![$ty];
        )+
    };


    // use default
    ($trait_name:ident,$($ty:ident),+ ) => {
        $(
            impl $trait_name for $ty{}
        )+
    };

    // use custom fn
    ($trait_name:ident,$fn:item,$($ty:ident),+ ) => {
        $(
            impl $trait_name for $ty{
                $fn
            }
        )+
    };

    // use custom fn with generic
    ($trait_name:ident,$($gen:ident)+,$fn:item,$($ty:ident)+ ) => {
        $(
            impl $trait_name<gen> for $ty{
                $fn
            }
        )+
    };

    ($trait_name:ident,$($fn:item)+,$($ty:ident),+ ) => {
        $(
            impl $trait_name for $ty{
                $fn
            }
        )+
    };


    (@ $trait_name:ident,$ty:ty,{$($items:item)+})=>{
        impl $trait_name for $ty {
            $($items)+
        }
    };
}
