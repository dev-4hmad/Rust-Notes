# Chainable methods can be made by Ownership + self
impl Circle {
    fn scale(mut self, factor: f64) -> Self {
        self.radius *= factor;
        self
    }
}

let c = Circle::new(10.0).scale(2.0);
