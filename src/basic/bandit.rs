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