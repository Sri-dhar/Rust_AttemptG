fn main1() {
    // println!("Hello, world!");
    let strr : &str =" hello";
    let sttrr : String = "hellow world".to_string();
    let str2 : String = String::from("Hleo world");
    let str3: String = "hellow world".to_owned();
    println!("{}", strr);
    println!("{}", sttrr);
    println!("{}", str2);
}

//strings by lets get rusty
//uses UTF 8 encoding


fn main(){
    let s1:String = String::from("hello");
    let s2: String = String::from(" world");
    let s3 : String = s1 + &s2;
    println!("{}",s3);

    //using format macro
    let s4 : String = String::from("hello");
    let s5 : String = String::from("world");
    let s6 : String = format!("{} {}",s4,s5);

    //format macro is less efficient than using + operator
    //beacuse it creates a new string by copying the data

    //slicing strings
    let s7 : String = String::from("hello");
    let s8 : &str = &s7[0..4];
    println!("{}",s8);

    //accessing strings like arrays
    let s9 : String = String::from("hello");
    let s10 : Vec<char> = s9.chars().collect();
    println!("{}",s10[0]);

    // println!("{}",s9[0]);  wrong

    //iterating over strings
    for c in s9.chars(){
        print!("{} ",c);
    }
    //iterating over characters
    println!();
    println!("We will get unicode values here");
    for c in s9.bytes(){
        print!("{} ",c);
    }

    println!();
    ///rust automatically converts a passed string literal
    /// to a string slice
    /// it is called automatic dereferencing
    ///
    /// and about automatic string coercian
    /// rust can automatically convert a string slice to a string
    /// so that we can use the + operator

    //constant time access to a string char is not available in rust
    //because of the UTF 8 encoding
    //it uses 4 bytes to store a single character

}


//In Rust, accessing a character in a string is not a constant-time operation because strings in Rust are UTF-8 encoded, and UTF-8 is a variable-width encoding. This means that characters in a string can take up different numbers of bytes.
//
// To access a character in a string, Rust needs to iterate over the bytes of the string until it finds the start of the desired character. This process involves checking byte by byte to identify the boundaries of each character, and the number of iterations depends on the encoding of the string.
//
// In contrast, languages with constant-time character access often use fixed-width encodings like ASCII, where each character takes up a consistent number of bytes, allowing for constant-time indexing.
//
// Rust provides other ways to work with characters efficiently, such as using iterators and methods like chars() to iterate over the characters in a string without exposing the underlying byte representation. However, direct constant-time indexing into a string is not supported due to the variable-width nature of UTF-8 encoding.