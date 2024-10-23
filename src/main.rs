use std::fs::OpenOptions;
use std::io::Read;
use std::{fs, io};
use yara_x;

struct Scandir {
    dir_to_scan: &'static str,
    yara_rule_path: &'static str,
}

impl Scandir {
    fn new(folder_to_scan: &'static str, rule_for_scan: &'static str) -> Scandir {
        Scandir {
            dir_to_scan: folder_to_scan,
            yara_rule_path: rule_for_scan,
        }
    }

    fn scanning(&self) -> io::Result<()> {
        let mut compiler = yara_x::Compiler::new();
        let mut sourceofyara = OpenOptions::new()
            .read(true)
            .open(self.yara_rule_path)
            .expect("Unable to open yara rule");

        let mut buffer = Vec::new();
        let _ = sourceofyara.read_to_end(&mut buffer);

        let yara_content: &[u8] = &buffer;

        compiler.add_source(yara_content).unwrap();

        let rules = compiler.build();

        let entries = fs::read_dir(self.dir_to_scan)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        for files in &entries {
            // println!("Path for file: {}", files.display());
            let mut scanner = yara_x::Scanner::new(&rules);
            let scan_results = scanner.scan_file(files).unwrap();
            let matching_rule = scan_results.matching_rules();
            let m = matching_rule.len();
            if m == 1 {
                println!("Virus found: {}", files.display());
            }
        }

        Ok(())
    }
}

fn main() {
    let test = Scandir::new(
        "/home/ambesh/personal/rustySaber/scantestdir/",
        "/home/ambesh/personal/rustySaber/test.yar",
    );

    let _ = test.scanning();
}
