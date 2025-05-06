pub struct RandomXorShiftGenerator(u64);

impl RandomXorShiftGenerator {
    pub fn new() -> RandomXorShiftGenerator {
        RandomXorShiftGenerator(187)
    }

    pub fn next(&mut self) -> u64 {
        let mut i = self.0;

        i ^= i << 13;
        i ^= i >> 17;
        i ^= i << 5;

        self.0 = i;
        i
    }
}
