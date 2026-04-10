use colored::Colorize;

use crate::{
    image_utils::{TestResult, TestResultKind},
    tests::{CHANNEL_TOLERANCE, PIXEL_FAIL_THRESHOLD_PCT, RunResults},
};

pub fn print_result(test_name: &str, result: &TestResult) {
    match &result.kind {
        TestResultKind::Identical => {
            println!("{} ... {}", test_name, "PASSED".green());
        }
        TestResultKind::PassedWithinTolerance {
            pixel_count,
            max_delta,
        } => {
            println!(
                "{} ... {} ({} pixels within tolerance ±{}, max delta: {})",
                test_name,
                "PASSED".yellow(),
                pixel_count,
                CHANNEL_TOLERANCE,
                max_delta,
            );
        }
        TestResultKind::PassedBelowThreshold {
            fail_pct,
            pixel_count,
            max_delta,
        } => {
            println!(
                "{} ... {} ({} pixels beyond tolerance, {:.4}% <= threshold {:.2}%, max delta: {})",
                test_name,
                "PASSED".yellow(),
                pixel_count,
                fail_pct,
                PIXEL_FAIL_THRESHOLD_PCT,
                max_delta,
            );
        }
        TestResultKind::Failed {
            fail_pct,
            pixel_count,
            max_delta,
        } => {
            println!(
                "{} ... {} ({} pixels beyond tolerance ±{}, {:.2}%, max channel delta: {})",
                test_name,
                "FAILED".red(),
                pixel_count,
                CHANNEL_TOLERANCE,
                fail_pct,
                max_delta,
            );
        }
        TestResultKind::BufferSizeMismatch { actual, golden } => {
            println!(
                "{} ... {} (buffer size mismatch: actual={} vs golden={})",
                test_name,
                "FAILED".red(),
                actual,
                golden,
            );
        }
        TestResultKind::Error(err) => {
            println!("{} ... {} (error: {})", test_name, "FAILED".red(), err,);
        }
    }
}

pub fn print_summary(results: RunResults) {
    let total = results.passed + results.failed_names.len();
    println!();
    println!("Results: {}/{} passed", results.passed, total);
    if !results.failed_names.is_empty() {
        println!("Failed tests:");
        for name in results.failed_names {
            println!("  {} {}", "FAILED".red(), name);
        }
    }
}
