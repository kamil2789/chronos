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

#[derive(Default)]
pub struct RunResults {
    pub passed: usize,
    pub failed_names: Vec<String>,
}

pub fn run_framework(args: &Args) {
    prepare_working_directory();
    let mut engine = create_engine(args);

    register_scenes(&mut engine, collect_wgpu_tests());

    engine.start().expect("Failed to start engine");

    let results = run_tests(&mut engine, &args.test_name);

    reporter::print_summary(results);
}

impl RunResults {
    fn update_result(&mut self, test_result: &TestResult, test_name: &str) {
        if test_result.passed {
            self.passed += 1;
        } else {
            self.failed_names.push(test_name.to_string());
        }
    }
}

fn run_tests(engine: &mut ChronosEngine, test_name_arg: &str) -> RunResults {
    let mut result = RunResults::default();

    if test_name_arg.eq_ignore_ascii_case("all") {
        for scene_name in engine.get_scenes_names() {
            let test_result = run_single_test(&scene_name, engine);
            result.update_result(&test_result, &scene_name);
        }
    } else {
        let test_result = run_single_test(test_name_arg, engine);
        result.update_result(&test_result, test_name_arg);
    }

    result
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

fn run_single_test(test_name: &str, engine: &mut ChronosEngine) -> TestResult {
    if let Err(err) = engine.set_current_scene(test_name) {
        let result = TestResult::error(err.to_string());
        reporter::print_result(test_name, &result);
        return result;
    }

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
