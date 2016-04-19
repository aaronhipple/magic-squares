fn create_square(sequence: &mut Vec<u32>) -> Vec<Vec<u32>> {
    let square = vec![
        sequence[0 .. 3].to_vec(),
        sequence[3 .. 6].to_vec(),
        sequence[6 .. 9].to_vec()
    ];

    return square;
}

fn check_square(square: &Vec<Vec<u32>>) -> bool {
    let mut y = 0;
    let mut x = 0;
    let mut sum = 0;
    let mut prev = 0;

    while y < 3 {
        sum = square[y][0] + square[y][1] + square[y][2];
        if prev != 0 && sum != prev { return false; }
        prev = sum;
        y += 1;
    };

    while x < 3 {
        sum = square[0][x] + square[1][x] + square[2][x];
        if prev != 0 && sum != prev { return false; }
        prev = sum;
        x += 1;
    }

    sum = square[0][0] + square[1][1] + square[2][2];
    if prev != 0 && sum != prev { return false; }
    prev = sum;

    sum = square[2][0] + square[1][1] + square[0][2];
    if prev != 0 && sum != prev { return false; }

    return true;
}

fn print_square(square: Vec<Vec<u32>>) {
    let mut y = 0;
    
    while y < 3 {
        println!("Row {}: {:?}", y + 1, square[y]);
        y += 1;
    }
    println!("\n");
}

fn generate(n : usize, a : &mut Vec<u32>) {
    if n == 1 {
        let square = create_square(a);
        if check_square(&square) == true {
            println!("Sequence: {:?}", a);
            print_square(square);
        }
    }
    else {
        for i in  0 .. n - 1 {
            generate(n - 1, a);

            if n % 2 == 0 {
                a.swap(i, n - 1);
            }
            else {
                a.swap(0, n - 1);
            }
        }
        generate(n - 1, a);
    }
}

fn main() {
    for n in 1..1001 {
        let mut sequence: Vec<u32> = vec![
            (n+0)^1,
            (n+1)^1,
            (n+2)^1,
            (n+3)^1,
            (n+4)^1,
            (n+5)^1,
            (n+6)^1,
            (n+7)^1,
            (n+8)^1
        ];

        generate(9, &mut sequence);
    }

}