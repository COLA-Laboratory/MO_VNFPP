use std::{ops::Range, path::PathBuf};

use clap::{load_yaml, App};

pub struct Args {
    pub test_id: Option<usize>,
    pub run_range: Range<usize>,
    pub test_sizes: Vec<usize>,
    pub output_folder: PathBuf,
    pub max_evaluations: usize,
    pub num_runs: usize,
    pub model: ModelArgs,
    pub problem: ProblemArgs,
    pub moea: MoeaArgs,
}

impl Args {
    pub fn run_range(&self) -> Range<usize> {
        self.run_range.clone()
    }
}

pub struct ModelArgs {
    pub accuracy: f64,
    pub converged_iterations: usize,
    pub port_sr: f64,
    pub port_ql: usize,
    pub active_cost: f64,
    pub idle_cost: f64,
    pub vms_per_server: usize,
}

pub struct ProblemArgs {
    pub mean_service_len: usize,
    pub variance_service_len: usize,
    pub max_service_len: usize,
    pub min_service_len: usize,
    pub mean_prod_rate: f64,
    pub variance_prod_rate: f64,
    pub min_prod_rate: f64,
    pub mean_service_rate: f64,
    pub variance_service_rate: f64,
    pub min_service_rate: f64,
    pub vnf_queue_length: usize,
    pub utilisation: f64,
}

pub struct MoeaArgs {
    pub pop_size: usize,
    pub pc: f64,
}

pub fn get_args() -> Args {
    // Config file where default information is stored
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Config")).unwrap();

    // CLI for overrides
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // ---- General ----
    let test_id: Option<usize> = match matches.value_of("test_id") {
        Some(test_id) => Some(test_id.parse().unwrap()),
        None => None,
    };

    let split_part: usize = match matches.value_of("split") {
        Some(evals) => evals.parse().unwrap(),
        None => 0,
    };

    let num_split: usize = settings.get("num_splits").unwrap();

    let test_size_id: usize = match matches.value_of("test_sizes") {
        Some(evals) => evals.parse().unwrap(),
        None => settings.get("test_sizes").unwrap(),
    };

    let test_sizes: Vec<usize> = match test_size_id {
        1 => vec![12, 16, 20],
        2 => vec![24, 28],
        3 => vec![32],
        _ => (12..=32).step_by(4).collect(),
    };

    let output_folder: String = match matches.value_of("folder") {
        Some(folder) => folder.to_string(),
        None => settings.get("results_folder").unwrap(),
    };
    let output_folder: PathBuf = [output_folder].iter().collect();

    let max_evaluations: usize = match matches.value_of("evaluations") {
        Some(evals) => evals.parse().unwrap(),
        None => settings.get("num_evaluations").unwrap(),
    };

    let num_runs: usize = match matches.value_of("num_runs") {
        Some(evals) => evals.parse().unwrap(),
        None => settings.get("num_runs").unwrap(),
    };

    // --- Model ---
    let accuracy = settings.get("accuracy").unwrap();
    let converged_iterations = settings.get("converged_iterations").unwrap();
    let port_sr = settings.get("port_sr").unwrap();
    let port_ql = settings.get("port_ql").unwrap();
    let active_cost = settings.get("active_cost").unwrap();
    let idle_cost = settings.get("idle_cost").unwrap();
    let vms_per_server = settings.get("vms_per_server").unwrap();

    let model = ModelArgs {
        accuracy,
        converged_iterations,
        port_sr,
        port_ql,
        active_cost,
        idle_cost,
        vms_per_server,
    };

    // --- MOEA ---
    let pc = settings.get("pc").unwrap();
    let pop_size = settings.get("pop_size").unwrap();

    let moea = MoeaArgs { pc, pop_size };

    // --- Problem ---
    let mean_service_len = settings.get("mean_service_len").unwrap();
    let variance_service_len = settings.get("variance_service_len").unwrap();
    let max_service_len = settings.get("max_service_len").unwrap();
    let min_service_len = settings.get("min_service_len").unwrap();

    let mean_prod_rate = settings.get("mean_prod_rate").unwrap();
    let variance_prod_rate = settings.get("variance_prod_rate").unwrap();
    let min_prod_rate = settings.get("min_prod_rate").unwrap();

    let mean_service_rate = settings.get("mean_service_rate").unwrap();
    let variance_service_rate = settings.get("variance_service_rate").unwrap();
    let min_service_rate = settings.get("min_service_rate").unwrap();

    let vnf_queue_length = settings.get("vnf_queue_length").unwrap();

    let utilisation = settings.get("utilisation").unwrap();

    let problem = ProblemArgs {
        mean_service_len,
        variance_service_len,
        max_service_len,
        min_service_len,
        mean_prod_rate,
        variance_prod_rate,
        min_prod_rate,
        mean_service_rate,
        variance_service_rate,
        min_service_rate,
        vnf_queue_length,
        utilisation,
    };

    let split_len = num_runs / num_split;
    let min_split = split_part * split_len;
    let max_split = min_split + split_len;

    let run_range = min_split..max_split;

    Args {
        test_id,
        run_range,
        test_sizes,
        output_folder,
        max_evaluations,
        num_runs,
        model,
        moea,
        problem,
    }
}
