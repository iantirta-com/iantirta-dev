
use std::{
    error::Error, fs, io::{self, Write}, path::PathBuf, process::{
        Command, exit
    },
};

use clap::{
    Args,
};

use globset::{
    Glob,
    GlobSet,
    GlobSetBuilder
};

use log::{
    debug,
    error,
    warn,
    trace,
};

use crate::{
    cli::{
        Action, ProjectArgs
    },
    config::{
        Config
    }
};



/// Project development
#[derive(Args, Debug, Default)]
pub struct DevelopCommand {
    #[command(flatten)]
    pub project: ProjectArgs,
}


/// Helper project method
impl DevelopCommand {

    /// Resolve project by flag or positional args
    fn get_project_id(&self) -> String {
        if let Some(ref id) = self.project.id {
            id.to_string()
        } else if let Some(ref id) = self.project.id_flag {
             id.to_string()
        } else {
            error!("Error: A Project ID is required!");
            exit(1);
        }
    }

    /// Iterate over project_dir to find exact match
    /// to the project_id
    fn resolve_project_dir_path(&self, config: &Config, project_id: &str) -> Result<PathBuf, Box<dyn Error>> {
        let project_dir = config.project_dir();
        let re = config.project_re();

        // get project_path
        for entry in project_dir.read_dir()? {
            if let Ok(entry) = entry {
                if let Some(caps) = re.captures(entry.file_name().to_str().unwrap()) {
                    if let Some(project_match) = caps.get(1) {
                        if project_match.as_str() == project_id && entry.path().is_dir() {
                            return Ok(entry.path());
                        }
                    }
                }
            }
        }
        return Err(format!("Could not find project directory for ID: {}", project_id).into());
    }

    /// Iterate over resolved project path to check if its a
    /// Python project
    fn resolve_is_project_python(&self, project_path: &PathBuf) -> bool {
        for path in WalkDir::new(&project_path) {
            if let Some(ext) = path.extension() {
                if ext == "py" {
                    return true;
                }
            }
        }
        false
    }
}


/// Venv create build process
impl DevelopCommand {
    fn _input(&self, msg: &str) -> Result<String, Box<dyn Error>> {
        print!("{}", msg);
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        Ok(user_input)
    }

    fn consent_venv_name(&self, config: &Config, project_id: &str) -> Result<String, Box<dyn Error>> {
        let default_name = config.venv_prefix()
            .replace("{prefix}", config.prefix())
            .replace("{project_id}", project_id);
        let msg = format!("Enter project venv folder name [{default_name}]: ");
        let user_input = self._input(&msg)?;
        let venv_name = user_input.trim();
        let final_name = if venv_name.is_empty() {
            &default_name
        } else {
            venv_name
        };
        Ok(final_name.to_string())
    }

    fn consent_venv_sys_sitepkg(&self) -> Result<bool, Box<dyn Error>> {
        let user_input = self._input("Give venv access to system site-packages dir? [Y/n] (default:yes): ")?;
        let sys_sitepkg = user_input.trim().to_lowercase();
        let mut sys_sitepkg_bool = true;
        if sys_sitepkg != "y" && sys_sitepkg != "yes" && !sys_sitepkg.is_empty() {
            sys_sitepkg_bool = false;
        };
        Ok(sys_sitepkg_bool)
    }

    fn consent_venv_prompt(&self) -> Result<String, Box<dyn Error>> {
        let user_input = self._input("Custom venv prompt: ")?;
        let custom_prompt = user_input.trim();
        Ok(custom_prompt.to_string())
    }

    /// Auto creation of venv with user consent
    fn create_venv_with_consent(&self, config: &Config, project_id: &str, venv_base_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>>  {
        println!("NOTE: If this is intentional you can skip creation process.\n");
        let user_input = self._input("> Would you like to create one now? [Y/n]: ")?;
        let consent = user_input.trim().to_lowercase();
        if consent != "y" && consent != "yes" && !consent.is_empty() {
            return Err(format!("Venv creation aborted by user for project: {}", project_id).into());
        }

        let venv_name = self.consent_venv_name(config, project_id)?;
        let sys_site_pkg = self.consent_venv_sys_sitepkg()?;
        let prompt = self.consent_venv_prompt()?;

        // Actual creation
        let venv_path = venv_base_dir.join(venv_name);
        if !venv_base_dir.exists() {
            fs::create_dir_all(&venv_base_dir)?;
        }
        println!("\x1b[1;36m⚙️  Creating virtual environment at {:?}...\x1b[0m", venv_path);
        let python_bin = if cfg!(target_os = "windows") { "python" } else { "python3" };

        let mut cmd = Command::new(python_bin);
        
        cmd.arg("-m")
            .arg("venv")
            .arg(&venv_path);

        if sys_site_pkg {
            cmd.arg("--system-site-packages");
        }
        if !prompt.is_empty() {
            cmd.arg("--prompt")
                .arg(prompt);
        }
        let status = cmd.status()?;

        if status.success() {
            println!("\x1b[1;32m✅ Virtual environment created successfully.\x1b[0m\n");
            Ok(venv_path)
        } else {
            // Clean up the partially created directory if venv creation fails
            if venv_path.exists() {
                let _ = fs::remove_dir_all(&venv_path);
            }
            error!("Failed to create venv. Is Python installed?");
            exit(status.code().unwrap_or(1));
        }
    }

    /// Iterate over venv_dir to find exact match to the user.
    /// And will create with user consent.
    fn resolve_venv_path(&self, config: &Config, project_id: &str) -> Result<PathBuf, Box<dyn Error>> {
        let re = config.venv_re(&project_id);
        let venv_base_dir = config.venv_dir();
        trace!("venv re built: {:?}", re);

        for entry in venv_base_dir.read_dir()? {
            if let Ok(entry) = entry {
                trace!("venv Iterating over: {:?}", entry);
                if let Some(caps) = re.captures(entry.file_name().to_str().unwrap()) {
                    trace!("found venv_path match: {:?}", caps);
                    return Ok(entry.path());
                }
            }
        }

        // create
        warn!("! Detected python environment but no venv available/found for project '{project_id}'.");
        match self.create_venv_with_consent(config, project_id, venv_base_dir) {
            Ok(venv_path) => Ok(venv_path),
            Err(e) => {
                Err(format!("Error while creating venv: {}", e).into())
            }
        }
    }
}


/// Build process method
impl DevelopCommand {

    fn _build_process_cmd_windows_python(
        &self,
        cmd: &mut Command,
        config: &Config,
        project_id: &str,
    ) -> Result<(), Box<dyn Error>>  {
        let vp = self.resolve_venv_path(
            config,
            project_id
        )?;
        let activate_ps1 = vp.join("Scripts/activate.ps1");
        cmd.arg("-Command")
            .arg(&format!("& '{}'", activate_ps1.display()));
        Ok(())
    }

    fn _build_process_cmd_windows(&self, is_python: bool, config: &Config, project_id: &str) -> Result<Command, Box<dyn Error>> {
        let mut cmd = Command::new("powershell");
        cmd.args([
            "-NoExit",
        ]);
        if is_python {
            self._build_process_cmd_windows_python(&mut cmd, config, project_id)?
        }
        Ok(cmd)
    }
    
    fn _build_process_cmd_linux_python(
        &self,
        cmd: &mut Command,
        config: &Config,
        is_bash: bool,
        project_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let vp = self.resolve_venv_path(
            config,
            project_id
        )?;

        if is_bash {
            let activate_sh = vp.join("bin/activate");
            let temp_rc_path = std::env::temp_dir().join(format!("idev_bashrc_{}", project_id));
            
            let rc_content = format!(
                "if [ -f ~/.bashrc ]; then source ~/.bashrc; fi\n\
                 source {activate}\n\
                 export PS1=\"\\[\\e[1;35m\\][idev]\\[\\e[0m\\] $PS1\"\n\
                 export IDEV_ACTIVE_PROJECT=\"{project}\"\n",
                activate=activate_sh.display(),
                project = project_id
            );
            std::fs::write(&temp_rc_path, rc_content)?;
            cmd.arg("--rcfile").arg(&temp_rc_path);

        } else {
            // fallback
            let bin_dir = vp.join("bin");
            if let Ok(current_path) = std::env::var("PATH") {
                debug!("Current shell env $PATH:, {:?}", current_path);
                //std::env::set_var(key, value);
                let new_path = format!("{}:{}", bin_dir.display(), current_path);
                cmd.env("PATH", new_path);
            };
            
            cmd.env("VIRTUAL_ENV", vp);
        }
        Ok(())
    }

    fn _build_process_cmd_linux(&self, is_python: bool, config: &Config, project_id: &str) -> Result<Command, Box<dyn Error>> {
        let shell = match std::env::var("SHELL") {
            Ok(s) => s,
            Err(e) => {
                warn!("Error while getting SHELL Value: {}", e);
                "bash".to_string()
            }
        };
        debug!("Shell: {:}", shell);
        let mut cmd = Command::new(&shell);
        if is_python {
            self._build_process_cmd_linux_python(
                &mut cmd,
                config,
                shell.ends_with("bash"),
                project_id,
            )?
        };
        Ok(cmd)
    }

    /// build process cmd according to os
    fn build_process_cmd(&self, is_python: bool, config: &Config, project_id: &str) -> Result<Command, Box<dyn Error>> {
        if cfg!(target_os = "windows") {
            self._build_process_cmd_windows(is_python, config, project_id)
        } else {
            self._build_process_cmd_linux(is_python, config, project_id)
        }
    }
}


impl Action for DevelopCommand {
    fn run(&self, config: &Config) -> Result<i32, Box<dyn Error>> {
        let project_id = self.get_project_id();
        debug!("Develop Command Run! Project ID: {:#?}", project_id);

        let project_path = self.resolve_project_dir_path(
            config,
            &project_id,
        )?;
        debug!("Successfully found path: {:?}", project_path);

        let is_python = self.resolve_is_project_python(&project_path);
        debug!("Is Python: {:?}", is_python);

        let mut cmd = self.build_process_cmd(
            is_python,
            config,
            &project_id,
        )?;
        cmd.current_dir(&project_path);

        let version = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");

        println!(
            "IDEV Development Workspace: {project_id}\n\
            > Version: {version}\n\
            > Author: {authors}\n\
            \n\
            ! Type 'exit' or press CTRL+D to return."
        );

        let mut child = cmd.spawn()?;
        child.wait()?;

        println!("See you in the next dev!");
        Ok(0)
    }
}


/// Helper Lazy Walk directory with ignore pattern matcher
struct WalkDir {
    stack: Vec<fs::ReadDir>,
    ignore_matcher: GlobSet,
}


impl WalkDir {
    fn new(root: &PathBuf) -> Self {
        let mut stack = Vec::new();

        let ignore_matcher = match Self::build_ignore_matcher(&root) {
            Ok(set) => set,
            Err(e) => {
                error!("Error while setting up ignore matcher: {}", e);
                exit(1)
            }
        };

        let rd = fs::read_dir(root);
        match rd {
            Ok(rd) => {
                stack.push(rd);
                Self { stack, ignore_matcher }
            },
            Err(e) => {
                error!("Error while walk_dir: {}", e);
                exit(1)
            }
        }
    }

    /// Build ignore matcher from .gitignore file if exists.
    fn build_ignore_matcher(root: &PathBuf) -> Result<GlobSet, Box<dyn Error>> {
        let gitignore_path = root.join(".gitignore");
        let mut builder = GlobSetBuilder::new();

        builder.add(Glob::new(".git")?);
        builder.add(Glob::new("**/.git/**")?);

        let raw_patterns = match fs::read_to_string(&gitignore_path) {
            Ok(content) => {
                content
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .collect::<Vec<String>>()
            }
            Err(_) => {
                warn!(".gitignore not found. Using default fallback patterns.");
                vec![
                    "__pycache__/".to_string(),
                    ".venv/".to_string(),
                    "venv/".to_string(),
                    "target/".to_string(),
                    "*.so".to_string(),
                ]
            }
        };

        for pattern in raw_patterns {
            let mut pat = pattern;

            if pat.ends_with('/') {
                let trimmed = pat.trim_end_matches("/");
                builder.add(Glob::new(&format!("**/{}/**", trimmed))?);
                pat = trimmed.to_string();
            }
            builder.add(Glob::new(&pat)?);
            builder.add(Glob::new(&format!("**/{}", pat))?);
        };

        Ok(builder.build()?)
    }
}


impl Iterator for WalkDir {
    type Item = PathBuf;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current_dir) = self.stack.last_mut() {
            if let Some(Ok(entry)) = current_dir.next() {
                let path = entry.path();
                if self.ignore_matcher.is_match(&path) {
                    continue;
                }
                if path.is_dir() {
                    if let Ok(next_dir) = fs::read_dir(path) {
                        self.stack.push(next_dir);
                    }
                } else {
                    return Some(path);
                }
            } else {
                self.stack.pop();
            }
        }
        None
    }
}
