fn main() {
    println!("Hello, world!");
    greet("mk-tools");
    demonstrate_features();
}

fn greet(name: &str) {
    println!("Welcome to {}!", name);
}

fn demonstrate_features() {
    println!("\nFeatures:");
    println!("- Code block synchronization");
    println!("- Table of contents generation");
    println!("- CI/CD integration");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        greet("test");
    }
}
