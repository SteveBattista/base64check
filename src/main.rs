use std::str;
use data_encoding::BASE64;
fn main() {
    let mut buffer =[0;1024];
       let input = b"SGVsbG8gd29ybGQ=";
       let output = &mut buffer[0 .. BASE64.decode_len(input.len()).unwrap()];
       let len = BASE64.decode_mut(input, output).unwrap();
       let output = &mut buffer[0 .. len];
       let s =  str::from_utf8(input).unwrap();
       println!("{:?}",s);
       let s = str::from_utf8(output).unwrap();
       println!("{:?}",s);

       let mut buffer =[0;1024];
       let input2 = output;
       let output2 = &mut buffer[0 .. BASE64.encode_len(input2.len())];
       BASE64.encode_mut(input2, output2);
       let s = str::from_utf8(input2).unwrap();
       println!("{:?}",s);
       let s = str::from_utf8(output2).unwrap();
       println!("{:?}",s);


}
