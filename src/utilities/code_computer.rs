pub fn run_codes(codes: &mut Vec<usize>) {
    let mut index = 0;
    loop {
        let code = codes[index];
        if code == 99 {
            break;
        }

        let first_index = codes[index + 1];
        let second_index = codes[index + 2];
        let third_index = codes[index + 3];

        while first_index >= codes.len()
            || second_index >= codes.len()
            || third_index >= codes.len()
        {
            codes.push(0);
        }

        if code == 1 {
            codes[third_index] = codes[first_index] + codes[second_index];
        } else if code == 2 {
            codes[third_index] = codes[first_index] * codes[second_index];
        } else {
            panic!("Code not recognized!");
        }

        index += 4;
    }
}
