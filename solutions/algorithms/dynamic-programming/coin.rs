fn main() {
    let amount = 12;
    let coins = vec![2, 4, 5];

    println!("Number of combinations of getting change for {} is: {}", amount, change(&coins, amount));
    println!("Minimum number of coins required for amount {} is: {}", amount, minimum_coins(&coins, amount));
}

/// This function finds the number of combinations of getting change for a given amount and set of coins.
fn change(coins: &[usize], amount: usize) -> usize {
    let mut combinations = vec![0; amount + 1];
    combinations[0] = 1;

    for &coin in coins {
        for i in coin..=amount {
            combinations[i] += combinations[i - coin];
        }
    }

    combinations[amount]
}

/// This function finds the minimum number of coins needed for a given amount.
fn minimum_coins(coins: &[usize], amount: usize) -> usize {
    let mut minimum_coins = vec![0; amount + 1];

    for i in 1..=amount {
        minimum_coins[i] = usize::MAX;
    }

    for i in 1..=amount {
        for &coin in coins {
            if coin <= i {
                let sub_res = minimum_coins[i - coin];
                if sub_res != usize::MAX && sub_res + 1 < minimum_coins[i] {
                    minimum_coins[i] = sub_res + 1;
                }
            }
        }
    }

    minimum_coins[amount]
}
