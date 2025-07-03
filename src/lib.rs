use bincode::{Decode, Encode};
use phf::phf_map;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};

static UNICODE_TO_PINYIN: OnceLock<HashMap<u32, Vec<String>>> = OnceLock::new();

static DIACRITIC_TO_LETTER: phf::Map<char, &'static str> = phf_map! {
    'ā' => "a1",
    'á' => "a2",
    'ǎ' => "a3",
    'à' => "a4",
    'ē' => "e1",
    'é' => "e2",
    'ě' => "e3",
    'è' => "e4",
    'ī' => "i1",
    'í' => "i2",
    'ǐ' => "i3",
    'ì' => "i4",
    'ō' => "o1",
    'ó' => "o2",
    'ǒ' => "o3",
    'ò' => "o4",
    'ǖ' => "v1",
    'ǘ' => "v2",
    'ǚ' => "v3",
    'ǜ' => "v4",
    'ū' => "u1",
    'ú' => "u2",
    'ǔ' => "u3",
    'ù' => "u4",
    'ḿ' => "m2",
    'ń' => "n2",
    'ň' => "n3",
    'ǹ' => "n4"
};

static LETTER_TO_DIACRITIC: phf::Map<&'static str, char> = phf_map! {
    "a1" => 'ā',
    "a2" => 'á',
    "a3" => 'ǎ',
    "a4" => 'à',
    "e1" => 'ē',
    "e2" => 'é',
    "e3" => 'ě',
    "e4" => 'è',
    "i1" => 'ī',
    "i2" => 'í',
    "i3" => 'ǐ',
    "i4" => 'ì',
    "o1" => 'ō',
    "o2" => 'ó',
    "o3" => 'ǒ',
    "o4" => 'ò',
    "ü1" => 'ǖ',
    "ü2" => 'ǘ',
    "ü3" => 'ǚ',
    "ü4" => 'ǜ',
    "yu1" => 'ǖ',
    "yu2" => 'ǘ',
    "yu3" => 'ǚ',
    "yu4" => 'ǜ',
    "v1" => 'ǖ',
    "v2" => 'ǘ',
    "v3" => 'ǚ',
    "v4" => 'ǜ',
    "u1" => 'ū',
    "u2" => 'ú',
    "u3" => 'ǔ',
    "u4" => 'ù',
    "m2" => 'ḿ',
    "n2" => 'ń',
    "n3" => 'ň',
    "n4" => 'ǹ'
};

// The `Encode` and `Decode` traits are for bincode's native, high-performance serialization.
// The `Serialize` and `Deserialize` traits are for serde-based formats like JSON.
// We keep the serde traits for two reasons:
// 1. Future-proofing: It allows easy serialization to JSON for debugging or other purposes.
// 2. Ecosystem compatibility: It's standard practice for data structures to be serde-compatible.
#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
pub struct UnicodeMapping {
    pub mappings: HashMap<u32, Vec<String>>,
}

/// Deserializes a `UnicodeMapping` from a byte slice.
/// This is the primary function for loading mapping data.
pub fn load_from_bytes(bytes: &[u8]) -> Result<UnicodeMapping, Box<dyn std::error::Error>> {
    let (decoded, _len): (UnicodeMapping, usize) =
        bincode::decode_from_slice(bytes, bincode::config::standard())?;
    Ok(decoded)
}

/// Loads the default, embedded unicode-to-pinyin mapping.
/// This function is only available when the `default-data` feature is enabled.
#[cfg(feature = "default-data")]
pub fn load_default() -> Result<UnicodeMapping, Box<dyn std::error::Error>> {
    let bytes = include_bytes!("../bincode/unicode-to-pinyin.bin");
    load_from_bytes(bytes)
}

pub fn init_map(_bytes: Option<&[u8]>) -> Result<(), Box<dyn std::error::Error>> {
    let unicode_mapping: UnicodeMapping;

    #[cfg(feature = "default-data")]
    {
        unicode_mapping = load_default()?;
    }

    #[cfg(not(feature = "default-data"))]
    {
        let bytes = _bytes.ok_or("bytes is required but None was provided")?;
        unicode_mapping = load_from_bytes(bytes)?;
    }

    UNICODE_TO_PINYIN
        .set(unicode_mapping.mappings)
        .map_err(|_| "failed to set mappings in OneLock")?;

    Ok(())
}

#[derive(Debug)]
pub struct LookupResult<K> {
    pub map: HashMap<K, Option<Vec<String>>>, 
    pub vec: Vec<Option<Vec<String>>>,
}

pub fn lookup_unicodes_map(keys: &[u32]) -> Result<HashMap<u32, Option<Vec<String>>>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;
    Ok(keys
        .iter()
        .map(|&key| (key, map.get(&key).cloned()))
        .collect())
}

pub fn lookup_unicodes_vec(keys: &[u32]) -> Result<Vec<Option<Vec<String>>>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;
    Ok(keys.iter().map(|&key| map.get(&key).cloned()).collect())
}

pub fn lookup_unicodes(keys: &[u32]) -> Result<LookupResult<u32>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;
    let mut result_map = HashMap::new();
    let mut values = Vec::new();

    for &key in keys {
        let value = map.get(&key).cloned();
        result_map.insert(key, value.clone());
        values.push(value);
    }

    Ok(LookupResult {
        map: result_map,
        vec: values,
    })
}

pub fn lookup_chars_map(keys: &[char]) -> Result<HashMap<char, Option<Vec<String>>>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;
    Ok(keys
        .iter()
        .map(|&key| (key, map.get(&(key as u32)).cloned()))
        .collect())
}

pub fn lookup_chars_vec(keys: &[char]) -> Result<Vec<Option<Vec<String>>>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;
    Ok(keys
        .iter()
        .map(|&key| map.get(&(key as u32)).cloned())
        .collect())
}

pub fn lookup_chars(keys: &[char]) -> Result<LookupResult<char>, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;

    let mut result_map = HashMap::new();
    let mut values = Vec::new();

    for &key in keys {
        let value = map.get(&(key as u32)).cloned();
        result_map.insert(key, value.clone());
        values.push(value);
    }

    Ok(LookupResult {
        map: result_map,
        vec: values,
    })
}

pub fn lookup_chars_map_for_str(s: &str) -> Result<HashMap<char, Option<Vec<String>>>, String> {
    let keys: Vec<char> = s.chars().collect();
    lookup_chars_map(&keys)
}

pub fn lookup_chars_vec_for_str(s: &str) -> Result<Vec<Option<Vec<String>>>, String> {
    let keys: Vec<char> = s.chars().collect();
    lookup_chars_vec(&keys)
}

pub fn lookup_chars_for_str(s: &str) -> Result<LookupResult<char>, String> {
    let keys: Vec<char> = s.chars().collect();
    lookup_chars(&keys)
}

pub fn to_pinyin_string(text: &str, separator: &str) -> Result<String, String> {
    let map = UNICODE_TO_PINYIN
        .get()
        .ok_or("UNICODE_TO_PINYIN not initialized. Call init_map first.")?;

    let pinyins: Vec<String> = text
        .chars()
        .map(|c| {
            map.get(&(c as u32))
                .and_then(|p_vec| p_vec.get(0))
                .map_or(c.to_string(), |p| p.clone())
        })
        .collect();

    Ok(pinyins.join(separator))
}

pub fn diacritic_to_tone_plus_number(pinyins: &[&str]) -> Vec<String> {
    pinyins
        .iter()
        .map(|&pinyin| replace_diacritic(pinyin))
        .collect()
}

fn replace_diacritic(pinyin: &str) -> String {
    // Pre-allocate string with estimated capacity
    let mut result = String::with_capacity(pinyin.len() * 2);
    for c in pinyin.chars() {
        if let Some(replacement) = DIACRITIC_TO_LETTER.get(&c) {
            result.push_str(replacement);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn tone_plus_number_to_diacritic(pinyins: &[&str]) -> Vec<String> {
    pinyins
        .iter()
        .map(|&pinyin| replace_numbered_pinyin(pinyin))
        .collect()
}

fn replace_numbered_pinyin(pinyin: &str) -> String {
    let mut result = String::with_capacity(pinyin.len());
    let mut chars: Vec<char> = pinyin.chars().collect();

    while let Some(current_char) = chars.pop() {
        if current_char.is_ascii_digit() {
            // Check for 2-char pinyin (e.g., "yu1")
            if chars.len() >= 2 {
                let prev_char = chars[chars.len() - 1];
                let prev_prev_char = chars[chars.len() - 2];
                let key = format!("{}{}{}", prev_prev_char, prev_char, current_char);
                if let Some(&diacritic) = LETTER_TO_DIACRITIC.get(key.as_str()) {
                    result.insert(0, diacritic);
                    chars.pop(); // consume second letter
                    chars.pop(); // consume first letter
                    continue;
                }
            }

            // Check for 1-char pinyin (e.g., "a1")
            if let Some(&prev_char) = chars.last() {
                let key = format!("{}{}", prev_char, current_char);
                if let Some(&diacritic) = LETTER_TO_DIACRITIC.get(key.as_str()) {
                    result.insert(0, diacritic);
                    chars.pop(); // consume the letter
                    continue;
                }
            }
        }
        // If not a convertible pinyin, just add the char
        result.insert(0, current_char);
    }
    result
}

#[cfg(feature = "prepare-data")]
pub fn save_to_vec(
    pinyin_map: HashMap<u32, Vec<String>>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mappings: HashMap<u32, Vec<String>> = pinyin_map
        .into_iter()
        .filter_map(|(k, v)| {
            if k <= 0x10FFFF {
                Some((k, v))
            } else {
                eprintln!("Codepoint {} out of Unicode range", k);
                None
            }
        })
        .collect();

    let unicode_mapping = UnicodeMapping { mappings };

    let encoded = bincode::encode_to_vec(&unicode_mapping, bincode::config::standard())?;
    Ok(encoded)
}

#[cfg(feature = "prepare-data")]
pub fn load_pinyin_map(
    pinyin_data_path: &str,
) -> Result<HashMap<u32, Vec<std::string::String>>, Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    let path = Path::new(pinyin_data_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut pinyin_map: HashMap<u32, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            if let Ok(codepoint) = u32::from_str_radix(parts[0], 16) {
                let pinyin = parts[1].to_string();
                let p: Vec<String> = pinyin.split_whitespace().map(|s| s.to_string()).collect();
                pinyin_map.insert(codepoint, p);
            }
        }
    }

    Ok(pinyin_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_diacritic_conversion_tests(test_fn: &dyn Fn(&[&str]) -> Vec<String>) {
        // Test case 1: Basic valid pinyin
        let input1 = vec!["ni3", "ha3o"];
        let expected1: Vec<&'static str> = vec!["nǐ", "hǎo"];
        assert_eq!(test_fn(&input1), expected1);

        // Test case 2: Pinyin with all tones and 'v' for 'ü'
        let input2 = vec!["ma1", "ma2", "ma3", "ma4", "nv3", "lv4"];
        let expected2 = vec!["mā", "má", "mǎ", "mà", "nǚ", "lǜ"];
        assert_eq!(test_fn(&input2), expected2);

        // Test case 3: Invalid pinyin (number not at the end, or not a valid tone)
        // The function should pass them through unchanged.
        let input3 = vec!["pin1yin1", "test5", "a", "b0"];
        let expected3 = vec!["pin1yin1", "test5", "a", "b0"];
        assert_eq!(test_fn(&input3), expected3);

        // Test case 4: Mix of valid and invalid pinyin
        let input4 = vec!["wo3", "shi4", "xue2sheng", "ni2hao3"];
        let expected4 = vec!["wǒ", "shì", "xuésheng", "níhaǒ"];
        assert_eq!(test_fn(&input4), expected4);

        // Test case 5: Empty input
        let input5: Vec<&str> = vec![];
        let expected5: Vec<String> = vec![];
        assert_eq!(test_fn(&input5), expected5);

        // Test case 6: Empty strings in the input
        let input6 = vec!["", "hao3", ""];
        let expected6 = vec!["", "haǒ", ""];
        assert_eq!(test_fn(&input6), expected6);
        
        // Test case 7: Pinyin where the letter before the number does not form a valid diacritic
        let input7 = vec!["x1", "z4"];
        let expected7 = vec!["x1", "z4"];
        assert_eq!(test_fn(&input7), expected7);

        // Test case 8: Test "yu" mapping
        let input8 = vec!["yu1", "yu2", "yu3", "yu4", "abcyu4o", "abcu4o"];
        let expected8 = vec!["ǖ", "ǘ", "ǚ", "ǜ", "abcǜo", "abcùo"];
        assert_eq!(test_fn(&input8), expected8);
    }

    #[test]
    fn test_tone_plus_number_to_diacritic() {
        run_diacritic_conversion_tests(&tone_plus_number_to_diacritic);
    }

    #[test]
    fn test_to_pinyin_string() {
        init_map(None).unwrap();
        // Test case 1: Normal sentence
        let input1 = "你好世界";
        let expected1 = "nǐ hǎo shì jiè";
        assert_eq!(to_pinyin_string(input1, " ").unwrap(), expected1);

        // Test case 2: Sentence with non-Chinese characters
        let input2 = "Hello 你好, world";
        let expected2 = "H e l l o   nǐ hǎo ,   w o r l d";
        assert_eq!(to_pinyin_string(input2, " ").unwrap(), expected2);

        // Test case 3: Empty string
        let input3 = "";
        let expected3 = "";
        assert_eq!(to_pinyin_string(input3, " ").unwrap(), expected3);

        // Test case 4: Different separator
        let input4 = "你好";
        let expected4 = "nǐ-hǎo";
        assert_eq!(to_pinyin_string(input4, "-").unwrap(), expected4);
    }
}
