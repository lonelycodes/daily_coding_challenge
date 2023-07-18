fn main() {
    println!("Hello, world!");
}

fn solution(arr: Vec<Vec<char>>, target: &str) -> bool {
    solution_naive(arr, target)
}

fn solution_naive(arr: Vec<Vec<char>>, target: &str) -> bool {
    let n = arr.len();
    if n == 0 {
        return false;
    }
    let m = arr[0].len();
    dbg!(m.clone());

    for (i, e) in arr.iter().enumerate() {
        let word: String = e.into_iter().collect();
        if word == target {
            return true;
        }

        let mut current_word = String::new();
        for j in 0..m {
            current_word.push(arr[j][i]);
        }
        dbg!(current_word.clone());
        if current_word == target {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_with_empty_input() {
        let arr = vec![vec![]];
        let target = "TARGET";
        let result = solution(arr, target);
        assert_eq!(result, false)
    }

    #[test]
    fn it_works_with_single_character() {
        let arr = vec![vec!['A']];
        let valid_target = "A";
        let invalid_target = "B";
        assert_eq!(solution(arr.clone(), invalid_target), false);
        assert_eq!(solution(arr.clone(), valid_target), true);
    }

    #[test]
    fn it_works_with_sample() {
        let arr = vec![
            vec!['F', 'A', 'C', 'I'],
            vec!['O', 'B', 'Q', 'P'],
            vec!['A', 'N', 'O', 'B'],
            vec!['M', 'A', 'S', 'S'],
        ];
        let target = "FOAM";
        let result = solution(arr, target);
        assert_eq!(result, true)
    }

    #[test]
    fn it_works_with_two_characters() {
        let arr = vec![vec!['A'], vec!['B']];
        let target = "AB";
        let result = solution(arr, target);
        assert_eq!(result, true)
    }
}
