pub mod write_csv_file{
    use std::error::Error;

    pub fn get_template_values(content_vector: &String, sprint_id: &str, squad: &str) -> Vec<String>{
        let mut processed_content:Vec<String> = Vec::new();
        let mut item: String = String::new();
        let mut histories: Vec<&str> = content_vector.split("*").collect();
        histories.remove(0);

        for (_, history) in histories.iter().enumerate() {
            let mut subtasks: Vec<&str> = history.split("--").collect();
            let parent = subtasks.first().unwrap().trim().to_string();
            subtasks.remove(0);
            for (_,subtask) in subtasks.iter().enumerate(){
                let issue_vec: Vec<&str> = subtask.split("<").collect();
                let hours = issue_vec.first().unwrap();
                let hour_vec: Vec<&str> = hours.split("|").collect();
                let hour: String = calculate_formatted_hours(hour_vec.last().unwrap());
                let mut description_vec: Vec<&str> = issue_vec.last().unwrap().split("+").collect();
                let issue_type = description_vec.first().unwrap().trim();
                description_vec.remove(0);
                let desc: String = format!("{:?}",description_vec.clone().into_iter().map(|i| i.to_string()).collect::<Vec<_>>()).replace("\"","").replace("[","").replace("]","");

                item = format!("{},{},\"{}\",{},{},{},{},{}",issue_type,hour_vec.first().unwrap(),desc,hour_vec.last().unwrap() ,hour, sprint_id, squad ,parent);
                processed_content.push((*item).parse().unwrap());
            }
        }

        processed_content
    }

    pub fn calculate_formatted_hours(hours: &str) -> String {
        let hour: u32 =  match hours.trim().parse() {
            Err(err) =>{
                println!("File does not follows the template {err}");
                0
            },
            Ok(hours) =>  hours
        };
        let total = hour * 216000;

        total.to_string()
    }

    pub fn write_file(path: &str, content_to_write: Vec<String>) -> Result<(), Box<dyn Error>> {

        let csv_file_path = path.replace(".txt", ".csv");
        let mut writer = csv::Writer::from_path(csv_file_path)?;

        writer.write_record(&["Issue Type","Summary","Description","Hours","Horas(Jira)","Sprint","Team","Parent"])?;

        for content in content_to_write {
            let mut values: Vec<&str> = content.split("\"").collect();
            let description: String = values.iter().nth(1).unwrap().to_string();
            values.remove(1);

            let row_start = values.iter().nth(0).unwrap().to_string();
            let row_end = values.iter().nth(1).unwrap().to_string();

            let issue_type: String = row_start.split(",").nth(0).unwrap().to_string();
            let summary: String = row_start.split(",").nth(1).unwrap().to_string();

            let hours: String = row_end.split(",").nth(1).unwrap().to_string();
            let hours_jira: String = row_end.split(",").nth(2).unwrap().to_string();
            let sprint: String = row_end.split(",").nth(3).unwrap().to_string();
            let team: String = row_end.split(",").nth(4).unwrap().to_string();
            let parent: String = row_end.split(",").nth(5).unwrap().to_string();

            writer.write_record(&[issue_type, summary, description, hours, hours_jira, sprint, team, parent])?;
        }
        writer.flush()?;

        Ok(())
    }

    #[cfg(test)]
    mod tests{
        use std::fs;
        use std::path::Path;
        use crate::file_handler::write_csv_file::write_csv_file::{calculate_formatted_hours, get_template_values, write_file};

        #[test]
        fn should_get_template_values(){
            let value: String = "*TEST--Test task | 1 < Sub-Imp+ Test description".to_string();
            let result: Vec<String> = get_template_values(&value, "1","test_squad");

            assert_eq!(1, result.len());
        }

        #[test]
        fn should_write_file(){
            let value: String = "*TEST--Test task | 1 < Sub-Imp+ Test description".to_string();
            let content: Vec<String> = get_template_values(&value, "1","test_squad");

            let result = write_file("resource/file.txt", content);

            assert_eq!(true, result.ok().is_some());
            assert_eq!(true, Path::new("resource/file.csv").exists());

            let path: String = String::from("resource/file.csv");
            fs::remove_file(path).expect("No file found in path {path}");

            assert_eq!(false, Path::new("resource/file.csv").exists());
        }

        #[test]
        fn should_calculate_hours_from_given_value(){
            let hour: &str = "1";

            let result = calculate_formatted_hours(hour);

            assert_eq!("216000", result)
        }
    }
}