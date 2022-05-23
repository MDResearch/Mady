use mady::prelude::*;

#[grad(f64)]
fn block_mul(a: f64) -> f64 {
    let mut b = a;
    {
        b = b * a;
        {
            b = b * a;
            {
                b = b * a;
            }
        }
    }
    b
}

#[grad(f64, f64)]
fn block_return(a: f64, b: f64) -> f64 {
    let c = (a + b) * a;
    {
        {
            {
                {
                    {
                        c
                    }
                }
            }
        }
    }
}

#[grad(f64, f64)]
fn separate_multiple(a: f64, b: f64) -> f64 {
    let c = (a + b) * a;
    let d;
    let e;
    let f;
    let g;
    {
        {
            {
                {
                    d = c;
                }
                e = d;
            }
            f = e;
        }
        g = f;
    }
    g
}

fn main() {
    assert_eq!((16., 32.), block_mul(2.));
    assert_eq!((48., (14., 6.)), block_return(6., 2.));
    assert_eq!((48., (14., 6.)), separate_multiple(6., 2.));
}
