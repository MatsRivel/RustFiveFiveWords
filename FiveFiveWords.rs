use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::vec::Vec;
use std::str::from_utf8;
#[allow(unused_imports)]
use std::env;

fn duplicate_or_non_letter_chars(word: &&str)->bool{
    let mut arr: [bool;26] = [false;26];
    let usize_a = 97usize;  // 'a' as usize
    let usize_z = 122usize; // 'z' as usize
    let usize_capital_a = 65usize;  // 'A' as usize
    let usize_capital_z = 90usize; // 'Z' as usize

    for c in word.chars(){
        let mut c_usize = c as usize;
        //print!("\nC: {} a: {} c: {} z: {}\n", c, usize_a, c_usize, usize_z);
        if (c_usize < usize_capital_a) || (c_usize > usize_capital_z && c_usize < usize_a) || (c_usize > usize_z){ // not between 'A' and 'Z' or 'a' and 'z'?
            return true; // Non-numeric character.
        }
        if c_usize >= usize_a{ // Adjust index for lower case letters.
            c_usize -= usize_a;
        }else{
            c_usize -= usize_capital_a; // Adjust index for upper case letters.
        }
        if arr[c_usize]{
            print!("Letter \"{}\" has been seen before! The word is \"{word}\"",c);
            return true; // Duplicate character
        }else{
            arr[c_usize]=true; // Character found once.
        }
    };
    return false; // No issues found.
}

fn word_check(word:&&str) -> bool{
    if word.len() != 5 || duplicate_or_non_letter_chars(&word){
        return false;
    }
    return true;
}

fn main()-> Result<(),Error>{
    //env::set_var("RUST_BACKTRACE","1");
    let file_name = "FewEnglishWords.txt";
    let mut file_in = File::open(file_name).expect("Failed to open file");
    let mut words_in = Vec::new();
    file_in.read_to_end(&mut words_in).expect("Failed to read file");
    
    let words_string = from_utf8(&words_in).expect("Failed to convert to string"); 
    let words: Vec<&str> = words_string.trim().split("\n").collect();
    let mut good_words = Vec::new();


    for word in words.iter(){
        if word_check(&word.trim_end()){
            print!("O {}\n", word.trim_end());
            good_words.push(word.trim_end());
            //print!("{} ", word);
        }else{
            print!("X {}\n",word.trim_end());
        }
    }
    




    Ok(())
}