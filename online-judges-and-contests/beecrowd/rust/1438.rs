use std::io::{self, BufRead};

const MAX: usize = 1000;

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut numbers = input_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let total_boxes = numbers.next().unwrap();
        let num_stacks = numbers.next().unwrap();
        
        if total_boxes == 0 && num_stacks == 0 {
            break;
        }

        let mut sizes = vec![0; MAX];
        let mut ans = 0;
        let mut stack_with_one = 0;

        for _ in 0..num_stacks {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut stack_sizes = input_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let stack_len = stack_sizes.next().unwrap();
            sizes.push(stack_len);

            for p in 0..stack_len {
                let box_value = stack_sizes.next().unwrap();
                if box_value == 1 {
                    ans = sizes[stack_with_one] - p - 1;
                    sizes[stack_with_one] = p + 1;
                    stack_with_one = stack_len;
                }
            }
        }

        let mut left = 0;
        for j in (0..stack_with_one).rev() {
            if sizes[j] >= sizes[stack_with_one] {
                left += sizes[j] - sizes[stack_with_one] + 1;
            } else {
                break;
            }
        }

        let mut right = 0;
        for j in (stack_with_one + 1)..num_stacks {
            if sizes[j] >= sizes[stack_with_one] {
                right += sizes[j] - sizes[stack_with_one] + 1;
            } else {
                break;
            }
        }

        println!("{}", ans + left.min(right));
    }
}
