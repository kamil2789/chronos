use std::path::Path;

use image::{ImageBuffer, Rgba};

use crate::{
    tests::{PIXEL_FAIL_THRESHOLD_PCT, RENDER_HEIGHT, RENDER_WIDTH},
    workspace,
};

pub struct TestResult {
    pub passed: bool,
    pub kind: TestResultKind,
}

pub enum TestResultKind {
    Identical,
    PassedWithinTolerance {
        pixel_count: usize,
        max_delta: u8,
    },
    PassedBelowThreshold {
        fail_pct: f64,
        pixel_count: usize,
        max_delta: u8,
    },
    Failed {
        fail_pct: f64,
        pixel_count: usize,
        max_delta: u8,
    },
    BufferSizeMismatch {
        actual: usize,
        golden: usize,
    },
}

pub struct DiffResult {
    pub failed_pixel_count: usize,
    pub pixel_within_tolerance_count: usize,
    pub max_delta: u8,
    pub diff_image: Vec<u8>,
}

pub fn compute_diff(
    actual: &[u8],
    golden: &[u8],
    width: u32,
    height: u32,
    tolerance: u8,
) -> DiffResult {
    let pixel_count = (width * height) as usize;
    let mut diff_image = vec![0u8; pixel_count * 4];
    let mut failed_pixel_count = 0usize;
    let mut pixel_within_tolerance_count = 0usize;
    let mut max_delta = 0u8;

    for i in 0..pixel_count {
        let base = i * 4;
        let ar = actual[base];
        let ag = actual[base + 1];
        let ab = actual[base + 2];
        let aa = actual[base + 3];

        let gr = golden[base];
        let gg = golden[base + 1];
        let gb = golden[base + 2];
        let ga = golden[base + 3];

        let dr = ar.abs_diff(gr);
        let dg = ag.abs_diff(gg);
        let db = ab.abs_diff(gb);
        let da = aa.abs_diff(ga);

        let pixel_max = dr.max(dg).max(db).max(da);

        if pixel_max == 0 {
            // Exact match - show dimmed original
            diff_image[base] = ar / 4;
            diff_image[base + 1] = ag / 4;
            diff_image[base + 2] = ab / 4;
            diff_image[base + 3] = 255;
        } else if pixel_max <= tolerance {
            // Within tolerance - show yellow
            pixel_within_tolerance_count += 1;
            max_delta = max_delta.max(pixel_max);
            diff_image[base] = 255;
            diff_image[base + 1] = 255;
            diff_image[base + 2] = 0;
            diff_image[base + 3] = 255;
        } else {
            // Beyond tolerance - show red
            failed_pixel_count += 1;
            max_delta = max_delta.max(pixel_max);
            diff_image[base] = 255;
            diff_image[base + 1] = 0;
            diff_image[base + 2] = 0;
            diff_image[base + 3] = 255;
        }
    }

    DiffResult {
        failed_pixel_count,
        pixel_within_tolerance_count,
        max_delta,
        diff_image,
    }
}

pub fn check_buffer_size(actual: &[u8], golden: &[u8]) -> Option<TestResult> {
    if actual.len() != golden.len() {
        Some(TestResult {
            passed: false,
            kind: TestResultKind::BufferSizeMismatch {
                actual: actual.len(),
                golden: golden.len(),
            },
        })
    } else {
        None
    }
}

/// Evaluates whether a rendered frame passes the visual correctness test.
///
/// The result is determined by two independent tolerance layers:
///
/// 1. **Per-pixel tolerance** (`CHANNEL_TOLERANCE`): a pixel is considered "failed" only if
///    at least one of its RGBA channels differs from the golden image by more than this value.
///    Pixels that differ but stay within this threshold are counted separately as "within tolerance".
///
/// 2. **Per-test threshold** (`PIXEL_FAIL_THRESHOLD_PCT`): even if some pixels exceed the
///    per-pixel tolerance, the test still passes if the percentage of such pixels is below
///    this threshold. This accounts for minor GPU rasterization differences at shape edges.
///
/// # Outcomes
///
/// | Condition                                                    | Result kind              | Passes |
/// |--------------------------------------------------------------|--------------------------|--------|
/// | All pixels identical                                         | `Identical`              | yes    |
/// | No failed pixels, but some within per-pixel tolerance        | `PassedWithinTolerance`  | yes    |
/// | Failed pixels exist, but their % is below the test threshold | `PassedBelowThreshold`   | yes    |
/// | Failed pixels exceed the test threshold                      | `Failed`                 | no     |
pub fn compute_pass(diff: &DiffResult) -> TestResult {
    let total = (RENDER_WIDTH * RENDER_HEIGHT) as usize;
    let fail_pct = diff.failed_pixel_count as f64 / total as f64 * 100.0;

    if diff.failed_pixel_count == 0 {
        if diff.pixel_within_tolerance_count > 0 {
            TestResult {
                passed: true,
                kind: TestResultKind::PassedWithinTolerance {
                    pixel_count: diff.pixel_within_tolerance_count,
                    max_delta: diff.max_delta,
                },
            }
        } else {
            TestResult {
                passed: true,
                kind: TestResultKind::Identical,
            }
        }
    } else if fail_pct <= PIXEL_FAIL_THRESHOLD_PCT {
        TestResult {
            passed: true,
            kind: TestResultKind::PassedBelowThreshold {
                fail_pct,
                pixel_count: diff.failed_pixel_count,
                max_delta: diff.max_delta,
            },
        }
    } else {
        TestResult {
            passed: false,
            kind: TestResultKind::Failed {
                fail_pct,
                pixel_count: diff.failed_pixel_count,
                max_delta: diff.max_delta,
            },
        }
    }
}

pub fn get_golden_image_bytes(test_name: &str) -> Vec<u8> {
    let golden_path = format!("{}{}.png", workspace::GOLDEN_DIR, test_name);
    image::open(&golden_path)
        .expect("Failed to open golden image")
        .to_rgba8()
        .into_raw()
}

pub fn save_result_images(test_name: &str, tested_bytes: &[u8], diff: &DiffResult) {
    let actual_path = format!("{}{}_actual.png", workspace::TEST_RESULTS_DIR, test_name);
    let diff_path = format!("{}{}_diff.png", workspace::TEST_RESULTS_DIR, test_name);

    save_image(&actual_path, RENDER_WIDTH, RENDER_HEIGHT, tested_bytes);
    save_image(&diff_path, RENDER_WIDTH, RENDER_HEIGHT, &diff.diff_image);

    println!("    -> Actual: {actual_path}");
    println!("    -> Diff:   {diff_path}");
}

pub fn save_image(path: &str, width: u32, height: u32, rgba_data: &[u8]) {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, rgba_data.to_vec())
        .expect("Failed to create image buffer");
    img.save(path).expect("Failed to save image");
}
