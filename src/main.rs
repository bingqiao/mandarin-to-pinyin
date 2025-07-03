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
        use mandarin_to_pinyin::{init_map, to_pinyin_string};
        use std::env;

        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: {} <chinese_string>", args[0]);
            return Ok(());
        }

        let chinese_string = &args[1];

        init_map(None)?;

        match to_pinyin_string(chinese_string, " ") {
            Ok(pinyin) => println!("{}", pinyin),
            Err(e) => eprintln!("Error converting to pinyin: {}", e),
        }

        Ok(())
    }
}
