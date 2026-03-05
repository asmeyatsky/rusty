use forge_runner::domain::models::Manifest;
use forge_runner::domain::ports::ManifestLoader;
use forge_runner::domain::services::validate_manifest;
use forge_runner::infrastructure::yaml_loader::SerdeYamlLoader;

#[tokio::test]
async fn test_load_valid_manifest() {
    let loader = SerdeYamlLoader;
    let manifest = loader
        .load("tests/fixtures/valid_forge.yaml")
        .await
        .unwrap();
    assert_eq!(manifest.project.name, "test-project");
    assert_eq!(manifest.project.stack, "basic");
    assert_eq!(manifest.steps.len(), 3);
    assert_eq!(manifest.project.variables.get("greeting").unwrap(), "Hello");
}

#[tokio::test]
async fn test_load_invalid_manifest_validates() {
    let loader = SerdeYamlLoader;
    let manifest = loader
        .load("tests/fixtures/invalid_forge.yaml")
        .await
        .unwrap();
    let result = validate_manifest(&manifest);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_load_nonexistent_file() {
    let loader = SerdeYamlLoader;
    let result = loader.load("tests/fixtures/does_not_exist.yaml").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_valid_manifest_passes_validation() {
    let loader = SerdeYamlLoader;
    let manifest = loader
        .load("tests/fixtures/valid_forge.yaml")
        .await
        .unwrap();
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_manifest_variables_parsed() {
    let yaml = r#"
project:
  name: vars-test
  stack: test
  variables:
    foo: bar
    baz: "123"

steps:
  - name: step1
    step_type: create_dir
    dest: out
"#;
    let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(manifest.project.variables.len(), 2);
    assert_eq!(manifest.project.variables.get("foo").unwrap(), "bar");
}

#[tokio::test]
async fn test_manifest_depends_on_parsed() {
    let yaml = r#"
project:
  name: deps-test
  stack: test

steps:
  - name: first
    step_type: create_dir
    dest: out
  - name: second
    step_type: command
    run: echo hi
    depends_on:
      - first
"#;
    let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
    assert!(manifest.steps[0].depends_on.is_empty());
    assert_eq!(manifest.steps[1].depends_on, vec!["first"]);
}
