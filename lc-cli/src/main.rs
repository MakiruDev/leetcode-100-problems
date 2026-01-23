use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "lc", version, about = "LeetCode scaffolding helper")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new problem scaffold
    New {
        /// problem number used in src/main.rs match, e.g. 5
        problem_no: String,
        /// folder/module name, e.g. max_consecutive_ones_iii
        problem: String,
        /// function name, e.g. longest_ones
        func: String,
        /// sample input (string)
        input: String,
        /// sample output (string)
        output: String,
        /// root of your leetcode repo (default: current dir)
        #[arg(long)]
        root: Option<PathBuf>,
    },

    /// Add next way file (copy latest wayN.rs -> wayN+1.rs)
    AddWay {
        /// problem number used in src/main.rs match, e.g. 5
        problem_no: String,
        /// root of your leetcode repo (default: current dir)
        #[arg(long)]
        root: Option<PathBuf>,
    },
}

#[derive(Serialize, Deserialize)]
struct Meta {
    count_way: i32,
    problem: String,
    func: String,
    sample_input: String,
    sample_output: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::New {
            problem_no,
            problem,
            func,
            input,
            output,
            root,
        } => {
            let root = root.unwrap_or(std::env::current_dir()?);

            let problems_dir = root.join("src").join("problems");
            let problem_dir = problems_dir.join(&problem);

            // create src/problems/<problem>
            create_dir_all(&problem_dir)?;

            // 0) update src/problems/catalog.json  (problem_no -> problem module name)
            let mut idx = load_catalog(&problems_dir)?;
            idx.insert(
                problem_no.clone(),
                CatalogEntry {
                    module: problem.clone(),
                    func: func.clone(),
                },
            );
            save_catalog(&problems_dir, &idx)?;

            // 1) create src/problems/<problem>/mod.rs
            let inner_mod = problem_dir.join("mod.rs");
            write_file_if_missing(&inner_mod, format!("pub mod way1;\n"))?;

            // 2) create src/problems/<problem>/way1.rs
            let way1 = problem_dir.join("way1.rs");
            let input_arr: Vec<&str> = input.split_whitespace().collect();
            let input_len = input_arr.len();
            write_file_if_missing(&way1, template_way1(&func, &input, input_len, &output))?;

            // 3) create meta.json
            let meta = Meta {
                count_way: 1,
                problem: problem.clone(),
                func: func.clone(),
                sample_input: input,
                sample_output: output,
            };
            let meta_path = problem_dir.join("meta.json");
            fs::write(meta_path, serde_json::to_string_pretty(&meta).unwrap())?;

            // 4) update src/problems/mod.rs
            let problems_mod = problems_dir.join("mod.rs");
            ensure_pub_mod(&problems_mod, &problem)?;

            let main_rs = root.join("src").join("main.rs");
            ensure_main_match_arm(&main_rs, &problem_no, &problem, "way1")?;

            println!("Created scaffold for: {problem}");
            println!("{}", problem_dir.display());
        }

        Commands::AddWay { problem_no, root } => {
            let root = root.unwrap_or(std::env::current_dir()?);
            let problems_dir = root.join("src").join("problems");

            let idx = load_catalog(&problems_dir)?;
            let module = idx
                .get(&problem_no)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("problem_no {problem_no} not found in src/problems/catalog.json"),
                    )
                })?
                .module
                .clone();

            let problem_dir = problems_dir.join(&module);

            // read meta.json
            let meta_path = problem_dir.join("meta.json");
            let meta_str = fs::read_to_string(&meta_path)?;
            let mut meta: Meta = serde_json::from_str(&meta_str)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

            let last = meta.count_way;
            if last < 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "count_way < 1"));
            }
            let next = last + 1;

            // copy way{last}.rs -> way{next}.rs
            let src_way = problem_dir.join(format!("way{}.rs", last));
            let dst_way = problem_dir.join(format!("way{}.rs", next));

            if !src_way.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("source way file not found: {}", src_way.display()),
                ));
            }
            if dst_way.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("target way already exists: {}", dst_way.display()),
                ));
            }
            fs::copy(&src_way, &dst_way)?;

            // update inner mod.rs  (src/problems/<module>/mod.rs)
            let inner_mod = problem_dir.join("mod.rs");
            ensure_pub_mod(&inner_mod, &format!("way{}", next))?;

            // update main.rs match arm  ("no", "way{next}") => problems::<module>::way{next}::solve
            let main_rs = root.join("src").join("main.rs");
            ensure_main_match_arm(&main_rs, &problem_no, &module, &format!("way{}", next))?;

            // bump meta.count_way
            meta.count_way = next;
            fs::write(meta_path, serde_json::to_string_pretty(&meta).unwrap())?;

            println!("Added way{} for problem {} ({})", next, problem_no, module);
        }
    }

    Ok(())
}

fn create_dir_all(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn write_file_if_missing(path: &Path, content: String) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    fs::write(path, content)?;
    Ok(())
}

fn ensure_pub_mod(mod_rs: &Path, module_name: &str) -> io::Result<()> {
    let line = format!("pub mod {};\n", module_name);

    if !mod_rs.exists() {
        fs::write(mod_rs, line)?;
        return Ok(());
    }

    let mut current = fs::read_to_string(mod_rs)?;
    if !current.contains(&line) {
        if !current.ends_with('\n') {
            current.push('\n');
        }
        current.push_str(&line);
        fs::write(mod_rs, current)?;
    }
    Ok(())
}

fn template_way1(func: &str, input_raw: &str, input_len: usize, output: &str) -> String {
    format!(
        r#"use crate::problems::func_run;

pub struct Solution;

impl Solution {{
    pub fn {func_}() {{
        // pass
    }}
}}

pub fn solve(input_str: &str) {{
    if input_str == "input-format" {{
        println!("{input_len_} input -> {input_r}");
        println!("1 output -> {out_}");
        println!("example : {input_r} => {out_}");
        return;
    }}

    // let param1 = ...;
    // let param2 = ...;

    // let output = Solution::{func}(...);

    // func_run::solve_log("{func}", input_str, output);
}}
"#,
        func_ = func,
        input_r = input_raw,
        input_len_ = input_len,
        out_ = output,
    )
}

fn ensure_main_match_arm(
    main_rs: &Path,
    problem_no: &str,
    module_name: &str,
    way: &str, // "way1"
) -> io::Result<()> {
    let arm = format!(
        r#"        // problems {no}
        ("{no}", "{way}") => problems::{module}::{way}::solve(&input_str),

"#,
        no = problem_no,
        module = module_name,
        way = way,
    );

    if !main_rs.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("main.rs not found at {}", main_rs.display()),
        ));
    }

    let mut current = fs::read_to_string(main_rs)?;

    let needle = format!(
        r#"("{no}", "{way}") => problems::{module}::{way}::solve"#,
        no = problem_no,
        module = module_name,
        way = way,
    );
    if current.contains(&needle) {
        return Ok(());
    }

    let marker = "        // Not found problem";
    if let Some(idx) = current.find(marker) {
        current.insert_str(idx, &arm);
        fs::write(main_rs, current)?;
        return Ok(());
    }

    let fallback = "        _ => eprintln!(\"Not found:";
    if let Some(idx) = current.find(fallback) {
        current.insert_str(idx, &arm);
        fs::write(main_rs, current)?;
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Could not find insertion point in main.rs (marker not found)",
    ))
}

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone)]
struct CatalogEntry {
    module: String,
    func: String,
}

type Catalog = BTreeMap<String, CatalogEntry>;

fn catalog_path(problems_dir: &Path) -> PathBuf {
    problems_dir.join("catalog.json")
}

fn load_catalog(problems_dir: &Path) -> io::Result<Catalog> {
    let p = catalog_path(problems_dir);
    if !p.exists() {
        return Ok(BTreeMap::new());
    }
    let s = fs::read_to_string(p)?;
    let map: Catalog = serde_json::from_str(&s)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(map)
}

fn save_catalog(problems_dir: &Path, map: &Catalog) -> io::Result<()> {
    let p = catalog_path(problems_dir);
    fs::write(p, serde_json::to_string_pretty(map).unwrap())?;
    Ok(())
}
