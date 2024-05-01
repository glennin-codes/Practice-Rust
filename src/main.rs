use std::str::from_utf8;
fn first_word(s: &String) -> &[u8]{
 
 let bytes: &[u8] = s.as_bytes();
 for (i, &item) in bytes.iter().enumerate() {
  if item == b' ' {
  println!("found a space at position : {}",i);
 let slice=&bytes[0..i];
 return slice;
 }
 ;

 }
let slice=&bytes[..];
  slice
}
fn main() {

    
       match from_utf8(first_word(& "magic's world!".to_string())){
        Ok(word)=>println!("this is the first word: {}",word),
        Err(err)=>println!("error: {}",err)
       }
   
      }
