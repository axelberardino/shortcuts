use std::{collections::VecDeque, io::BufRead};

type AnyResult<T> = Result<T, ()>;

// To solve that, we're using a bfs traversal. Every time we found a new node
// (non visited yet or with a too costly dist) we're adding it to a traversal
// list.
//
// Nodes filling
// =============
//
// For each node we're attributing the value of all neighbors (left, right and
// shortcut).
//
// Example:
//        1 - 2 - 3
//         \_____/
// Dist = .   .   .
//  For 1, we're putting a score of 1 to (left: none) (right: 2) (shortcut: 3)
//  As we found a 2 and 3, we're putting them in the queue.
//
// Dist = 0   1   1
//
// As 2 and 3 are already filled with the cheapest distance, nothing more to do.
//
// Traversal of all nodes
// ======================
//
// We're doing the same nodes filling process on all affected nodes, until all
// nodes are done. By doing so, every nodes will be traversed once, and we're
// propagating the cost to the 3 neighbors of each nodes, keeping only the
// cheapest distance.
// Order matters, so we're computing the cost, "floor" by "floor", in an
// ascendant order.
//
// One way to visualize that, would be to draw it like a tree.
//
// Let's take an example:
//        1 - 2 - 3 - 4 - 5 - 6 - 7
//        |   |  \__//           /
//        |    \____/           /
//         \___________________/
//
// The graph would looks like this:
//                     1
//                    / \
//                   2   7
//                  / \  |
//                 3 - 4 6
//                     |/
//                     5
//
// Now, by keeping only the shortest path, one possible tree would be:
//              0:     1
//                    / \
//              1:   2   7
//                  / \   \
//              2: 3   4   6
//                      \
//              3:       5
//
// Here, each level of the tree would be the cost (distance).
// (Note that 5 could be attached to 6 instead)
fn solve(shortcuts: &[usize]) -> impl Iterator<Item = usize> {
    // If we found a better distance (meaning the current_dist is better than
    // the one we found in the given position), update the dist and push it to
    // the working queue.
    fn update_and_push_if_better(
        queue: &mut VecDeque<usize>,
        distances: &mut [Option<usize>],
        other: usize,
        current_dist: usize,
    ) {
        if distances[other].map_or(true, |distance_other| current_dist < distance_other) {
            distances[other] = Some(current_dist);
            queue.push_front(other);
        }
    }

    // Just to have 0-base indexing.
    let shortcuts = shortcuts.iter().map(|elt| elt - 1).collect::<Vec<_>>();
    let mut queue = VecDeque::<usize>::new();
    let mut distances = vec![None; shortcuts.len()];
    distances[0] = Some(0);

    queue.push_front(0);
    while let Some(current) = queue.pop_front() {
        let shortcut = shortcuts[current];
        let prev = (current > 0).then(|| current - 1);
        let next = (current < shortcuts.len() - 1).then(|| current + 1);
        let current_dist = distances[current].map_or(0, |current_dist| 1 + current_dist);

        if let Some(prev) = prev {
            update_and_push_if_better(&mut queue, &mut distances, prev, current_dist);
        }
        if let Some(next) = next {
            update_and_push_if_better(&mut queue, &mut distances, next, current_dist);
        }
        update_and_push_if_better(&mut queue, &mut distances, shortcut, current_dist);
    }
    distances.into_iter().flatten() // Assuming here there isn't any None.
}

fn main() -> AnyResult<()> {
    let mut lines = std::io::stdin().lock().lines().skip(1);
    let line = lines.next().unwrap().map_err(|_| ())?;
    let split = line
        .split(' ')
        .map(|ch| ch.parse().expect("no issue"))
        .collect::<Vec<_>>();

    let res = solve(&split).collect::<Vec<_>>();
    for elt in res {
        print!("{} ", elt);
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 - 2 - 3 - 4 - 5
    //
    // No shortcuts at all, linear score.
    #[test]
    fn test_identity() {
        let res = solve(&vec![1, 2, 3, 4, 5]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2, 3, 4]);
    }

    // 1 - 2 - 3
    // \___/
    //
    // Non useful shortcut, score would be the same without.
    #[test]
    fn test_non_useful_shortcuts() {
        let res = solve(&vec![2, 2, 3]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2]);
    }

    // 1 - 2 - 3
    // \______/
    //
    // Simple shortcut.
    #[test]
    fn test_basic_example() {
        let res = solve(&vec![3, 2, 3]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 1]);
    }

    // 1 - 2 - 3 - 4
    // |   |  \__///
    // |    \____//
    //  \________/
    //
    // Simply apply all shortcuts to reduce dist.
    #[test]
    fn test_one_shortcut_groups() {
        let res = solve(&vec![4, 4, 4, 4]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2, 1]);
    }

    // 1 - 2 - 3 - 4 - 5 - 6 - 7
    // |   |  \__///   |    \_//
    // |    \____//     \____//
    //  \________/
    //
    // Simply apply all shortcuts to reduce dist. Put two groups to ensure it's
    // still working as intended.
    #[test]
    fn test_two_shortcut_groups() {
        let res = solve(&vec![4, 4, 4, 4, 7, 7, 7]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2, 1, 2, 3, 3]);
    }

    // 1 - 2 - 3 - 4 - 5
    // \______________/
    //
    // Basic dist for all, except for 4!
    // To go to 4: start at 1, then shortcut to 5 then go to 4 in backward.
    // It means it's 2 dist and not 3.
    #[test]
    fn test_can_go_backward() {
        let res = solve(&vec![5, 2, 3, 4, 5]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2, 2, 1]);
    }

    // 1 - 2 - 3 - 4 - 5 - 6 - 7
    // |   |  \__//           /
    // |    \____/           /
    //  \___________________/
    //
    // Here, we're testing the main example, which handles shortcuts and a
    // backward shortcut.
    #[test]
    fn test_main_example() {
        let res = solve(&vec![7, 4, 4, 4, 5, 6, 7]).collect::<Vec<_>>();
        assert_eq!(res, vec![0, 1, 2, 2, 3, 2, 1]);
    }
}
