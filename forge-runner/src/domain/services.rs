use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};

use super::models::{Manifest, Step, StepType};

pub fn validate_manifest(manifest: &Manifest) -> Result<()> {
    if manifest.project.name.is_empty() {
        bail!("Project name is required");
    }
    if manifest.project.stack.is_empty() {
        bail!("Project stack is required");
    }
    if manifest.steps.is_empty() {
        bail!("At least one step is required");
    }

    let mut seen_names = HashSet::new();
    for step in &manifest.steps {
        if step.name.is_empty() {
            bail!("Step name is required");
        }
        if !seen_names.insert(&step.name) {
            bail!("Duplicate step name: '{}'", step.name);
        }

        match step.step_type {
            StepType::CreateDir => {
                if step.dest.is_none() {
                    bail!("Step '{}': create_dir requires 'dest'", step.name);
                }
            }
            StepType::Template => {
                if step.source.is_none() {
                    bail!("Step '{}': template requires 'source'", step.name);
                }
                if step.dest.is_none() {
                    bail!("Step '{}': template requires 'dest'", step.name);
                }
            }
            StepType::Command => {
                if step.run.is_none() {
                    bail!("Step '{}': command requires 'run'", step.name);
                }
            }
        }

        let step_names: HashSet<&str> = manifest.steps.iter().map(|s| s.name.as_str()).collect();
        for dep in &step.depends_on {
            if !step_names.contains(dep.as_str()) {
                bail!("Step '{}': depends on unknown step '{}'", step.name, dep);
            }
        }
    }

    detect_cycles(&manifest.steps)?;
    Ok(())
}

pub fn detect_cycles(steps: &[Step]) -> Result<()> {
    let index_map: HashMap<&str, usize> = steps
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name.as_str(), i))
        .collect();

    let mut state = vec![0u8; steps.len()]; // 0=unvisited, 1=in-progress, 2=done

    fn dfs(
        node: usize,
        steps: &[Step],
        index_map: &HashMap<&str, usize>,
        state: &mut [u8],
        path: &mut Vec<String>,
    ) -> Result<()> {
        state[node] = 1;
        path.push(steps[node].name.clone());

        for dep in &steps[node].depends_on {
            if let Some(&dep_idx) = index_map.get(dep.as_str()) {
                match state[dep_idx] {
                    1 => {
                        path.push(dep.clone());
                        bail!("Dependency cycle detected: {}", path.join(" -> "));
                    }
                    0 => {
                        dfs(dep_idx, steps, index_map, state, path)?;
                    }
                    _ => {}
                }
            }
        }

        path.pop();
        state[node] = 2;
        Ok(())
    }

    for i in 0..steps.len() {
        if state[i] == 0 {
            let mut path = Vec::new();
            dfs(i, steps, &index_map, &mut state, &mut path)?;
        }
    }

    Ok(())
}

/// Returns steps grouped into levels for parallel execution.
/// Each level contains steps whose dependencies are all in prior levels.
pub fn resolve_execution_order(steps: &[Step]) -> Result<Vec<Vec<Step>>> {
    detect_cycles(steps)?;

    let step_names: HashSet<&str> = steps.iter().map(|s| s.name.as_str()).collect();
    for step in steps {
        for dep in &step.depends_on {
            if !step_names.contains(dep.as_str()) {
                bail!("Step '{}': depends on unknown step '{}'", step.name, dep);
            }
        }
    }

    let mut resolved: HashSet<String> = HashSet::new();
    let mut remaining: Vec<Step> = steps.to_vec();
    let mut levels: Vec<Vec<Step>> = Vec::new();

    while !remaining.is_empty() {
        let (ready, not_ready): (Vec<Step>, Vec<Step>) = remaining
            .into_iter()
            .partition(|s| s.depends_on.iter().all(|d| resolved.contains(d)));

        if ready.is_empty() {
            let names: Vec<&str> = not_ready.iter().map(|s| s.name.as_str()).collect();
            bail!("Unresolvable dependencies for steps: {:?}", names);
        }

        for s in &ready {
            resolved.insert(s.name.clone());
        }
        levels.push(ready);
        remaining = not_ready;
    }

    Ok(levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_step(name: &str, step_type: StepType, deps: Vec<&str>) -> Step {
        Step {
            name: name.to_string(),
            step_type,
            source: match step_type {
                StepType::Template => Some("src".to_string()),
                _ => None,
            },
            dest: match step_type {
                StepType::CreateDir | StepType::Template => Some("dest".to_string()),
                _ => None,
            },
            run: match step_type {
                StepType::Command => Some("echo hi".to_string()),
                _ => None,
            },
            depends_on: deps.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_no_deps_single_level() {
        let steps = vec![
            make_step("a", StepType::CreateDir, vec![]),
            make_step("b", StepType::CreateDir, vec![]),
        ];
        let levels = resolve_execution_order(&steps).unwrap();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].len(), 2);
    }

    #[test]
    fn test_linear_deps() {
        let steps = vec![
            make_step("a", StepType::CreateDir, vec![]),
            make_step("b", StepType::Command, vec!["a"]),
            make_step("c", StepType::Command, vec!["b"]),
        ];
        let levels = resolve_execution_order(&steps).unwrap();
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0][0].name, "a");
        assert_eq!(levels[1][0].name, "b");
        assert_eq!(levels[2][0].name, "c");
    }

    #[test]
    fn test_diamond_deps() {
        let steps = vec![
            make_step("a", StepType::CreateDir, vec![]),
            make_step("b", StepType::Command, vec!["a"]),
            make_step("c", StepType::Command, vec!["a"]),
            make_step("d", StepType::Command, vec!["b", "c"]),
        ];
        let levels = resolve_execution_order(&steps).unwrap();
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0][0].name, "a");
        assert_eq!(levels[1].len(), 2);
        assert_eq!(levels[2][0].name, "d");
    }

    #[test]
    fn test_cycle_detection() {
        let steps = vec![
            make_step("a", StepType::CreateDir, vec!["b"]),
            make_step("b", StepType::CreateDir, vec!["a"]),
        ];
        let result = detect_cycles(&steps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_self_cycle() {
        let steps = vec![make_step("a", StepType::CreateDir, vec!["a"])];
        let result = detect_cycles(&steps);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_name() {
        let manifest = Manifest {
            project: super::super::models::Project {
                name: String::new(),
                stack: "rust".to_string(),
                variables: Default::default(),
            },
            steps: vec![make_step("a", StepType::CreateDir, vec![])],
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_duplicate_step_names() {
        let manifest = Manifest {
            project: super::super::models::Project {
                name: "test".to_string(),
                stack: "rust".to_string(),
                variables: Default::default(),
            },
            steps: vec![
                make_step("a", StepType::CreateDir, vec![]),
                make_step("a", StepType::CreateDir, vec![]),
            ],
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_missing_dest() {
        let manifest = Manifest {
            project: super::super::models::Project {
                name: "test".to_string(),
                stack: "rust".to_string(),
                variables: Default::default(),
            },
            steps: vec![Step {
                name: "a".to_string(),
                step_type: StepType::CreateDir,
                source: None,
                dest: None,
                run: None,
                depends_on: vec![],
            }],
        };
        assert!(validate_manifest(&manifest).is_err());
    }
}
