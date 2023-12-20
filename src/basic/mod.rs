mod bandit;
mod strategy;
mod solver;
use rand::prelude::*;
use crate::basic::bandit::Bandit;

#[cfg(test)]
mod tests {
    use crate::basic::solver::Solver;
    use crate::basic::strategy::EpsilonGreedy;
    use super::*;

    #[test]
    fn bandit_test() {
        let bandit = Bandit::new(10);
        println!("{:?}", bandit);
        for i in 0..10 {
            println!("拉动第{}号杆，得到奖励 {}", i, bandit.step(i))
        }
    }

    #[test]
    fn solver_test() {
        const EPSILON: f32 = 0.4;
        let bandit = Bandit::new(10);
        let epsilon_strategy = EpsilonGreedy::new(&bandit, EPSILON);
        let mut solver = Solver::new(&bandit, epsilon_strategy);
        solver.run(100);

        let data = solver.regrets
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect::<Vec<(usize, f32)>>();
        data.iter().for_each(|(index, regret)| println!("step:{}, regret:{}", index+1, regret));
    }
}
