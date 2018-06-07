use std::io::{BufReader, BufRead, Write};
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::env;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    query(&out_dir);
    doc(&out_dir);
}

fn query(out_dir: &Path) {
    let dest_path = out_dir.join("query.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "/// Query to run to set up the database.").unwrap();
    writeln!(f, "pub const INITIALISE_DATABASE: &str = r#####\"").unwrap();
    for doc_f in doc_files() {
        println!("cargo:rerun-if-changed={}", doc_f.display());

        let mut copying = false;
        let mut no_run = false;
        for l in BufReader::new(File::open(doc_f).unwrap()).lines().map(Result::unwrap) {
            if l == "<!-- no_run -->" {
                no_run = true;
            } else if l == "```sql" {
                copying = !no_run;
                no_run = false;
            } else if l == "```" {
                copying = false;
                writeln!(f).unwrap();
            } else if copying {
                f.write_all(l.as_bytes()).unwrap();
                writeln!(f).unwrap();
            }
        }
        writeln!(f).unwrap();
    }
    writeln!(f, "\"#####;").unwrap();
    writeln!(f).unwrap();
}

fn doc(out_dir: &Path) {
    let dest_path = out_dir.join("doc.rs");
    let mut f = File::create(&dest_path).unwrap();

    for doc_f in doc_files() {
        println!("cargo:rerun-if-changed={}", doc_f.display());

        writeln!(f, "pub mod {} {{", doc_f.file_stem().unwrap().to_str().unwrap()).unwrap();

        for l in BufReader::new(File::open(doc_f).unwrap()).lines().map(Result::unwrap) {
            write!(f, "    //! ").unwrap();
            f.write_all(l.as_bytes()).unwrap();
            writeln!(f).unwrap();
        }
        f.write_all("\n".as_bytes()).unwrap();

        writeln!(f, "}}").unwrap();
        writeln!(f).unwrap();
    }
}

fn doc_files() -> Vec<PathBuf> {
    fs::read_dir("doc").unwrap().map(Result::unwrap).map(|de| de.path()).filter(|p| p.extension().is_some() && p.extension().unwrap() == "md").collect()
}
