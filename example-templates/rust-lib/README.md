# {{project_name}}

{{description}}

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
{{project_name}} = "0.1"
```

## Usage

```rust
use {{project_name}}::{add, subtract};

fn main() {
    let sum = add(2, 3);
    let diff = subtract(5, 3);
    println!("Sum: {}, Diff: {}", sum, diff);
}
```

## Development

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code
cargo clippy

# Format code
cargo fmt

# Build docs
cargo doc --open
```

## License

MIT
