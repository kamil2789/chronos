use chronos::{
    configs::EngineConfig,
    graphic_engine::{ChronosEngine, RendererType},
    scene::Scene,
};

use crate::{
    args_parser::{Args, GraphicApi},
    image_utils::{
        TestResult, check_buffer_size, compute_diff, compute_pass, get_golden_image_bytes,
        save_result_images,
    },
    reporter,
    test_collector::collect_wgpu_tests,
    workspace::prepare_working_directory,
};

pub(crate) mod basic_2d_geometries;

pub const RENDER_WIDTH: u32 = 1280;
pub const RENDER_HEIGHT: u32 = 720;
pub const CHANNEL_TOLERANCE: u8 = 2;
pub const PIXEL_FAIL_THRESHOLD_PCT: f64 = 0.01;

pub fn run(args: &Args) {
    println!("Running graphic tests...");
    prepare_working_directory();
    let mut engine = create_engine(args);

    let all_tests = collect_wgpu_tests();
    let selected_tests = select_tests(&args.test_name, all_tests);
    let test_names: Vec<String> = selected_tests.iter().map(|s| s.name.clone()).collect();
    register_scenes(&mut engine, selected_tests);

    engine.start().expect("Failed to start engine");

    let test_name_refs: Vec<&str> = test_names.iter().map(String::as_str).collect();
    let results = run_tests(&test_name_refs, &mut engine);
    reporter::print_summary(results.passed, &results.failed_names);

    if !results.failed_names.is_empty() {
        std::process::exit(1);
    }
}

fn select_tests(test_name: &str, all_tests: Vec<Scene>) -> Vec<Scene> {
    if test_name == "All" {
        all_tests
    } else {
        all_tests
            .into_iter()
            .filter(|s| s.name == test_name)
            .collect()
    }
}

struct RunResults {
    passed: usize,
    failed_names: Vec<String>,
}

fn run_tests(test_names: &[&str], engine: &mut ChronosEngine) -> RunResults {
    let mut passed = 0;
    let mut failed_names = Vec::new();

    for &name in test_names {
        let result = run_test(name, engine);
        if result.passed {
            passed += 1;
        } else {
            failed_names.push(name.to_string());
        }
    }

    RunResults {
        passed,
        failed_names,
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

fn register_scenes(engine: &mut ChronosEngine, scenes: Vec<Scene>) {
    for test_scene in scenes {
        engine.register_scene(test_scene);
    }
}

fn run_test(test_name: &str, engine: &mut ChronosEngine) -> TestResult {
    engine.set_current_scene(test_name);
    let tested_bytes = engine.run_one_frame().expect("Failed to run one frame");
    let golden_bytes = get_golden_image_bytes(test_name);

    if let Some(result) = check_buffer_size(&tested_bytes, &golden_bytes) {
        reporter::print_result(test_name, &result);
        return result;
    }

    let diff = compute_diff(
        &tested_bytes,
        &golden_bytes,
        RENDER_WIDTH,
        RENDER_HEIGHT,
        CHANNEL_TOLERANCE,
    );

    let result = compute_pass(&diff);
    reporter::print_result(test_name, &result);

    if !result.passed {
        save_result_images(test_name, &tested_bytes, &diff);
    }

    result
}
