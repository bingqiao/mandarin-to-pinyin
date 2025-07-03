# Mandarin to Pinyin

[![Crates.io](https://img.shields.io/crates/v/mandarin-to-pinyin.svg)](https://crates.io/crates/mandarin-to-pinyin)
[![Docs.rs](https://docs.rs/mandarin-to-pinyin/badge.svg)](https://docs.rs/mandarin-to-pinyin)

A lightweight, fast, and easy-to-use Rust crate for converting Mandarin Chinese characters to their corresponding Pinyin representation. It uses a pre-compiled Perfect Hash Function (PHF) map for instant lookups.

## Key Features

*   **Fast:** Blazing-fast lookups using `phf`.
*   **Simple API:** Get started with just a few lines of code.
*   **Self-Contained:** Includes a default Unicode-to-Pinyin mapping, no external files needed.
*   **Lightweight:** Option to exclude the default data to minimize your binary size if you provide your own.
*   **Customizable:** Includes a utility to build your own mapping file from a tab-separated source.
*   **Convenient String Conversion:** Easily convert entire Chinese sentences to Pinyin strings.

## Usage

1.  **Add to your project:**

    Add this line to your `Cargo.toml`:
    ```toml
    [dependencies]
    mandarin-to-pinyin = "0.0.1" # Replace with the latest version from crates.io
    ```

2.  **Use in your code:**

    The primary way to use the crate is to initialize the global map and use the lookup functions.

    ```rust
    use mandarin_to_pinyin::{init_map, to_pinyin_string};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Initialize the map (loads default data)
        init_map(None)?;

        // 2. Convert a Chinese sentence to Pinyin
        let chinese_sentence = "你好世界";
        let pinyin_sentence = to_pinyin_string(chinese_sentence, " ")?;
        println!("Pinyin for '{}': {}", chinese_sentence, pinyin_sentence);
        // Expected output: Pinyin for '你好世界': nǐ hǎo shì jiè

        // You can also use a different separator
        let pinyin_with_hyphens = to_pinyin_string("你好", "-")?;
        println!("Pinyin for '你好': {}", pinyin_with_hyphens);
        // Expected output: Pinyin for '你好': nǐ-hǎo

        Ok(())
    }
    ```

## Feature Flags

This crate uses feature flags to control its behavior and size.

#### `default-data` (enabled by default)

This feature embeds the `unicode-to-pinyin.bin` file directly into your library, allowing you to use `init_map(None)` for easy setup.

If you want to minimize binary size and provide your own data file at runtime, you can disable this feature.

**Disabling default features:**
```toml
[dependencies]
mandarin-to-pinyin = { version = "0.0.1", default-features = false }
```

When `default-data` is disabled, you must pass your own byte slice to `init_map()`:

```rust
use mandarin_to_pinyin::init_map;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read your custom .bin file
    let custom_data_bytes = fs::read("path/to/your/unicode-to-pinyin.bin")?;
    
    // Initialize the map with your custom data
    init_map(Some(&custom_data_bytes))?;
    
    // ... now you can use the lookup functions
    
    Ok(())
}
```

#### `prepare-data` (optional)

This feature is for developers who want to create their own `unicode-to-pinyin.bin` file from a source file. It enables a binary target that you can use as a command-line tool. The source file should be a text file where each line contains a Unicode code point and its Pinyin representation, separated by a tab.

Most users of this library will not need to enable this feature.

**To install the conversion tool:**
```bash
cargo install mandarin-to-pinyin --features prepare-data --no-default-features
```

**To run the tool:**
The tool will read `data/Mandarin.dat` and generate `bincode/unicode-to-pinyin.bin`.
```bash
mandarin-to-pinyin
```

## Data Source

The `data/Mandarin.dat` file used in this project is sourced from the [Lingua::Han::PinYin Perl module](https://github.com/fayland/perl-lingua-han/tree/master/Lingua-Han-PinYin/lib/Lingua/Han/PinYin) by Fayland Lam.

## API Reference

*   `fn init_map(bytes: Option<&[u8]>) -> Result<(), Box<dyn Error>>`
    Initializes the global Pinyin map. If `bytes` is `None`, it uses the default embedded data (requires the `default-data` feature). If `bytes` is `Some`, it uses the provided byte slice.

*   `fn to_pinyin_string(text: &str, separator: &str) -> Result<String, String>`
    Converts a Chinese string to a Pinyin string, using the first Pinyin pronunciation for each character and joining them with the specified separator.

*   `fn lookup_chars_for_str(chars: &str) -> Result<LookupResult<char>, String>`
    Looks up the Pinyin for a string slice and returns a space-separated string of Pinyin.

*   `fn lookup_unicodes(unicodes: &[u32]) -> Result<LookupResult<u32>, String>`
    Looks up the Pinyin for a slice of Unicode code points and returns a space-separated string of Pinyin.

*   `fn lookup_chars_map_for_str(chars: &str) -> Result<HashMap<char, Option<Vec<String>>>, String>`
    Looks up the Pinyin for a string slice and returns a `HashMap` of characters to their Pinyin.

*   `fn lookup_unicodes_map(unicodes: &[u32]) -> Result<HashMap<u32, Option<Vec<String>>>, String>`
    Looks up the Pinyin for a slice of Unicode code points and returns a `HashMap` of code points to their Pinyin.

*   `fn lookup_chars_vec_for_str(chars: &str) -> Result<Vec<Option<Vec<String>>>, String>`
    Looks up the Pinyin for a string slice and returns a `Vec` of Pinyin strings.

*   `fn lookup_unicodes_vec(unicodes: &[u32]) -> Result<Vec<Option<Vec<String>>>, String>`
    Looks up the Pinyin for a slice of Unicode code points and returns a `Vec` of Pinyin strings.

*   `fn diacritic_to_tone_plus_number(pinyins: &[&str]) -> Vec<String>`
    Converts Pinyin with diacritics to Pinyin with tone numbers (e.g., "xiāng" -> "xiang1").

*   `fn tone_plus_number_to_diacritic(pinyins: &[&str]) -> Vec<String>`
    Converts Pinyin with tone numbers to Pinyin with diacritics (e.g., "xiang1" -> "xiāng").