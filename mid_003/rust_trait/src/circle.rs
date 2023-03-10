use crate::graph_trait::Graph;

/// 圆
pub struct Circle {
    // 半径
    pub(crate) radius: f64,
}

impl Graph for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * 3.14;
    }
}
