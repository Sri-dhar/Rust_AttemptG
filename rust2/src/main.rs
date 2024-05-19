// fn main() {
//     {
//         let s = "sridhar";
//         println!("{}", s);
//     }
// // }

// fn main()
// {
//     let mut strr = "hello how are you";
//     println!("{}", strr);
//
//     let mut str2 = strr;
//     println!("{}", str2);
//
//     println!("{}", strr);
// }

// fn main()
// {
//     let strr = "dskajf";
//     let str2 = strr;
//     println!("{}", str2);
//     println!("{}", strr);
// }

// fn main() {
//     let a = 23;
//     let b = a;
//     println!("{}", a);
//     println!("{}", b);
// }


// fn main() {
//     let mut s = String::with_capacity(50);
//     s.push_str("Hello, world!");
//
//     println!("Length: {}", s.len()); // Length is the number of bytes currently in the string
//     println!("Capacity: {}", s.capacity()); // Capacity is the total number of bytes the string can hold without reallocating memory
//
//     s.push_str(" Adding some more text.");
//
//
//     let scopy = s.clone();
//
//     println!("Length after adding text: {}", scopy.len());
//     println!("Capacity after adding text: {}", scopy.capacity());
// }

// fn main() {
//     let mut strr = String::from("Hello:)");
//     fnn(&mut strr);
//     println!("{}", strr);
// }

fn fnn(s: &mut String){
    s.push_str(" World!");
    println!("{}", s);
}

//example of mutable string reference

// fn main() {
//     let mut s = String::from("hello");
//     { let rr = &mut s; }
//     let rrr = &mut s;
//     println!("{}", rrr);
//     // println!("{}", rrr);
// }


// =============================
//SLICE TYPE IN RUST

// fn main() {
//     let mut s = String::from("Hello World");
//     let word = first_word(&s);
//     println!("{}", word);
//
//     let stringpart = &s[0..word];
//     println!("String part is : {}", stringpart);
//     let newS = &s[0..s.len()];
//
//     // 0..s.len() is equivalent to .. only
//     println!("{}", newS);
// }
//
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

// fn main() {
//     let mut s = String::from("Hello World");
//     let word = first_word2(&s);
//     s.clear();//we are trying to borrow the string mutably when
//     //first_word2 has already borrowed it immutably
//     //this is what is throwing the error here
//     //error:
//     println!("{}", word);
// }


fn main()
{
    let a = [1,2,3,4,5];
    let slice = &a[..3];
    println!("{:?}", slice);
    assert_eq!(slice, &[1,2,3]);
}