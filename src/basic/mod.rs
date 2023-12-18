use rand::prelude::*;
/// multi-arm bandit
#[derive(Debug)]
pub struct Bandit {
    pub k: usize,            // k臂老虎机
    pub probes: Vec<f32>, // 每个杆的获奖概率
    pub best_idx: usize,     // 获奖概率最大的拉杆
    pub best_probe: f32,  // 最大的获奖概率
}

impl Bandit {
    pub fn new(k: usize) -> Self {
        let mut rng = thread_rng();
        let probes = (0..k)
            .map(|_| rng.gen_range(0f32..=1f32))
            .collect::<Vec<f32>>();

        let (best_idx, best_probe) = probes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, &v)| (i, v))
            .unwrap();

        Bandit {
            k,
            probes,
            best_idx,
            best_probe,
        }
    }

    pub fn step(&self, k: usize) -> i32 {
        let mut rng = thread_rng();
        if rng.gen_range(0f32..=1f32) < self.probes[k] {
            1
        } else {
            0
        }
    }
}

pub trait Strategy {
    //返回当前动作选择哪一根拉杆,由每个具体的策略实现
    fn run_one_step(&self) -> usize;
}

struct Solver<T: Strategy> {
    bandit: Bandit,
    count: Vec<u8>,   // 每根拉杆的尝试次数
    strategy: T,      // 具体策略类
    actions: Vec<usize>, //维护一个列表,记录每一步的动作
    regret: f32,
    regrets: Vec<f32>, //维护一个列表,记录每一步的累积懊悔
}

impl<T: Strategy> Solver<T> {
    pub fn new(bandit: Bandit, strategy: T) -> Self {
        let count = vec![0; bandit.k as usize];
        Solver {
            bandit,
            count,
            strategy,
            actions: vec![],
            regret: 0.0,
            regrets: vec![],
        }
    }

    fn update_regret(&mut self, k: usize) {
        self.regret += self.bandit.best_probe - self.bandit.probes[k];
        self.regrets.push(self.regret);
    }

    fn run(&mut self, num_step: usize) {
        for i in 0..num_step {
            let k = self.strategy.run_one_step();
            self.count[k] += 1;
            self.actions.push(k);
            self.update_regret(k as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bandit_new() {
        let bandit = Bandit::new(10);
        println!("{:?}", bandit);
        for i in 0..10 {
            println!("拉动第{}号杆，得到奖励 {}", i, bandit.step(i))
        }
    }

    #[test]
    fn solver_test() {
        let bandit = Bandit::new(10);
    }
}
