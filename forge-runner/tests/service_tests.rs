use forge_runner::domain::models::{Manifest, Project, Step, StepType};
use forge_runner::domain::services::{detect_cycles, resolve_execution_order, validate_manifest};
use std::collections::HashMap;

fn step(name: &str, step_type: StepType, deps: Vec<&str>) -> Step {
    Step {
        name: name.to_string(),
        step_type,
        source: if step_type == StepType::Template {
            Some("src".to_string())
        } else {
            None
        },
        dest: if matches!(step_type, StepType::CreateDir | StepType::Template) {
            Some("dest".to_string())
        } else {
            None
        },
        run: if step_type == StepType::Command {
            Some("echo hi".to_string())
        } else {
            None
        },
        depends_on: deps.into_iter().map(String::from).collect(),
    }
}

fn manifest(steps: Vec<Step>) -> Manifest {
    Manifest {
        project: Project {
            name: "test".to_string(),
            stack: "test".to_string(),
            variables: HashMap::new(),
        },
        steps,
    }
}

#[test]
fn test_parallel_steps_grouped() {
    let steps = vec![
        step("a", StepType::CreateDir, vec![]),
        step("b", StepType::CreateDir, vec![]),
        step("c", StepType::CreateDir, vec![]),
    ];
    let levels = resolve_execution_order(&steps).unwrap();
    assert_eq!(levels.len(), 1);
    assert_eq!(levels[0].len(), 3);
}

#[test]
fn test_sequential_deps() {
    let steps = vec![
        step("a", StepType::CreateDir, vec![]),
        step("b", StepType::Command, vec!["a"]),
        step("c", StepType::Command, vec!["b"]),
    ];
    let levels = resolve_execution_order(&steps).unwrap();
    assert_eq!(levels.len(), 3);
}

#[test]
fn test_mixed_deps() {
    // a -> c, b -> c, c -> d
    let steps = vec![
        step("a", StepType::CreateDir, vec![]),
        step("b", StepType::CreateDir, vec![]),
        step("c", StepType::Command, vec!["a", "b"]),
        step("d", StepType::Command, vec!["c"]),
    ];
    let levels = resolve_execution_order(&steps).unwrap();
    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0].len(), 2); // a, b in parallel
    assert_eq!(levels[1].len(), 1); // c
    assert_eq!(levels[2].len(), 1); // d
}

#[test]
fn test_cycle_three_nodes() {
    let steps = vec![
        step("a", StepType::CreateDir, vec!["c"]),
        step("b", StepType::CreateDir, vec!["a"]),
        step("c", StepType::CreateDir, vec!["b"]),
    ];
    assert!(detect_cycles(&steps).is_err());
}

#[test]
fn test_validate_unknown_dependency() {
    let m = manifest(vec![step("a", StepType::CreateDir, vec!["nonexistent"])]);
    let result = validate_manifest(&m);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown step"));
}

#[test]
fn test_validate_command_without_run() {
    let m = manifest(vec![Step {
        name: "bad-cmd".to_string(),
        step_type: StepType::Command,
        source: None,
        dest: None,
        run: None,
        depends_on: vec![],
    }]);
    let result = validate_manifest(&m);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("requires 'run'"));
}

#[test]
fn test_validate_template_without_source() {
    let m = manifest(vec![Step {
        name: "bad-tmpl".to_string(),
        step_type: StepType::Template,
        source: None,
        dest: Some("out".to_string()),
        run: None,
        depends_on: vec![],
    }]);
    let result = validate_manifest(&m);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("requires 'source'"));
}

#[test]
fn test_validate_no_steps() {
    let m = manifest(vec![]);
    let result = validate_manifest(&m);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("At least one step"));
}

#[test]
fn test_valid_manifest_ok() {
    let m = manifest(vec![
        step("a", StepType::CreateDir, vec![]),
        step("b", StepType::Command, vec!["a"]),
    ]);
    assert!(validate_manifest(&m).is_ok());
}
