use super::residu::State;
use super::simple_wave::Wave;
// Linear reconstruction of conserved variables, with exact boundary samples.
pub struct Background {
    pub nodes: Vec<(f64, State)>,
}
impl Background {
    pub fn new(h: f64, phase: f64, wave: Wave, time: f64) -> Self {
        assert!(h > 0.0 && h.is_finite() && (0.0..=0.5).contains(&phase));
        let origin = phase * h;
        let count = ((120.0 - origin) / h).ceil() as usize + 1;
        let mut xs: Vec<_> = (0..count).map(|j| origin + j as f64 * h).collect();
        xs.extend([30.0, 90.0]);
        xs.sort_by(f64::total_cmp);
        xs.dedup();
        Self {
            nodes: xs.into_iter().map(|x| (x, wave.at(x, time))).collect(),
        }
    }
    fn segment(&self, x: f64) -> usize {
        assert!(x >= self.nodes[0].0 && x <= self.nodes.last().unwrap().0);
        self.nodes
            .partition_point(|(p, _)| *p <= x)
            .saturating_sub(1)
            .min(self.nodes.len() - 2)
    }
    pub fn at(&self, x: f64) -> State {
        let j = self.segment(x);
        let (a, fa) = self.nodes[j];
        let (b, fb) = self.nodes[j + 1];
        fa.plus(fb.minus(fa).times((x - a) / (b - a)))
    }
    pub fn at_time(&self, x: f64, wave: Wave, time: f64) -> State {
        let j = self.segment(x);
        let a = self.nodes[j].0;
        let b = self.nodes[j + 1].0;
        let fa = wave.at(a, time);
        fa.plus(wave.at(b, time).minus(fa).times((x - a) / (b - a)))
    }
    pub fn mean(&self, a: f64, b: f64) -> State {
        assert!(b > a);
        let mut x = a;
        let mut result = State::default();
        while x < b {
            let right = b.min(self.nodes[self.segment(x) + 1].0);
            assert!(right > x);
            result = result.plus(self.at(x).plus(self.at(right)).times(0.5 * (right - x)));
            x = right;
        }
        result.times(1.0 / (b - a))
    }
}
