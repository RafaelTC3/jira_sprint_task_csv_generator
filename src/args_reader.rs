pub mod args_reader{

    pub fn read_args(args: &[String]) -> (&str, &str, &str) {
        let file_path: &String = &args[2];
        let sprint_id: &String = &args[3];
        let squad: &String = &args[4];

        (file_path, sprint_id, squad)
    }

    #[cfg(test)]

    mod tests{
        use super::read_args;

        #[test]
        fn read_args_test(){
            let mut args: Vec<String> = Vec::new();
            args.push("application Name".parse().unwrap());
            args.push("action".parse().unwrap());
            args.push("test".parse().unwrap());
            args.push("1".parse().unwrap());
            args.push("sq".parse().unwrap());
            let (file_path, sprint_id, squad) = read_args(&args);
            assert_eq!(file_path, "test");
            assert_eq!(sprint_id, "1");
            assert_eq!(sprint_id.trim().parse::<u32>().unwrap(), 1);
            assert_eq!(squad, "sq");
        }
    }
}