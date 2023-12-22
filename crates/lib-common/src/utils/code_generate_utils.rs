use snowflake::SnowflakeIdGenerator;
use std::sync::{Arc, Mutex};
pub fn gen_code() -> Option<i64> {
    let instance = get_instance();
    let mut sf = instance.lock().unwrap();

    let a = sf.lazy_generate();
    let numeber_str = a.to_string();
    let sliced_str = &numeber_str[5..];
    let res = sliced_str.parse::<i64>().unwrap_or_default();
    Some(res)
}

fn get_instance() -> Arc<Mutex<SnowflakeIdGenerator>> {
    static mut SINGLETON: Option<Arc<Mutex<SnowflakeIdGenerator>>> = None;

    unsafe {
        SINGLETON
            .get_or_insert_with(|| Arc::new(Mutex::new(SnowflakeIdGenerator::new(1, 1))))
            .clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        for _ in 0..100 {
            let a = super::gen_code().unwrap();
            println!("{}", a);
            let numeber_str = a.to_string();
            let sliced_str = &numeber_str[5..];
            let res = sliced_str.parse::<i64>().unwrap();
            println!("{}", res);
            // 79951478411364
            // 11574588974624
        }
    }
}
