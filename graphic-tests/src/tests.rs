use std::path::Path;

use chronos::{
    configs::EngineConfig,
    graphic_engine::{ChronosEngine, HeadlessRenderer, RendererType},
};
use image::{ImageBuffer, Rgba};

use crate::{
    args_parser::{Args, GraphicApi}, test_collector::collect_wgpu_tests, workspace::{self, prepare_working_directory}
};

pub(crate) mod basic_2d_geometries;

const RENDER_WIDTH: u32 = 1280;
const RENDER_HEIGHT: u32 = 720;

/// Max allowed difference per color channel (accounts for GPU rounding differences).
const CHANNEL_TOLERANCE: u8 = 2;
/// Max percentage of pixels allowed to exceed the tolerance before the test fails.
const PIXEL_FAIL_THRESHOLD_PCT: f64 = 0.01;

pub fn run(args: &Args) {
    println!("Running graphic tests...");
    prepare_working_directory();
    let mut engine = create_engine(args);

    let mut passed = 0u32;
    let mut failed = 0u32;

    let tests = collect_wgpu_tests();

    for test_scene in tests {
        engine.register_scene(test_scene);
    }

    engine.start().expect("Failed to start engine");

    let test_name = "2d_two_triangles";
    if run_test(test_name, &mut engine) {
        passed += 1;
    } else {
        failed += 1;
    }

     println!("\nTest summary: {passed} passed, {failed} failed");
     if failed > 0 {
         std::process::exit(1);
    }

    let result = run_test("2d_two_triangles", &mut engine);

    println!("\nResults: {result}");
    if !result {
        std::process::exit(1);
    }
}

fn create_engine(args: &Args) -> ChronosEngine {
    let renderer_type = get_render_type(args);
    ChronosEngine::new(EngineConfig {
        renderer_type,
        headless: true,
        ..Default::default()
    })
}

fn get_render_type(args: &Args) -> RendererType {
    match args.graphic_api {
        GraphicApi::Wgpu => RendererType::Wgpu,
    }
}

/// Returns `true` if the test passed.
fn run_test(
    test_name: &str,
    engine: &mut ChronosEngine
) -> bool {
    engine.set_current_scene(test_name);
    let pixels = engine.run_one_frame().expect("Failed to run one frame");

    let golden_path = format!("{}{}.png", workspace::GOLDEN_DIR, test_name);

    let golden_img = image::open(&golden_path)
        .expect("Failed to open golden image")
        .to_rgba8();
    let golden_bytes = golden_img.as_raw();

    if pixels.len() != golden_bytes.len() {
        println!(
            "{test_name} ... FAILED (buffer size mismatch: actual={} vs golden={})",
            pixels.len(),
            golden_bytes.len()
        );
        return false;
    }

    let diff = compute_diff(
        &pixels,
        golden_bytes,
        RENDER_WIDTH,
        RENDER_HEIGHT,
        CHANNEL_TOLERANCE,
    );
    let total = (RENDER_WIDTH * RENDER_HEIGHT) as usize;
    let fail_pct = diff.fail_count as f64 / total as f64 * 100.0;

    if diff.fail_count == 0 {
        if diff.within_tolerance_count > 0 {
            println!(
                "{test_name} ... PASSED ({} pixels within tolerance ±{}, max delta: {})",
                diff.within_tolerance_count, CHANNEL_TOLERANCE, diff.max_delta
            );
        } else {
            println!("{test_name} ... PASSED");
        }
        true
    } else if fail_pct <= PIXEL_FAIL_THRESHOLD_PCT {
        println!(
            "{test_name} ... PASSED ({} pixels beyond tolerance, {:.4}% <= threshold {:.2}%, max delta: {})",
            diff.fail_count, fail_pct, PIXEL_FAIL_THRESHOLD_PCT, diff.max_delta
        );
        true
    } else {
        println!(
            "{test_name} ... FAILED ({} pixels beyond tolerance ±{}, {:.2}%, max channel delta: {})",
            diff.fail_count, CHANNEL_TOLERANCE, fail_pct, diff.max_delta
        );

        let actual_path = format!(
            "{}{}_actual.png",
            workspace::TEST_RESULTS_DIR,
            test_name
        );
        let diff_path = format!(
            "{}{}_diff.png",
            workspace::TEST_RESULTS_DIR,
            test_name
        );
        let diff_path = format!(
            "{}{}_diff.png",
            workspace::TEST_RESULTS_DIR,
            test_name
        );
        save_image(&actual_path, RENDER_WIDTH, RENDER_HEIGHT, &pixels);
        save_image(&diff_path, RENDER_WIDTH, RENDER_HEIGHT, &diff.diff_image);
        println!("    -> Actual: {actual_path}");
        println!("    -> Diff:   {diff_path}");
        false
    }
}

struct DiffResult {
    /// Pixels that differ beyond the tolerance threshold.
    fail_count: usize,
    /// Pixels that differ but are within the tolerance threshold.
    within_tolerance_count: usize,
    max_delta: u8,
    diff_image: Vec<u8>,
}

fn compute_diff(
    actual: &[u8],
    golden: &[u8],
    width: u32,
    height: u32,
    tolerance: u8,
) -> DiffResult {
    let pixel_count = (width * height) as usize;
    let mut diff_image = vec![0u8; pixel_count * 4];
    let mut fail_count = 0usize;
    let mut within_tolerance_count = 0usize;
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
            within_tolerance_count += 1;
            max_delta = max_delta.max(pixel_max);
            diff_image[base] = 255;
            diff_image[base + 1] = 255;
            diff_image[base + 2] = 0;
            diff_image[base + 3] = 255;
        } else {
            // Beyond tolerance - show red
            fail_count += 1;
            max_delta = max_delta.max(pixel_max);
            diff_image[base] = 255;
            diff_image[base + 1] = 0;
            diff_image[base + 2] = 0;
            diff_image[base + 3] = 255;
        }
    }

    DiffResult {
        fail_count,
        within_tolerance_count,
        max_delta,
        diff_image,
    }
}

fn save_image(path: &str, width: u32, height: u32, rgba_data: &[u8]) {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, rgba_data.to_vec())
        .expect("Failed to create image buffer");
    img.save(path).expect("Failed to save image");
}
