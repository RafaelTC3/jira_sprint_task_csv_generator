pub mod read_txt_file{
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;

    pub fn read_file(path: &str) -> String {
        let mut content: String = String::new();
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                let line_content: String = line.expect("Error reading line.").trim().to_string();
                if line_content.is_empty() || line_content.starts_with("//") {
                    continue;
                }
                content.push_str(line_content.as_str());
            }
        }
        content
    }

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[cfg(test)]
    mod tests{
        use std::fs::File;
        use std::io;
        use crate::file_handler::read_txt_file::read_txt_file::{read_file, read_lines};

        #[test]
        fn should_read_file(){
            let content = read_file("resource/file.txt");

            assert_eq!(content, "*Trying--to | read < somenthing+ comment");
        }

        #[test]
        fn should_read_lines_from_file(){
            let lines: io::Result<io::Lines<io::BufReader<File>>> = read_lines("resource/file.txt");
            println!("{:?}",lines);
            assert_eq!(true, lines.ok().is_some());
        }
    }
}