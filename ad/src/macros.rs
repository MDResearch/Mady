#[macro_export]
macro_rules! impl_ops_all {
    ($opstype:tt[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:$at,$b:$bt)->$r$code);
        impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:&$at,$b:$bt)->$r$code);
        impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:$at,$b:&$bt)->$r$code);
        impl_ops!($opstype[<$($ge),+> where $($wh)+]($a:&$at,$b:&$bt)->$r$code);
    };
}

#[macro_export]
macro_rules! impl_ops {
    (+[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        impl_trait!(Add,add,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (-[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        impl_trait!(Sub,sub,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (*[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        impl_trait!(Mul,mul,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };

    (/[<$($ge:tt),+> where $($wh:tt)+]($a:ident:$at:ty,$b:ident:$bt:ty)->$r:ty$code:block) => {
        impl_trait!(Div,div,[$($ge),+],[$($wh)+],$a,$at,$b,$bt,$r,$code);
    };
}

#[macro_export]
macro_rules! impl_trait {
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

