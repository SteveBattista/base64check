use std::str;
use data_encoding::BASE64;
fn main() {
    let mut buffer =[0;1024];
       let input = b"SGVsbG8gd29ybGQ=";
       let output = &mut buffer[0 .. BASE64.decode_len(input.len()).unwrap()];
       let len = BASE64.decode_mut(input, output).unwrap();
       let output = &mut buffer[0 .. len];
       let s0 =  str::from_utf8(input).unwrap().to_string();
       let s1 = str::from_utf8(output).unwrap().to_string();
       println!("\"{}\" decodes to \"{}\"",s0,s1);
       

       let mut buffer =[0;1024];
       let input2 = output;
       let output2 = &mut buffer[0 .. BASE64.encode_len(input2.len())];
       BASE64.encode_mut(input2, output2);
       let s2 = str::from_utf8(input2).unwrap().to_string();
       let s3 = str::from_utf8(output2).unwrap().to_string();
       println!("\"{}\" decodes to \"{}\"",s2,s3);

       assert_eq!(s0,s3);
       assert_eq!(s1,s2);


}
