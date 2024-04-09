use std::process::Command;
use std::error::Error;

// This Rust program demonstrates how to create a React project using Vite, and how to install necessary dependencies 
// and tools. It includes several functions:
//
// - `run_command`: Executes a given command and returns the output.
// - `check_and_install`: Checks if a package is installed on the system and installs it if it is not found.
// - `create_react_project`: Uses Vite to create a new React project.
// - `install_dependencies`: Installs required dependencies for the project.
// - `configure_tools`: Configures additional tools like Tailwind CSS, Shacdn UI, ESLint, and Prettier.
// - `main`: The entry point of the program that coordinates the flow and execution of the above functions.
//
// Before running this program, you must define the variable `project_name` with your desired project name. Ensure that 
// the functions are called with the correct arguments. The program manages the installation of Node.js, pnpm, and Vite 
// if they are not already present on your system.
//
// Please note: The code presupposes the availability of the `pnpm` command and utilizes it to run various commands. If 
// you use an alternative package manager, you would need to modify the commands accordingly.


fn run_command(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let output = command.output()?;
    if !output.status.success() {
        Err(Box::new(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command execution failed: {:?}", command),
            )
        ))
    } else {
        Ok(())
    }
}

fn check_and_install(package_name: &str, install_cmds: Vec<&str>) -> Result<(), Box<dyn Error>> {
    if !Command::new("command").arg("-v").arg(package_name).output()?.status.success() {
        println!("Installing {}...", package_name);
        let mut install_cmd = Command::new(install_cmds[0]);
        for arg in &install_cmds[1..] {
            install_cmd.arg(arg);
        }
        run_command(&mut install_cmd)?;
    }
    Ok(())
}

fn create_react_project(project_name: &str) -> Result<(), Box<dyn Error>> {
    println!("Creating React project...");
    run_command(Command::new("pnpm")
        .arg("create")
        .arg("vite")
        .arg(project_name)
        .arg("--template")
        .arg("react-ts"))
}

fn install_dependencies(project_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Installing dependencies...");
    run_command(Command::new("pnpm")
        .arg("add")
        .arg("-D")
        .args([
            "tailwindcss@latest",
            "eslint@latest",
            "prettier@latest",
            "@types/node",
            "@types/react",
            "@types/react-dom",
            // ... (add remaining dependencies)
        ])
        .current_dir(project_path))?;

    println!("Installing additional packages...");
    run_command(Command::new("pnpm")
        .arg("add")
        .arg("react-router-dom")
        .arg("shacdn-ui@latest") // Assuming this is how you install this package
        .current_dir(project_path))
}

fn configure_tools(project_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Initializing Tailwind CSS...");
    run_command(Command::new("pnpm").arg("exec").arg("tailwind").arg("init").current_dir(project_path))?;

    println!("Initializing Shacdn UI...");
    // Adjust command based on the correct installation process
    run_command(Command::new("pnpm").arg("dlx").arg("shacdn-ui@latest").arg("init").current_dir(project_path))?;

    println!("Initializing ESLint...");
    run_command(Command::new("pnpm").arg("exec").arg("eslint").arg("--init").current_dir(project_path))?;

    println!("Initializing Prettier...");
    run_command(Command::new("pnpm").arg("exec").arg("prettier").arg("--init").current_dir(project_path))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    check_and_install(
        "node",
        vec!["curl", "-o-", "https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh", "|", "bash"],
    )?;
    check_and_install("pnpm", vec!["npm", "install", "-g", "pnpm@latest"])?;
    check_and_install("vite", vec!["pnpm", "install", "-g", "vite"])?;

    // You would continue with defining 'project_name' and so on as before,
    // but ensure that you call the new functions with appropriate arguments.
    let project_name = "my_new_project"; // This should be defined or retrieved appropriately
    create_react_project(&project_name)?;
    let project_path = format!("./{}", project_name);
    install_dependencies(&project_path)?;
    configure_tools(&project_path)?;

    Ok(())
}
