pub mod kmers{
    use core::panic;

    use crumsort::ParCrumSort;

    pub fn add_kmers(config: &crate::Config, vec: &mut Vec<u64>, seq: &[u8]) {
        let kmer_mask: u64 = if config.k < 32 {(1 << (2 * config.k)) - 1} else {std::u64::MAX};
        let mut current_kmer: u64 = 0;
        let mut current_kmer_len: usize = 0;

        for i in 0..seq.len() {
            match encode_base(seq[i]) {
                Some(v) => {
                    current_kmer = (current_kmer << 2) | v;
                    current_kmer &= kmer_mask;
                    current_kmer_len += 1;

                    if current_kmer_len >= config.k {
                        vec.push(current_kmer);
                    }
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[i] as char);
                }
            }
        }

        vec.par_crumsort();
        vec.dedup();
    }

    pub fn query_kmers(config: &crate::Config, index: &Vec<u64>, seq: &[u8]) -> f64 {
        if seq.len() < config.k {
            panic!("Sequence is too short to contain any k-mers");
        }

        let mut nb_found: f64 = 0.0;
        let mut nb_kmers: f64 = 0.0;

        let kmer_mask: u64 = if config.k < 32 {(1 << (2 * config.k)) - 1} else {std::u64::MAX};
        let mut current_kmer: u64 = 0;
        let mut current_kmer_len: usize = 0;

        for i in 0..seq.len() {
            match encode_base(seq[i]) {
                Some(v) => {
                    current_kmer = (current_kmer << 2) | v;
                    current_kmer &= kmer_mask;
                    current_kmer_len += 1;

                    if current_kmer_len >= config.k {
                        nb_found += match index.binary_search(&current_kmer) {
                            Ok(_) => 1.0,
                            Err(_) => 0.0,
                        };
                        nb_kmers += 1.0;
                    }
                }
                None => {
                    panic!("Invalid base not handled yet: {}", seq[i] as char);
                }
            }
        }

        nb_found / nb_kmers
    }

    fn encode_base(base: u8) -> Option<u64> {
        match base {
            b'A' => Some(0),
            b'C' => Some(1),
            b'G' => Some(2),
            b'T' => Some(3),
            _ => None,
        }
    }
}