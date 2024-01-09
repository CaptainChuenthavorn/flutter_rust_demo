use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")

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
    let tokenizer = NewmmTokenizer::new("rust/words_th.txt");
    let tokens = tokenizer.segment(&input, true, false).unwrap();
    // for token in tokens {
    //     println!("{}", token);
    // }
    // Collect tokens into a Vec<String>
    // Collect tokens into a Vec<String>
    let token_strings: Vec<String> = tokens.iter().cloned().collect();

    // Print tokens
    for token in &token_strings {
        println!("{}", token);
    }

    // Return the Vec<String>
    token_strings
    // format!("Hello, {input}!")
}

// pub fn get_token(input: String) -> Vec<String> {
//     let tokenizer = NewmmTokenizer::new("rust/words_th.txt");
//     let tokens = tokenizer.segment(&input, true, false).unwrap();
//     println!("{:?}", tokens); 
//     format!("Token: {tokens}!")
//     tokens.into_iter().collect()
    
// }