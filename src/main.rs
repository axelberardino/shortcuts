fn solve(vec_size: usize, shortcuts: Vec<i32>) -> Vec<i32> {
    let mut distances = vec![9999; vec_size];
    fn solve_rec(distances: &mut Vec<i32>, shortcuts: &Vec<i32>, idx: i32, price: i32) -> i32 {
        if idx as usize >= distances.len() {
            return price;
        }

        let price_normal = solve_rec(distances, shortcuts, idx + 1, price + 1) - 1;

        if idx != shortcuts[idx as usize] - 1 {
            let price_shortcut =
                solve_rec(distances, shortcuts, shortcuts[idx as usize] - 1, price + 1) - 1;
            if price_shortcut < price_normal {
                distances[idx as usize] = price_shortcut;
            } else {
                distances[idx as usize] = price_normal;
            }
        } else {
            distances[idx as usize] = price_normal;
        }

        distances[idx as usize]
    }

    solve_rec(&mut distances, &shortcuts, 0, 0);
    distances
}

fn main() {
    let res = solve(5, vec![1, 2, 3, 4, 5]);
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = solve(3, vec![2, 2, 3]);
        assert_eq!(res, vec![0, 1, 2]);
    }

    // #[test]
    // fn test_2() {
    //     let res = solve(5, vec![1, 2, 3, 4, 5]);
    //     assert_eq!(res, vec![0, 1, 2, 1, 2, 3, 3]);
    // }

    #[test]
    fn test_4() {
        let res = solve(7, vec![4, 4, 4, 4, 7, 7, 7]);
        assert_eq!(res, vec![0, 1, 2, 1, 2, 3, 3]);
    }
}
