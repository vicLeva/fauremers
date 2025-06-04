use seq_io::fasta::Reader;
use std::fs::OpenOptions;
use std::io::Write;
pub mod fauremers;
pub mod kmers;
use crate::fauremers::fauremers::{add_fauremers, Thresholds, query_fauremers};
use crate::kmers::kmers::{add_kmers, query_kmers};

const QUERIES_PATH: &str = "/home/vlevallo/documents/rusty/fauremers/queries.fasta";

pub struct Config {
    pub file_path: String,
    pub order: usize,
    pub k: usize,
    pub c_ratio: f64,
}

impl Config{
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path (cargo run -- <file_path> <order> <c_ratio> <k>)"),
        };

        let order = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("Didn't get an order (cargo run -- <file_path> <order> <c_ratio> <k>)"),
        };

        let c_ratio = match args.next() {
            Some(arg) => arg.parse::<f64>().unwrap(),
            None => return Err("Didn't get a compression ratio (cargo run -- <file_path> <order> <c_ratio> <k>)"),
        };

        let k = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("Didn't get a k value (cargo run -- <file_path> <order> <c_ratio> <k>)"),
        };

        Ok(Config {
            file_path,
            order,
            k,
            c_ratio,
        })
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(&config.file_path).unwrap();

    let mut kmers_index: Vec<u64> = vec![];

    let t = Thresholds::new(config.c_ratio);
    let mut fauremers_index: Vec<u64> = vec![];

    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let s = record.full_seq();
        
        //println!("{}", std::str::from_utf8(&s).unwrap());
        add_kmers(&config, &mut kmers_index, &s);
        add_fauremers(&config, &t, &mut fauremers_index, &s);
    }

    let queries: Vec<Vec<u8>> = get_queries(QUERIES_PATH);
    let mut avg_fauremers: f64 = 0.0;
    let mut avg_kmers: f64 = 0.0;
    for query in &queries {
        avg_fauremers += query_fauremers(&config, &t, &fauremers_index, query);
        avg_kmers += query_kmers(&config, &kmers_index, query);
    }
    println!("Average Fauremers ratio: {} ({} queries)", avg_fauremers / queries.len() as f64, queries.len());
    println!("Average Kmers ratio: {} ({} queries)", avg_kmers / queries.len() as f64, queries.len());

    Ok(())
}

pub fn run_expes(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let queries: Vec<Vec<u8>> = get_queries(QUERIES_PATH);

    let mut reader = Reader::from_path(&config.file_path).unwrap();
    
    //KMERS=====================================================================
    //BUILD
    let mut kmers_index: Vec<u64> = vec![];
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let s = record.full_seq();
        
        add_kmers(&config, &mut kmers_index, &s);
    }
    //QUERY
    let mut results: Vec<f64> = vec![];
    for query in &queries {
        results.push(query_kmers(&config, &kmers_index, query));
    }
    write_results(false, &config, kmers_index.len(), results)?;

    
    //FAUREMERS=================================================================
    for order in (5..=50).step_by(5) {
        for c_ratio in (1..=10).map(|i| i as f64 * 0.05) {
            let t = Thresholds::new(config.c_ratio);
            println!("Running Fauremers with order: {}, c_ratio: {}", order, c_ratio);
        

            let config = Config {
                file_path: config.file_path.clone(),
                order,
                k: config.k,
                c_ratio,
            }; 
            
            // BUILD
            let mut fauremers_index: Vec<u64> = vec![];
            // Rewind the reader to the beginning for each order
            let mut reader = Reader::from_path(&config.file_path).unwrap();
            while let Some(record) = reader.next() {
                let record = record.expect("Error reading record");
                let s = record.full_seq();

                add_fauremers(&config, &t, &mut fauremers_index, &s);
            }
            // QUERY
            let mut results: Vec<f64> = vec![];
            for query in &queries {
                results.push(query_fauremers(&config, &t, &fauremers_index, query));
            }

            write_results(true, &config, fauremers_index.len(), results)?;
        }
    }

    Ok(())
}

pub fn get_queries(path: &str) -> Vec<Vec<u8>> {
    let mut queries_reader = Reader::from_path(path).unwrap();
    let mut queries: Vec<Vec<u8>> = vec![];
    while let Some(record) = queries_reader.next() {
        let record = record.expect("Error reading query record");
        let s = record.full_seq().to_vec();
        queries.push(s);
    }
    queries
}

pub fn write_results(
    fauremers: bool,
    config: &Config,
    index_size: usize,
    results: Vec<f64>,
) -> std::io::Result<()> {
    let file_name = format!("results_k{}.txt", config.k);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    if fauremers {
        write!(file, "{},{},{};{};", config.order, config.c_ratio, config.k, index_size)?;
    } else {
        write!(file, "{};{};", config.k, index_size)?;
    }

    for i in results.iter() {
        write!(file, "{},", i)?;
    }
    writeln!(file)?;

    Ok(())
}