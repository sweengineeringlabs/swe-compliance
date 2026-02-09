pub mod cargo_toml;
pub mod source_layout;
pub mod test_org;
pub mod naming;
pub mod metadata;
pub mod documentation;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;

pub fn get_handler(name: &str, def: &RuleDef) -> Option<Box<dyn CheckRunner>> {
    match name {
        // Cargo.toml handlers (structure)
        "crate_root_exists" => Some(Box::new(cargo_toml::CrateRootExists { def: def.clone() })),
        "rustboot_crate_root_exists" => Some(Box::new(cargo_toml::RustbootCrateRootExists { def: def.clone() })),
        "benches_dir_if_declared" => Some(Box::new(cargo_toml::BenchesDirIfDeclared { def: def.clone() })),
        "license_field_exists" => Some(Box::new(cargo_toml::LicenseFieldExists { def: def.clone() })),

        // Cargo.toml target handlers
        "lib_path_correct" => Some(Box::new(cargo_toml::LibPathCorrect { def: def.clone() })),
        "bin_path_correct" => Some(Box::new(cargo_toml::BinPathCorrect { def: def.clone() })),
        "test_targets_declared" => Some(Box::new(cargo_toml::TestTargetsDeclared { def: def.clone() })),
        "bench_harness_false" => Some(Box::new(cargo_toml::BenchHarnessFalse { def: def.clone() })),
        "no_undeclared_tests" => Some(Box::new(cargo_toml::NoUndeclaredTests { def: def.clone() })),
        "no_undeclared_benches" => Some(Box::new(cargo_toml::NoUndeclaredBenches { def: def.clone() })),
        "example_targets_if_dir" => Some(Box::new(cargo_toml::ExampleTargetsIfDir { def: def.clone() })),
        "test_paths_resolve" => Some(Box::new(cargo_toml::TestPathsResolve { def: def.clone() })),

        // Test organization handlers
        "test_file_suffixes" => Some(Box::new(test_org::TestFileSuffixes { def: def.clone() })),
        "test_fn_prefixes" => Some(Box::new(test_org::TestFnPrefixes { def: def.clone() })),
        "test_fn_suffixes" => Some(Box::new(test_org::TestFnSuffixes { def: def.clone() })),
        "int_tests_location" => Some(Box::new(test_org::IntTestsLocation { def: def.clone() })),
        "unit_tests_colocated" => Some(Box::new(test_org::UnitTestsColocated { def: def.clone() })),
        "no_test_in_src" => Some(Box::new(test_org::NoTestInSrc { def: def.clone() })),

        // Naming handlers
        "module_names_match" => Some(Box::new(naming::ModuleNamesMatch { def: def.clone() })),
        "bin_names_valid" => Some(Box::new(naming::BinNamesValid { def: def.clone() })),

        // Documentation handlers
        "doc_dir_exists" => Some(Box::new(documentation::DocDirExists { def: def.clone() })),
        "examples_dir_lib" => Some(Box::new(documentation::ExamplesDirLib { def: def.clone() })),

        _ => None,
    }
}
