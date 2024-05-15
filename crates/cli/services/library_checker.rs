use std::{fs::create_dir_all, process::Command};

use verify_core::service::library_checker::{find_problem, root_dir};

pub fn fetch_testcases(problem: verify_core::ProblemForVerify) -> anyhow::Result<()> {
    let problem = find_problem(&problem.problem_id)?;
    let in_dir = problem.dir.join("in");
    let out_dir = problem.dir.join("out");
    create_dir_all(in_dir)?;
    create_dir_all(out_dir)?;
    let info_path = problem.dir.join("info.toml");
    if !info_path.exists() {
        log::warn!("info path is not found: {}", info_path.to_string_lossy());
    }
    Command::new(option_env!("PYTHON").unwrap_or("python"))
        .arg(root_dir()?.join("generate.py"))
        .arg(problem.dir.join("info.toml"))
        .output()?;
    Ok(())
}

const LIBRARY_CHECKER_GIT_REPOSITORY: &str = "https://github.com/yosupo06/library-checker-problems";
pub fn fetch_problem_repository() -> anyhow::Result<()> {
    let root_dir = root_dir()?;
    log::debug!("root directory: {:?}", root_dir.to_str());
    if root_dir.exists() {
        let result = Command::new("git")
            .arg("-C")
            .arg(root_dir.as_os_str())
            .arg("pull")
            .output()?;
        log::debug!("pull stdout: {:?}", String::from_utf8(result.stdout));
        log::debug!("pull stderr: {:?}", String::from_utf8(result.stderr));
    } else {
        let result = Command::new("git")
            .arg("clone")
            .arg(LIBRARY_CHECKER_GIT_REPOSITORY)
            .arg(root_dir.as_os_str())
            .output()?;
        log::debug!("clone stdout: {:?}", String::from_utf8(result.stdout));
        log::debug!("clone stderr: {:?}", String::from_utf8(result.stderr));
    }
    Ok(())
}
