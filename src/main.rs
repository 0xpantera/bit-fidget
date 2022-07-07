fn main() {
    let n = 17;
    let bin_n = dec_to_bin(n);

    println!("{:?}", bin_n);
}

fn dec_to_bin(mut decimal: i32) -> String {
    let len = f32::floor(f32::log2(decimal as f32)) + 1 as f32;
    let mut bits = vec![0; len as usize];
    
    if decimal == 0 {
       return decimal.to_string();
    } else {
        let mut i = 0;
        while decimal > 0 {
            bits[i] = decimal % 2;
            decimal /= 2;
            i += 1;
        }
    }

    bits
        .iter()
        .rev()
        .map(ToString::to_string)
        .collect::<String>()
        .trim_start_matches("0")
        .to_string()
}
