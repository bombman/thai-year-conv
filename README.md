# Thai Year Converter

A Rust library for converting between Christian (AD) and Buddhist (BE) years.

## Usage
Documentation = "https://docs.rs/thai-year-conv"

Add this to your `Cargo.toml`:

```toml
[dependencies]
thai-year-conv = "0.1.0"
```

```Rust
use thai_year_conv::{Year, YearConversion};
fn main() {
    let buddhist_year = Year::new(2024).to_buddhist_year();
    let christian_year = Year::new(2557).to_christian_year();
    println!("buddhist_year {}", buddhist_year);
    println!("christian_year {}", christian_year);
}

```