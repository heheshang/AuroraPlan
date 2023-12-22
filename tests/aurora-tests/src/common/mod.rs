#[cfg(test)]
mod tests {

    #[test]
    fn regex_is_work() {
        let text = "copy process definition from {0} to {2} error : {1}";
        let mut new_text = text.to_string();
        let args = vec![String::from("aaa"), String::from("bb"), String::from("cc")];

        let re = regex::Regex::new(r"\{(\d+)").unwrap();
        // let mut result = String::new();

        for cap in re.captures_iter(text) {
            let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", index);
            if args.len() <= index {
                continue;
            }
            let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
            new_text = ss.clone();
            // println!("{}", ss);
            // result.push_str(&args[index]);
        }

        println!("{}", new_text);
    }

    #[test]
    fn test_snowflake() {
        use snowflake::SnowflakeIdBucket;
        let id = SnowflakeIdBucket::new(1, 1).get_id();
        println!("{}", id);
    }
}
