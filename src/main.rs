use mad::ad;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testmad() {
        aplusb(1f64, 2f64);
    }

    #[ad]
    fn aplusb(a: f64, b: f64) -> f64 {
        a * (a + b)
    }
}
