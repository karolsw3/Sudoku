extern crate reqwest;

use std::io::{BufReader, BufRead, Write};
use std::collections::BTreeSet;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::env;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    query(&out_dir);
    doc(&out_dir);
    words(&out_dir);
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

fn words(out_dir: &Path) {
    let dest_path = Path::new(&out_dir).join("words.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all("/// A set of first-upper-case adjectives for order numbers.\n".as_bytes()).unwrap();
    f.write_all("pub static ADJECTIVES: &[&str] = &[\n".as_bytes()).unwrap();
    for adj in first_adjectives().into_iter().chain(second_adjectives().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(adj.as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of first-upper-case nouns for order numbers.\n".as_bytes()).unwrap();
    f.write_all("pub static NOUNS: &[&str] = &[\n".as_bytes()).unwrap();
    for noun in nouns() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(uppercase_first(noun).as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of first-upper-case adverbs for order numbers.\n".as_bytes()).unwrap();
    f.write_all("pub static ADVERBS: &[&str] = &[\n".as_bytes()).unwrap();
    for adv in first_adverbs().into_iter().chain(second_adverbs().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(uppercase_first(adv).as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
}

/// Stolen from https://stackoverflow.com/a/38406885/2851815.
fn uppercase_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn first_adjectives() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://enchantedlearning.com/wordlist/adjectives.shtml")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l == "</td></tr></table>" {
                currently = false;
            }

            if currently && !l.is_empty() {
                let l = l.replace("<br>", "");
                if !l.contains('<') && l.len() > 1 {
                    coll.push(l);
                }
            }

            if l.contains("<font size=+0>a</font>") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn second_adjectives() -> Vec<String> {
    talkenglish("http://www.talkenglish.com/vocabulary/top-1500-nouns.aspx")
}

fn nouns() -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://www.desiquintans.com/downloads/nounlist/nounlist.txt")
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.len() != 1)
        .collect()
}

fn first_adverbs() -> Vec<String> {
    talkenglish("http://www.talkenglish.com/vocabulary/top-250-adverbs.aspx")
}

fn second_adverbs() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("https://www.espressoenglish.net/100-common-english-adverbs/")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l.contains("100.") {
                currently = false;
            }

            if currently && !l.contains("div>") {
                let l = l.replace("<br />", "").replace("<p>", "").replace("</p>", "").replace("/>", "");
                coll.push(l.rsplitn(2, " ").next().unwrap().trim().to_string());
            }

            if l.contains("<p>1.") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn talkenglish(url: &str) -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get(url)
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.contains(r#"<a href="/how-to-use/"#))
        .filter_map(|l| l.replace("</a>", "").replace('>', "\n").split("\n").skip(1).next().map(|s| s.to_string()))
        .skip(1)
        .filter(|l| l.len() != 1)
        .collect()
}

