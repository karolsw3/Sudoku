extern crate reqwest;

use std::io::{BufReader, BufRead, Write};
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::env;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    query(&out_dir);
}

fn query(out_dir: &Path) {
    let dest_path = Path::new(&out_dir).join("query.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all("/// Query to run to set up the database.\n".as_bytes()).unwrap();
    f.write_all("pub static INITIALISE_DATABASE: &str = r#####\"\n".as_bytes()).unwrap();
    for doc_f in doc_files() {
        let mut copying = false;
        for l in BufReader::new(File::open(doc_f).unwrap()).lines() {
            let l = l.unwrap();
            if l == "```sql" {
                copying = true;
            } else if l == "```" {
                copying = false;
                f.write_all("\n".as_bytes()).unwrap();
            } else if copying {
                f.write_all(l.as_bytes()).unwrap();
                f.write_all("\n".as_bytes()).unwrap();
            }
        }
        f.write_all("\n".as_bytes()).unwrap();
    }
    f.write_all("\"#####;\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
}

fn doc_files() -> Vec<PathBuf> {
    fs::read_dir("doc").unwrap().map(Result::unwrap).map(|de| de.path()).filter(|p| p.extension().is_some() && p.extension().unwrap() == "md").collect()
}
