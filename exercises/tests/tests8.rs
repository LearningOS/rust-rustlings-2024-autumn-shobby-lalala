// tests8.rs

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;
        panic!("no cfg set");
    }
}
