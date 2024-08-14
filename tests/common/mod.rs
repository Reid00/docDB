use docdb::SerializationMethod;
use std::fs;
use std::path::Path;

pub struct TestResources {
    ///
    file: String,
}

impl TestResources {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
        }
    }
}

impl Drop for TestResources {
    fn drop(&mut self) {
        let path = Path::new(&self.file);
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }
}

#[macro_export]
macro_rules! set_test_src {
    ($filename:expr) => {
        let _test_src = crate::common::TestResources::new($filename);
    };
}

#[macro_export]
macro_rules! ser_method {
    ($ser_method_int:expr) => {
        SerializationMethod::from($ser_method_int)
    };
}

#[macro_export]
macro_rules! test_setup {
    ($function_name:expr, $ser_method_int:expr, $db_name:ident) => {
        let $db_name = format!(
            "{}_{}.db",
            $function_name,
            ser_method!($ser_method_int).to_string()
        );
        set_test_src!(&$db_name);
    };
}
