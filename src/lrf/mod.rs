
#[derive(Debug)]
pub struct Dimension {
    limits: Box<[usize]>,
    mults: Box<[usize]>
}

impl Dimension {
    pub fn new(limits: Box<[usize]>) -> Self {
        let mut mults = Vec::with_capacity(limits.len()+1);
        mults.push(1);

        for limit in &(*limits) {
            debug_assert!(limit > &1usize);
            let mult: usize = limit * mults[mults.len() - 1];
            mults.push(mult);
        }

        Dimension {
            limits: limits,
            mults: mults.into_boxed_slice()
        }
    }
}