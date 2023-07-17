/*
 * (0,1) (1,3), (2,2), (3,5)
 *
 * idea: model as DP problem
 * create table T: n * m, n = length of input array, m = max (input array values)
 * T(i,j) = largest rectangle that can be built
 *
 *
 *
 * idea: pass the array once, keep i, initialize max to 0
 * at place i we have the biggest rect that can be built up to i-1
 * e.g. i = 0, current max is 0. at i=0, we can build a rect of size 1. update max = 1.
 * at i = 1, current max is 1. we can build rect of size 3. update max = 3.
 * at i = 2 , we acrtually need to check if A[i] <= A[i-1]. if so, we can build a rect with the
 * value before!
 */
fn main() {
    let rectangle_histogram = vec![1, 3, 2, 5];
    let largest_rectangle = solution(rectangle_histogram);
    dbg!(largest_rectangle);
}

fn solution(rectangle_histogram: Vec<i32>) -> i32 {
    let mut max = 0;
    if rectangle_histogram.len() == 1 {
        return rectangle_histogram[0];
    }
    for (i, e) in rectangle_histogram.iter().enumerate() {
        if i == 0 {
            max = *e;
        } else if e <= &rectangle_histogram[i - 1] {
            max += e
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_empty_list() {
        let rectangle_histogram = vec![];
        let largest_rectangle = solution(rectangle_histogram);
        assert_eq!(largest_rectangle, 0);
    }

    #[test]
    fn it_works_with_single_rectangle() {
        let rectangle_histogram = vec![1];
        let largest_rectangle = solution(rectangle_histogram);
        assert_eq!(largest_rectangle, 1);
    }

    #[test]
    fn it_works_with_sample_problem() {
        let rectangle_histogram = vec![1, 3, 2, 5];
        let largest_rectangle = solution(rectangle_histogram);
        assert_eq!(largest_rectangle, 6);
    }
}
