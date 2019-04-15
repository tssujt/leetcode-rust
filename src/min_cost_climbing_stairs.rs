impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost.clone();

        let l = cost.len();

        for i in 2..l {
            cost[i] += cost[i - 1].min(cost[i - 2]);
        }

        cost[l - 1].min(cost[l - 2])
    }
}

pub struct Solution;
