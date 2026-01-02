use data_encoding::BASE64;
use rand::{distributions::Standard, thread_rng, Rng};
static NUMBER_OF_RUNS: usize = 20000000;
const MAX_U8_LEN: usize = 512;
const MAX_NUMBER_OF_UUENCODE_PER_U8: usize = 4;
static HOW_OFTEN_TO_REPORT: usize = 10000;
fn remove_right_zeros(input: &[u8]) -> &[u8] {
    let mut local = input;
    loop {
        if local.is_empty() {
            break;
        } else if *local.last().unwrap() == 0 {
            local = &local[..local.len() - 1];
        } else {
            break;
        }
    }
    local
}

fn does_uuencode_match_input(input: &[u8]) -> bool {
    let mut buffer = [0; MAX_U8_LEN * MAX_NUMBER_OF_UUENCODE_PER_U8];
    let output = &mut buffer[0..BASE64.encode_len(input.len())];
    BASE64.encode_mut(input, output);
    let mut buffer = [0; MAX_U8_LEN + 1];
    let output2 = &mut buffer[0..BASE64.decode_len(output.len()).unwrap()];
    BASE64.decode_mut(output, output2).unwrap();
    let output2 = remove_right_zeros(output2);
    let input = remove_right_zeros(input);
    input == output2
}
#[tokio::main]
async fn main() {
   
    let mut rng = rand::thread_rng();
    for run_number in 0..NUMBER_OF_RUNS {
        let input : Vec<u8> = thread_rng()
            .sample_iter(Standard)
            .take(rng.gen_range(0..MAX_U8_LEN))
            .collect::<Vec<u8>>();
            tokio::spawn(async move {
                 if !does_uuencode_match_input(&input) {
                    println!("{:?} won't convert", &input);
                }});
        
        if run_number % HOW_OFTEN_TO_REPORT == 0 {
            println!("Testing run number {}.", run_number)
        }
    }
}
