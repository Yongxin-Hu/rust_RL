use rand::prelude::*;
use crate::basic::bandit::Bandit;

pub trait Strategy {
    //返回当前动作选择哪一根拉杆,由每个具体的策略实现
    fn run_one_step(&mut self) -> usize;
}

//ϵ-贪心策略
pub struct EpsilonGreedy<'a>{
    bandit: &'a Bandit,
    epsilon: f32,
    count: Vec<usize>,   // 每根拉杆的尝试次数
    estimates: Vec<f32>
}

impl<'a> EpsilonGreedy<'a>{
    pub fn new(bandit: &'a Bandit, epsilon: f32) -> Self{
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