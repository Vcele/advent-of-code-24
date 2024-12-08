use std::fs;

fn radixsort(vec: &mut Vec<usize>) {
    //sort optimized for 5 digit numbers
    for i in 0..5 {
        let mut array: [Vec<usize>; 10] = Default::default();
        for &num in vec.iter() {
            let mut temp = num;
            for _ in 0..i {
                temp = temp / 10;
            }
            temp = temp % 10;
            array[temp].push(num);
        }
        vec.clear();
        for mut bucket in array {
            vec.append(&mut bucket);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for pair in contents.split("\n") {
        let mut nums = pair.split("   ");
        if pair == "" {
            continue;
        }
        l1.push(nums.next().unwrap().parse::<usize>().unwrap());
        l2.push(nums.next().unwrap().parse::<usize>().unwrap());
    }

    use std::time::Instant;
    let now = Instant::now();

    radixsort(&mut l1);
    radixsort(&mut l2);
    let mut result = 0;
    for i in 0..l1.len() {
        let num1 = l1.get(i).unwrap();
        let num2 = l2.get(i).unwrap();
        result = result + num1.abs_diff(*num2);
    }
    println!("{}", result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
