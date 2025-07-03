fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This entire block will be compiled based on the 'prepare-data' feature.
    // It ensures that main() always returns a Result.
    #[cfg(feature = "prepare-data")]
    {
        use mandarin_to_pinyin::{load_pinyin_map, save_to_vec};
        use std::fs;
        use std::io::Write;

        // No arguments: Generate default bincode file
        let input_pinyin_data_path = "data/Mandarin.dat";
        let output_bin_path = "bincode/unicode-to-pinyin.bin";

        println!(
            "Generating default bincode file: {} from {}",
            output_bin_path, input_pinyin_data_path
        );

        // Ensure bincode directory exists
        fs::create_dir_all("bincode")?;

        let pinyin_map_result = load_pinyin_map(input_pinyin_data_path);
        match pinyin_map_result {
            Ok(result) => {
                let encoded_bytes = save_to_vec(result)?;
                let mut output_file = fs::File::create(output_bin_path)?;
                output_file.write_all(&encoded_bytes)?;

                println!("Default bincode file generated successfully.");
            }
            Err(e) => println!("Error: {}", e), // Output: Error: No valid value found
        }

        Ok(())
    }

    // This block will only compile if 'prepare-data' feature is NOT enabled.
    // It provides a runtime error message.
    #[cfg(not(feature = "prepare-data"))]
    {
        use mandarin_to_pinyin::{
            diacritic_to_tone_plus_number, init_map, lookup_chars_for_str,
            lookup_chars_map_for_str, lookup_chars_vec_for_str, lookup_unicodes,
            lookup_unicodes_map, lookup_unicodes_vec, tone_plus_number_to_diacritic,
        };
        init_map(None)?;
        let lookup_result = lookup_unicodes(&vec![25497, 156094, 138716, 21340]);
        println!("testing lookup_unicodes: {lookup_result:?}");

        let lookup_result = lookup_unicodes_map(&vec![25497, 156094, 138716, 21340]);
        println!("testing lookup_unicodes_map: {lookup_result:?}");

        let lookup_result = lookup_unicodes_vec(&vec![25497, 156094, 138716, 21340]);
        println!("testing lookup_unicodes_vec: {lookup_result:?}");

        let lookup_result = lookup_chars_for_str("春眠不觉晓");
        println!("testing lookup_chars: {lookup_result:?}");

        let lookup_result = lookup_chars_map_for_str("春眠不觉晓");
        println!("testing lookup_chars_map: {lookup_result:?}");

        let lookup_result = lookup_chars_vec_for_str("春眠不觉晓");
        println!("testing lookup_chars_vec: {lookup_result:?}");

        let lookup_result = diacritic_to_tone_plus_number(&vec!["xiāng", "zhǐ", "lǘ"]);
        println!("testing diacritic_to_tone_plus_number: {lookup_result:?}");

        let lookup_result = tone_plus_number_to_diacritic(&vec!["xia1ng", "zhi3", "lü2", "jiv3"]);
        println!("testing tone_plus_number_to_diacritic: {lookup_result:?}");

        Ok(())
    }
}
