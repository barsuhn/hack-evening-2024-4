#[derive(Debug,Copy,Clone)]
pub struct Measurement {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}

impl Measurement {
    pub fn new(value: f64) -> Self {
        Measurement { count: 1, sum: value, min: value, max: value }
    }

    pub fn add(&mut self, value: f64) {
        if value < self.min {
            self.min = value;
        } else if value > self.max {
            self.max = value
        }

        self.sum += value;
        self.count += 1;
    }
}

#[derive(Debug,Clone)]
pub struct Weather {
    pub name: String,
    pub measurement: Measurement
}

impl Weather {
    pub fn new(name: String, measurement: Measurement) -> Self {
        Weather {name, measurement}
    }

    pub fn summarize(&self) -> String {
        let min = self.measurement.min;
        let max = self.measurement.max;
        let mean = self.measurement.sum / self.measurement.count as f64;

        format!("{}={:.1}/{:.1}/{:.1}", self.name, min, mean, max)
    }
}