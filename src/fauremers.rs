pub mod fauremers
{
    use crumsort::ParCrumSort;
    use murmurhash32::murmurhash3;

    pub struct Thresholds {
        pub a: u32,
        pub c: u32,
        pub g: u32,
        pub t: u32,
    }

    impl Thresholds {
        pub fn new(c_ratio: f64) -> Self {
            let t = (std::u32::MAX as f64 * c_ratio).round() as u32;
            let step = (t as f64 / 4.0).round() as u32;

            Self {
                a: t - 3 * step,
                c: t - 2 * step,
                g: t - step,
                t: t,
            }
        }
    }

    pub fn add_fauremers(config: &crate::Config, t: &Thresholds, vec: &mut Vec<u64>, seq: &[u8]) {
        let sketch_mask: u64 = if config.order < 32 {(1 << (2 * config.order)) - 1} else {std::u64::MAX};
        let mut current_sketch: u64 = 0;

        let kmer_mask: u64 = if config.k < 32 {(1 << (2 * config.k)) - 1} else {std::u64::MAX};
        let mut current_kmer: u64 = 0;
        let mut current_kmer_len: usize = 0;

        for i in 0..config.order-1 {
            match encode_base(seq[i]) {
                Some(v) => {
                    current_sketch = (current_sketch << 2) | v;
                    //println!("curr_sketch: {}, [u8]: {:?},  threshold : {}", current_sketch, current_sketch.to_be_bytes(), t.t);
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[i] as char);
                }
            }
        }

        for j in config.order-1..seq.len() {
            match encode_base(seq[j]) {
                Some(v) => {
                    current_sketch = (current_sketch << 2) | v;
                    current_sketch &= sketch_mask;

                    let hash = murmurhash3(&current_sketch.to_be_bytes());
                    if hash < t.t { //means new char in sketch
                        current_kmer = (current_kmer << 2) | sketch_base(hash, t);
                        current_kmer_len += 1;

                        if current_kmer_len >= config.k {
                            vec.push(current_kmer & kmer_mask);
                        }                    
                    }
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[j] as char);
                }
            }
        }

        vec.par_crumsort();
        vec.dedup();
    }

    pub fn query_fauremers(config: &crate::Config, t: &Thresholds, index: &Vec<u64>, seq: &[u8]) -> f64{
        if seq.len() < config.order {
            panic!("Sequence must be at least <order> long");
        }
        let mut nb_found: f64 = 0.0;
        let mut nb_kmers: f64 = 0.0;

        let sketch_mask: u64 = if config.order < 32 {(1 << (2 * config.order)) - 1} else {std::u64::MAX};
        let mut current_sketch: u64 = 0;

        let kmer_mask: u64 = if config.k < 32 {(1 << (2 * config.k)) - 1} else {std::u64::MAX};
        let mut current_kmer: u64 = 0;
        let mut current_kmer_len: usize = 0;

        for i in 0..config.order-1 {
            match encode_base(seq[i]) {
                Some(v) => {
                    current_sketch = (current_sketch << 2) | v;
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[i] as char);
                }
            }
        }

        for j in config.order-1..seq.len() {
            match encode_base(seq[j]) {
                Some(v) => {
                    current_sketch = (current_sketch << 2) | v;
                    current_sketch &= sketch_mask;

                    let hash = murmurhash3(&current_sketch.to_be_bytes());
                    if hash < t.t { //means new char in sketch
                        current_kmer = (current_kmer << 2) | sketch_base(hash, t);
                        current_kmer_len += 1;

                        if current_kmer_len >= config.k {
                            nb_found += match index.binary_search(&(current_kmer & kmer_mask)) {
                                Ok(_) => 1.0,
                                Err(_) => 0.0,
                            };
                            nb_kmers += 1.0;
                        }                    
                    }
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[j] as char);
                }
            }
        }

        nb_found / nb_kmers
    }

    fn encode_base(b: u8) -> Option<u64> {
        match b {
            b'A' => Some(0),
            b'C' => Some(1),
            b'G' => Some(2),
            b'T' => Some(3),
            _ => None, // skip Ns or other ambiguous bases
        }
    }

    fn sketch_base(hash: u32, t: &Thresholds) -> u64 {
        match hash {
            h if h < t.a => 0,
            h if h < t.c => 1,
            h if h < t.g => 2,
            h if h < t.t => 3,
            _ => unreachable!(),
        }
    }
}