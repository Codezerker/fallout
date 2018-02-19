use std::path::PathBuf;

extern crate fallout;
use fallout::xcode_output::driver::Driver;

#[test]
fn test_sample_log_parsing() {
    let mut sample_file  = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sample_file.push("samples/xcodebuild.log");
    let file_path = PathBuf::from(sample_file);

    let mut driver = Driver::new(file_path).unwrap();
    driver.run();

    let warnings = driver.parsed_warnings();
    assert!(warnings.len() == 8);
    assert!(warnings[2].message == "<module-includes>:1:1: warning: umbrella header for module 'WarningKit' does not include header '/Users/eyeplum/Projects/fallout/samples/BuildWarningSampler/WarningKit/WNGView.h'");
    assert!(warnings[2].hint.is_none());
    assert!(warnings[2].location.is_none());

    let warning = warnings.remove(5);
    assert!(warning.message == "/Users/eyeplum/Projects/fallout/samples/BuildWarningSampler/BuildWarningSampler/AppDelegate.swift:23:13: warning: initialization of immutable value 'unusedVariable' was never used; consider replacing with assignment to '_' or removing it");

    let hint = warning.hint.unwrap(); 
    assert!(hint.source    == "        let unusedVariable = \"unused\"");
    assert!(hint.indicator == "        ~~~~^~~~~~~~~~~~~~");
}
