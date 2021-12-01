use std::str::SplitWhitespace;

fn max_pairwise_slow(vector: &mut Vec<i64>) -> i64 {
    let length = vector.len();
    let mut product: i64 = 0;
    for x in 0..length {
        let x_value = vector.get(x).unwrap();
        for y in 0..length {
            let y_value = vector.get(y).unwrap();
            if x_value == y_value {
                continue;
            }
            let temp_product = x_value * y_value;
            if temp_product > product {
                product = temp_product;
            }
        }
    }
    return product;
}

fn max_pairwise_fast(mut words: SplitWhitespace, vector: &mut Vec<i64>) -> i64 {
    let (largest, second_largest) = find_two_largest_non_negative_integers(words.clone(), vector);
    largest * second_largest
}

fn find_two_largest_non_negative_integers(
    mut words: SplitWhitespace,
    vector: &mut Vec<i64>,
) -> (i64, i64) {
    let mut largest: i64 = 0;
    let mut second_largest: i64 = 0;
    // Your implementation goes here

    (largest, second_largest)
}

#[cfg(test)]
mod tests {
    use crate::week1::ii_maximum_pairwise_product::{max_pairwise_fast, max_pairwise_slow};
    use rand::Rng;

    #[test]
    fn stress_test() {
        let mut rng = rand::thread_rng();
        let mut count = 0;

        println!("Go");
        loop {
            let length = rng.gen::<usize>() % 10000 + 2;
            let mut word_str = String::new();
            for x in 0..length {
                let num = rng.gen::<usize>() % 200000;
                word_str.push_str(&*num.to_string());
                word_str.push(' ');
            }
            let words = word_str.split_whitespace();
            let mut vector: Vec<i64> = Vec::with_capacity(length);
            let product1 = max_pairwise_fast(words.clone(), &mut vector);

            let mut vector: Vec<i64> = Vec::with_capacity(length);
            for number_str in words {
                // println!("number is {}", number_str);
                let number = number_str.parse::<i64>().unwrap();
                vector.push(number);
            }
            let product = max_pairwise_slow(&mut vector);
            if product != product1 {
                assert_eq!(product, product1, "Failed as {} != {}", product, product1);

                break;
            }

            count += 1;
            if count >= 9000 {
                break;
            }
        }
    }
}
