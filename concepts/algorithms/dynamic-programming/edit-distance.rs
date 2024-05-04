/*
  A DynamicProgramming based solution for Edit Distance problem In Java
  Description of Edit Distance with an Example:

  Edit distance is a way of quantifying how dissimilar two strings (e.g., words) are to one another,
  by counting the minimum number of operations required to transform one string into the other. The
  distance operations are the removal, insertion, or substitution of a character in the string.


  The Distance between "kitten" and "sitting" is 3. A minimal edit script that transforms the former into the latter is:
  
  kitten → sitten (substitution of "s" for "k")
  sitten → sittin (substitution of "i" for "e")
  sittin → sitting (insertion of "g" at the end).
 
 
 */

use std::io;


fn min_distance(word1: &str, word2: &str) -> usize {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();

    // Cria  matriz para armazenar as distândistanciascias.
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // Preenche as condições iniciais.
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    // Preenche a matriz com as distancias de edição.
    for (i, c1) in word1.chars().enumerate() {
        for (j, c2) in word2.chars().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                let replace = dp[i][j] + 1;
                let insert = dp[i][j + 1] + 1;
                let delete = dp[i + 1][j] + 1;
                dp[i + 1][j + 1] = replace.min(insert).min(delete);
            }
        }
    }

    // Retorna a distancia de edicao entre as duas strings.
    dp[len1][len2]
}

fn main() {
    
    println!("Enter the First String:");
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("Failed to read line");
    let s1 = s1.trim();

    println!("Enter the Second String:");
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("Failed to read line");
    let s2 = s2.trim();

    let ans = min_distance(s1, s2);
    println!("The minimum Edit Distance between \"{}\" and \"{}\" is {}", s1, s2, ans);
}
