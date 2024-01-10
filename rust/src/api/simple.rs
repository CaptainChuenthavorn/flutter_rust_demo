use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;

use std::env;
use std::path::Path;
pub fn getcurrentdir() -> std::io::Result<(String)> {

    let path = env::current_dir()?;
    let result = format!("The current directory is {}", path.display());
    // println!("The current directory is {}", path.display());
    Ok(result)
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    let mut result = String::new();  // Initialize an empty string to accumulate results
    let entries = vec!["usr","Applications","Rust", "target", "debug"];  // Use a vector instead of an array
    for entry in entries {
        let exists = Path::new(entry).exists();
        result.push_str(&format!("{}: {}\n", entry, exists.to_string()));
    }
    result


    // format!("Hello, {name}!")
    // let path = Path::new("kk.txt").exists();
    // let path = Path::new("../word_th.txt").exists();
    // let path = Path::new("word_th.txt").exists();
    // format!("{name}{path}")
    // println!("{}", Path::new("../word_th.txt").exists());
    // println!("{}", Path::new("../src/word_th.txt").exists());
    // match env::current_dir() {
    //     Ok(current_dir) => {
    //         if let Some(path) = current_dir.to_str() {
    //             format!("{}", path)
    //         } else {
    //             format!("Error converting path to string")
    //         }
    //     }
    //     Err(e) => {
    //         format!("Error getting current directory: {}", e)
    //     }
    // }
    // match env::current_dir() {
    //     Ok(current_dir) => {
    //         match current_dir.read_dir() {
    //             Ok(entries) => {
                    
    //                 result  // Explicitly return the accumulated result
    //             }
    //             Err(e) => {
    //                 format!("Error reading directory: {}", e)
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         format!("Error getting current directory: {}", e)
    //     }
    // }

}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
pub fn hello(a: String) -> String { a.repeat(2) }
pub fn my_rust_function(a: String) -> String { a.repeat(2) }
// #[flutter_rust_bridge::frb(sync)]
pub fn get_token(input: String) -> Vec<String> {
    
    let tokenizer = NewmmTokenizer::new("rust/src/words_th.txt");
    let tokens = tokenizer.segment(&input, true, false).unwrap();
    let token_strings: Vec<String> = tokens.iter().cloned().collect();

    // Print tokens
    for token in &token_strings {
        println!("{}", token);
    }

    token_strings
}

// pub fn get_token(input: String) -> Vec<String> {
//     let tokenizer = NewmmTokenizer::new("rust/words_th.txt");
//     let tokens = tokenizer.segment(&input, true, false).unwrap();
//     println!("{:?}", tokens); 
//     format!("Token: {tokens}!")
//     tokens.into_iter().collect()
    
// }