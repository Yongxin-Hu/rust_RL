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
    fn run_one_step(&mut self) -> usize;
}

struct Solver<'a, T: Strategy> {
    bandit: &'a Bandit,
    strategy: T,      // 具体策略类
    actions: Vec<usize>, //维护一个列表,记录每一步的动作
    regret: f32,
    regrets: Vec<f32>, //维护一个列表,记录每一步的累积懊悔
}

impl<'a, T: Strategy> Solver<'a, T> {
    pub fn new(bandit: &'a Bandit, strategy: T) -> Self {
        let count = vec![0; bandit.k];
        Solver {
            bandit,
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
            self.actions.push(k);
            self.update_regret(k as usize);
        }
    }
}

//ϵ-贪心策略
struct EpsilonGreedy<'a>{
    bandit: &'a Bandit,
    epsilon: f32,
    count: Vec<usize>,   // 每根拉杆的尝试次数
    estimates: Vec<f32>
}

impl<'a> EpsilonGreedy<'a>{
    fn new(bandit: &'a Bandit, epsilon: f32) -> Self{
        EpsilonGreedy{
            bandit,
            epsilon,
            count: vec![0; bandit.k],
            estimates: vec![1.; bandit.k]
        }
    }
}

impl Strategy for EpsilonGreedy<'_>{
    fn run_one_step(&mut self) -> usize {
        let mut rng = thread_rng();
        let mut k = 0;
        if rng.gen_range(0f32..=1f32) < self.epsilon{
            k = rng.gen_range(0..self.bandit.k);
            println!("探索{k}号杆")
        }else{
            k = self.estimates
                .iter()
                .enumerate()
                .max_by(|(_, a),(_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, a)| i)
                .unwrap();
            println!("利用{k}号杆")
        }
        self.count[k] += 1;
        let r = self.bandit.step(k);
        self.estimates[k] += (1. / self.count[k] as f32) * (r as f32 - self.estimates[k]);
        k
    }
}

#[cfg(test)]
mod tests {
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
