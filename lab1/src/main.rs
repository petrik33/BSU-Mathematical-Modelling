use std::cmp::max;

struct MultiplicativeCongruentialMethod {
    state: u32,
    b: u32,
}

impl MultiplicativeCongruentialMethod {
    fn new(a: u32, c: u32) -> MultiplicativeCongruentialMethod {
        MultiplicativeCongruentialMethod {
            state: a,
            b: max(c, (1u32 << 31) - c),
        }
    }
}

struct McLarenMarsagliaMethod {
    generator1: Box<dyn RandGenerator>,
    generator2: Box<dyn RandGenerator>,
    table: Vec<f64>,
}

impl McLarenMarsagliaMethod {
    fn new(
        mut generator1: Box<dyn RandGenerator>,
        mut generator2: Box<dyn RandGenerator>,
        table_size: usize,
    ) -> McLarenMarsagliaMethod {
        let mut table = Vec::with_capacity(table_size);

        for _ in 0..table_size {
            table.push(generator1.rand());
        }

        McLarenMarsagliaMethod {
            generator1,
            generator2,
            table,
        }
    }
}

pub trait RandGenerator {
    fn rand(&mut self) -> f64;
}

impl RandGenerator for MultiplicativeCongruentialMethod {
    fn rand(&mut self) -> f64 {
        self.state = ((self.state as u64 * self.b as u64) % (1u32 << 31) as u64) as u32;
        self.state as f64 / (1u32 << 31) as f64
    }
}

impl RandGenerator for McLarenMarsagliaMethod {
    fn rand(&mut self) -> f64 {
        let rand_f = self.generator2.rand();
        let index: usize = (rand_f * (self.table.len()) as f64) as usize;
        let result = self.table[index];
        self.table[index] = self.generator1.rand();
        result
    }
}

struct RandSet {
    generator: Box<dyn RandGenerator>,
    data: Vec<f64>,
}

impl RandSet {
    fn new(mut generator1: Box<dyn RandGenerator>) -> RandSet {
        RandSet {
            generator: generator1,
            data: Vec::new(),
        }
    }

    fn reserve(&mut self, size: usize) {
        self.data.reserve(size);
    }

    fn push(&mut self) {
        self.data.push(self.generator.rand());
    }
}

#[derive(Debug)]
struct RandMoments {
    mean : f64,
    variance : f64,
    skewness : f64,
    kurtosis : f64
}

fn calculate_moments(data: &[f64]) -> RandMoments {
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let skewness = data.iter().map(|x| (x - mean).powi(3)).sum::<f64>() / (n * variance.powf(1.5));
    let kurtosis =
        data.iter().map(|x| (x - mean).powi(4)).sum::<f64>() / (n * variance.powi(2)) - 3.0;

    RandMoments {mean, variance, skewness, kurtosis}
}

fn test_moments(a : &RandMoments, b : &RandMoments, significance : f64) -> bool {
    (a.mean - b.mean).abs() <= significance
        && (a.variance - b.variance).abs() <= significance
        && (a.skewness - b.skewness).abs() <= significance
        && (a.kurtosis - b.kurtosis).abs() <= significance
}

fn main() {
    const A1: u32 = 564853681;
    const C1: u32 = 790941697;
    const A2: u32 = 10449689;
    const C2: u32 = 176234371;
    const K: usize = 64;

    let mut rng1 = MultiplicativeCongruentialMethod::new(A1, C1);
    let mut rng2 = McLarenMarsagliaMethod::new(
        Box::new(MultiplicativeCongruentialMethod::new(A1, C1)),
        Box::new(MultiplicativeCongruentialMethod::new(A2, C2)),
        K,
    );

    let mut random_set1 = RandSet::new(Box::new(rng1));
    let mut random_set2 = RandSet::new(Box::new(rng2));

    random_set1.reserve(1000);
    random_set2.reserve(1000);

    random_set1.push();
    random_set2.push();

    println!(
        "First random number from MultiplicativeCongruentialMethod: {}",
        random_set1.data[0]
    );
    println!(
        "First random number from McLarenMarsagliaMethod: {}",
        random_set2.data[0]
    );

    for _ in 1..15 {
        random_set1.push();
        random_set2.push();
    }

    println!(
        "15th random number from MultiplicativeCongruentialMethod: {}",
        random_set1.data[14]
    );
    println!(
        "15th random number from McLarenMarsagliaMethod: {}",
        random_set2.data[14]
    );

    for _ in 15..1000 {
        random_set1.push();
        random_set2.push();
    }

    println!(
        "Last (1000th) random number from MultiplicativeCongruentialMethod: {}",
        random_set1.data[999]
    );
    println!(
        "Last (1000th) random number from McLarenMarsagliaMethod: {}",
        random_set2.data[999]
    );

    let tested_moments1 = calculate_moments(&random_set1.data);
    let tested_moments2 = calculate_moments(&random_set2.data);
    const theorethical_moments : RandMoments = RandMoments {
        mean: 0.5,
        variance: 0.0833,
        skewness: 0.0,
        kurtosis: -1.2
    };

    const significance1 : f64 = 0.05;

    println!(
        "Moments test for MultiplicativeCongruentialMethod: {:?} vs {:?}. Test completed: {}",
        tested_moments1, theorethical_moments, test_moments(&tested_moments1, &theorethical_moments, significance1)
    );

    println!(
        "Moments test for McLarenMarsagliaMethod: {:?} vs {:?}. Test completed: {}",
        tested_moments2, theorethical_moments, test_moments(&tested_moments1, &theorethical_moments, significance1)
    );
}
