use crate::basic::bandit::Bandit;
use crate::basic::strategy::Strategy;

pub struct Solver<'a, T: Strategy> {
    bandit: &'a Bandit,
    strategy: T,      // 具体策略类
    actions: Vec<usize>, //维护一个列表,记录每一步的动作
    regret: f32,
    pub regrets: Vec<f32>, //维护一个列表,记录每一步的累积懊悔
}

impl<'a, T: Strategy> Solver<'a, T> {
    pub fn new(bandit: &'a Bandit, strategy: T) -> Self {
        Solver {
            bandit,
            strategy,
            actions: vec![],
            regret: 0.0,
            regrets: vec![],
        }
    }

    pub fn update_regret(&mut self, k: usize) {
        self.regret += self.bandit.best_probe - self.bandit.probes[k];
        self.regrets.push(self.regret);
    }

    pub fn run(&mut self, num_step: usize) {
        for _ in 0..num_step {
            let k = self.strategy.run_one_step();
            self.actions.push(k);
            self.update_regret(k as usize);
        }
    }
}