pub mod write_template_file{
    use std::fs::File;
    use std::io::Write;

    pub fn write_template_file(file_path: &str) -> std::io::Result<()> {
        let path= String::from(file_path) + "/template.txt";
        let mut file = File::create(path)?;
        let content: &str = "//* Represents History
//-- Represents subtasks
//| Represents the estimated hours to resolve the task
//< Represents the type of the Task
//Do not use double quotes \" inside the file
*history number
--Task Summary | 10 < Sub-Imp
\t+ comment for the task
\t+ Can have multiple comments
\t+ comment with example on ho to write 'specific information' without double quotes";
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[cfg(test)]

    mod tests{
        use std::fs;
        use std::path::Path;
        use super::write_template_file;

        #[test]
        fn write_template_file_test(){
            let result = write_template_file("resource");
            assert_eq!(true, Path::new("resource/template.txt").exists());

            let path: String = String::from("resource/template.txt");
            fs::remove_file(path).expect("No file found in path {path}");
            assert_eq!(false, Path::new("resource/template.txt").exists());
        }
    }
}