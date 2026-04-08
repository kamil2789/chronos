use std::{env, fs};

pub const TEST_RESULTS_DIR: &str = "test_results/";
pub const GOLDEN_DIR: &str = "src/assets/golden_standart/";
pub const TEST_PROJECT_NAME: &str = "graphic-tests";

pub fn prepare_working_directory() {
    let current_path = env::current_dir().expect("Invalid current directory");
    let dir_name = current_path.file_name().expect("Invalid directory name");
    if dir_name != TEST_PROJECT_NAME {
        env::set_current_dir(TEST_PROJECT_NAME)
            .expect("Failed to set current directory to graphic-tests");
    }

    if fs::metadata(TEST_RESULTS_DIR).is_err() {
        fs::create_dir(TEST_RESULTS_DIR).expect("Cannot create test_results directory");
    }
}
