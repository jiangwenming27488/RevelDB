use std::fmt::Display;
const K_NUM_BUCKETS: usize = 154;
#[derive(Debug)]
struct Histogram {
    min_: f64,
    max_: f64,
    num_: i32,
    sum_: f64,
    sum_squares_: f64,
    buckets_: [f64; K_NUM_BUCKETS],
}


impl Drop for Histogram {
    fn drop(&mut self) {
        println!("destruction the class histogram");
    }
}

impl Histogram {
    const K_BUCKET_LIMIT: [f64; K_NUM_BUCKETS] = [
        1f64,
        2.0f64,
        3.0f64,
        4.0,
        5.0,
        6.0,
        7.0,
        8.0,
        9.0,
        10.0,
        12.0,
        14.0,
        16.0,
        18.0,
        20.0,
        25.0,
        30.0,
        35.0,
        40.0,
        45.0,
        50.0,
        60.0,
        70.0,
        80.0,
        90.0,
        100.0,
        120.0,
        140.0,
        160.0,
        180.0,
        200.0,
        250.0,
        300.0,
        350.0,
        400.0,
        450.0,
        500.0,
        600.0,
        700.0,
        800.0,
        900.0,
        1000.0,
        1200.0,
        1400.0,
        1600.0,
        1800.0,
        2000.0,
        2500.0,
        3000.0,
        3500.0,
        4000.0,
        4500.0,
        5000.0,
        6000.0,
        7000.0,
        8000.0,
        9000.0,
        10000.0,
        12000.0,
        14000.0,
        16000.0,
        18000.0,
        20000.0,
        25000.0,
        30000.0,
        35000.0,
        40000.0,
        45000.0,
        50000.0,
        60000.0,
        70000.0,
        80000.0,
        90000.0,
        100000.0,
        120000.0,
        140000.0,
        160000.0,
        180000.0,
        200000.0,
        250000.0,
        300000.0,
        350000.0,
        400000.0,
        450000.0,
        500000.0,
        600000.0,
        700000.0,
        800000.0,
        900000.0,
        1000000.0,
        1200000.0,
        1400000.0,
        1600000.0,
        1800000.0,
        2000000.0,
        2500000.0,
        3000000.0,
        3500000.0,
        4000000.0,
        4500000.0,
        5000000.0,
        6000000.0,
        7000000.0,
        8000000.0,
        9000000.0,
        10000000.0,
        12000000.0,
        14000000.0,
        16000000.0,
        18000000.0,
        20000000.0,
        25000000.0,
        30000000.0,
        35000000.0,
        40000000.0,
        45000000.0,
        50000000.0,
        60000000.0,
        70000000.0,
        80000000.0,
        90000000.0,
        100000000.0,
        120000000.0,
        140000000.0,
        160000000.0,
        180000000.0,
        200000000.0,
        250000000.0,
        300000000.0,
        350000000.0,
        400000000.0,
        450000000.0,
        500000000.0,
        600000000.0,
        700000000.0,
        800000000.0,
        900000000.0,
        1000000000.0,
        1200000000.0,
        1400000000.0,
        1600000000.0,
        1800000000.0,
        2000000000.0,
        2500000000.0,
        3000000000.0,
        3500000000.0,
        4000000000.0,
        4500000000.0,
        5000000000.0,
        6000000000.0,
        7000000000.0,
        8000000000.0,
        9000000000.0,
        1e200];

    pub fn clear(&mut self) {
        self.min_ = Histogram::K_BUCKET_LIMIT[K_NUM_BUCKETS - 1];
        self.max_ = 0.0f64;
        self.num_ = 0i32;
        self.sum_ = 0.0f64;
        self.sum_squares_ = 0.0f64;
        self.buckets_ = [0.0f64; K_NUM_BUCKETS];
    }

    pub fn add(&mut self, value: f64) {
        // Linear search is fast enough for our usage in db_bench
        let mut b: usize = 0;
        while (b < K_NUM_BUCKETS - 1) && (Self::K_BUCKET_LIMIT[b] <= value) {
            b += 1;
        }
        self.buckets_[b] += 1.0f64;
        if self.min_ > value {
            self.min_ = value;
        }
        if self.max_ < value {
            self.max_ = value;
        }
        self.num_ += 1;
        self.sum_ += value;
        self.sum_squares_ += value * value;
    }


    pub fn merge(&mut self, other: &Histogram) {
        if other.min_ < self.min_ {
            self.min_ = other.min_;
        }
        if other.max_ > self.max_ {
            self.max_ = other.max_;
        }
        self.num_ += other.num_;
        self.sum_ += other.sum_;
        self.sum_squares_ += other.sum_squares_;
        for i in 0..K_NUM_BUCKETS {
            self.buckets_[i] = other.buckets_[i];
        }
    }

    pub fn median(&self) -> f64 {
        self.percentile(50.0)
    }


    pub fn percentile(&self, percent: f64) -> f64 {
        let threshold: f64 = (self.num_ as f64) * (percent / 100.0);
        let mut sum: f64 = 0.0f64;
        for i in 0..K_NUM_BUCKETS {
            sum += self.buckets_[i];
            if sum >= threshold {
                // Scale linearly within this bucket
                let left_point = if i == 0 {
                    0.0
                } else {
                    Histogram::K_BUCKET_LIMIT[i - 1]
                };

                let right_point: f64 = Self::K_BUCKET_LIMIT[i];
                let left_sum: f64 = sum - self.buckets_[i];
                let right_sum: f64 = sum;
                let pos: f64 = (threshold - left_sum) / (right_sum - left_sum);
                let mut r: f64 = left_point + (right_point - left_point) * pos;
                if r < self.min_ {
                    r = self.min_;
                }
                if r > self.max_ {
                    r = self.max_;
                }
                return r;
            }
        }
        self.max_
    }

    fn average(&self) -> f64 {
        if self.num_ == 0i32 {
            return 0.0;
        }
        self.sum_ / (self.num_ as f64)
    }

    fn new() -> Histogram {
        Histogram {
            min_: 0.0f64,
            max_: 0.0f64,
            num_: 0i32,
            sum_: 0.0f64,
            sum_squares_: 0.0f64,
            buckets_: [0.0f64; K_NUM_BUCKETS],
        }
    }

    pub fn standard_deviation(&self) -> f64 {
        if self.num_ == 0i32 {
            return 0.0;
        }
        let variance: f64 = (self.sum_squares_ * self.num_ as f64 - self.sum_ * self.sum_) / (self.num_ * self.num_) as f64;
        variance.sqrt()
    }
}
impl Display for Histogram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = format!("Count: {:.2}  Average: {:.4}  StdDev: {:.2}\n",
                            self.num_ as f64, self.average(), self.standard_deviation());

        let buf =
            format!("Min: {:.4}  Median: {:.4}  Max: {:.4}\n",
                    if self.num_ == 0i32 { 0.0 } else {
                        self.min_
                    }, self.median(), self.max_);
        r = r + &buf;
        let buf = "------------------------------------------------------\n";
        r = r + &buf;

        let mul = 100.0 / self.num_ as f64;
        let mut sum = 0.0;
        for i in 0..K_NUM_BUCKETS {
            if self.buckets_[i] <= 0.0 {
                continue;
            }
            sum += self.buckets_[i];
            let buf =
                format!("[ {:.7}, {:.7} ) {:.7}  {:7.3}%  {:7.3}% ",
                        if i == 0 {
                            0.0
                        } else {
                            Self::K_BUCKET_LIMIT[i - 1]
                        },  // left
                        Self::K_BUCKET_LIMIT[i],                         // right
                        self.buckets_[i],                             // count
                        mul * self.buckets_[i],                      // percentage
                        mul * sum);  // cumulative percentage
            r = r + &buf;
            // Add hash marks based on percentage; 20 marks for 100%.
            let marks = (20.0 * self.buckets_[i] / (self.num_ as f64) + 0.5) as usize;
            let buf = format!("{}\n", "#".repeat(marks));
            r = r + &buf;
        }
        write!(f, "{}", r)
    }
}
