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

    println!("First random number from MultiplicativeCongruentialMethod: {}", rng1.rand());
    println!("First random number from McLarenMarsagliaMethod: {}", rng2.rand());

    for _ in 1..14 {
        rng1.rand();
        rng2.rand();
    }

    println!("15th random number from MultiplicativeCongruentialMethod: {}", rng1.rand());
    println!("15th random number from McLarenMarsagliaMethod: {}", rng2.rand());

    for _ in 15..999 {
        rng1.rand();
        rng2.rand();
    }

    println!("Last (1000th) random number from MultiplicativeCongruentialMethod: {}", rng1.rand());
    println!("Last (1000th) random number from McLarenMarsagliaMethod: {}", rng2.rand());
}
