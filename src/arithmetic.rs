pub fn add(a: String, b: String) {
    //  1        1
    //  11      11
    // +19     +91
    // ----    ----
    //  30      03

    let mut vec_a: Vec<u32> = vec![];
    let mut vec_b: Vec<u32> = vec![];
    for n in a.chars() {
        vec_a.insert(0, n.to_digit(10).unwrap() )
    }

    for n in b.chars() {
        vec_b.insert(0, n.to_digit(10).unwrap() )
    }

    let mut len: usize = 0;
    if a.len() > b.len() {
        len = a.len();
    } else {
        len = b.len();
    }

    let mut answer: Vec<u32> = vec![];
    let mut sum: u32 = 0;
    let mut remainder: u32 = 0;
    for i in 0 .. len {
        if vec_b.get(i) != None {
            sum = vec_a[i] + vec_b[i] + remainder;
        } else {
            sum = vec_a[i] + remainder;
        }
        if sum > 9 {
            remainder = sum - 10;
            if remainder == 0 {
                remainder = 1;
            }
            sum = 0;
        }
        answer.push(sum);
    }

    answer.reverse();
    for i in answer {
        print!("{}", i);
    }
}