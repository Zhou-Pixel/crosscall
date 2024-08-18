use std::{
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

use anyhow::Context;
use clap::Parser;
use derive_new::new;
use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Command {
    #[command(subcommand)]
    sub: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Gen {
        dart: Option<PathBuf>,
    },
    Tem {},
    New {
        #[arg(long)]
        dart: Option<PathBuf>,
        project_name: String,
    },
}

fn check_flutter_version(path: &str) -> anyhow::Result<()> {
    let mut cmd = process::Command::new(path);
    cmd.arg("--version");

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Failed to execute: {} --version:\n{}",
            path,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn create_flutter_project(path: &str, project_name: &str) -> anyhow::Result<()> {
    let mut cmd = process::Command::new(path);
    cmd.arg("create");
    cmd.arg(project_name);

    cmd.output()?;

    Ok(())
}

#[derive(new)]
pub struct Flutter {
    path: PathBuf,
    cwd: PathBuf,
}

impl Flutter {
    fn command(&self) -> process::Command {
        let mut cmd = process::Command::new(self.path.as_path());
        cmd.current_dir(self.cwd.as_path());
        cmd
    }

    fn check_version(&self) -> anyhow::Result<()> {
        let mut cmd = process::Command::new(self.path.as_path());
        cmd.arg("--version");

        self.exec_command(&mut cmd)
    }

    fn create_project(&self, project_name: &str) -> anyhow::Result<()> {
        let mut cmd = process::Command::new(self.path.as_path());

        cmd.arg("create");
        cmd.arg(project_name);

        cmd.output()?;

        Ok(())
    }

    fn add_crosscall(&self) -> anyhow::Result<()> {
        let mut cmd = self.command();

        cmd.arg("pub");
        cmd.arg("add");
        cmd.arg("crosscall");

        self.exec_command(&mut cmd)
    }

    fn exec_command(&self, cmd: &mut process::Command) -> anyhow::Result<()> {
        let output = cmd.output()?;
        if !output.status.success() {
            let args = cmd.get_args();
            let program = cmd.get_program();
            let cmd_line = format!(
                "{}:{}",
                program.to_string_lossy(),
                args.collect::<Vec<_>>()
                    .join(std::ffi::OsStr::new(" "))
                    .to_string_lossy()
            );
            let cwd = cmd.get_current_dir().unwrap_or(&self.cwd);

            anyhow::bail!(
                "Failed to exec <{}> in <{}>",
                cmd_line,
                cwd.to_string_lossy()
            );
        }

        Ok(())
    }
}

fn exec_command(cmd: &mut process::Command) -> anyhow::Result<()> {
    let output = cmd.output()?;
    if !output.status.success() {
        let args = cmd.get_args();
        let program = cmd.get_program();
        let cmd_line = format!(
            "{}:{}",
            program.to_string_lossy(),
            args.collect::<Vec<_>>()
                .join(std::ffi::OsStr::new(" "))
                .to_string_lossy()
        );
        let cwd = cmd.get_current_dir().map(|v| v.to_string_lossy());

        anyhow::bail!("Failed to exec <{}> in <{:?}>", cmd_line, cwd);
    }

    Ok(())
}

enum LineBreaks {
    Unix,
    Windows,
}

#[derive(new, Debug)]
pub struct Template {
    cwd: PathBuf,
}

impl Template {
    fn create_workspace_cargo_toml(&self) -> anyhow::Result<()> {
        let content = r#"
[workspace]
members = ["./native/*"]
resolver = "2"
        "#;

        let path = self.cwd.join("Cargo.toml");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn native_hub_dir(&self) -> PathBuf {
        self.cwd.join("native").join("hub")
    }

    fn proto_dir(&self) -> PathBuf {
        self.cwd.join("rpc")
    }

    fn create_proto_dir(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(self.proto_dir())?;
        Ok(())
    }

    fn create_calculate(&self) -> anyhow::Result<()> {
        let content = r#"
syntax = "proto3";

package calculate;


message Request {
    int32 first = 1;
    int32 second = 2;
}



message Response {
    int32 result = 1;
}


service Calculate {

    rpc Sum (Request) returns (Response);
}
        "#;

        let path = self.proto_dir().join("calculate.proto");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn create_native_hub(&self) -> anyhow::Result<()> {
        let dir = self.cwd.join("native").join("hub").join("src");

        std::fs::create_dir_all(dir)?;

        Ok(())
    }

    fn create_crosscall_toml(&self) -> anyhow::Result<()> {
        let content = r#"
generate_rust = true
generate_dart = true
[[protobuf]]
file = [ "rpc/*.proto" ]
include = []
        "#;

        let path = self.cwd.join("crosscall.toml");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn create_native_hub_build_rs(&self) -> anyhow::Result<()> {
        let content = r#"
fn main() {
    tonic_build::configure()
        .compile(&["../../rpc/calculate.proto"], &["../../rpc"])
        .unwrap();
}
        "#;

        let path = self.native_hub_dir().join("build.rs");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn create_native_hub_cargo_toml(&self) -> anyhow::Result<()> {
        let content = r#"
[package]
name = "hub"
version = "0.1.0"
edition = "2021"

[dependencies]
crosscall = "*"
tokio = { version = "*", features = ["full"] }
prost = "*"
tonic = "*"

[build-dependencies]
tonic-build = "*"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

        "#;

        let path = self.native_hub_dir().join("Cargo.toml");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn create_native_hub_lib_rs(&self) -> anyhow::Result<()> {
        let content = r#"
crosscall::generate_endpoint!();


pub mod calculate {
    tonic::include_proto!("calculate");
}


pub struct Server {}

#[tonic::async_trait]
impl calculate::calculate_server::Calculate for Server {
    async fn sum(
        &self,
        request: tonic::Request<calculate::Request>,
    ) -> std::result::Result<tonic::Response<calculate::Response>, tonic::Status> {
        Ok(tonic::Response::new(calculate::Response {
            result: request.get_ref().first + request.get_ref().second,
        }))
    }
}


// rust endpoint
async fn start() {
    let listener = crosscall::MemoryListener::bind(None).unwrap();
    tonic::transport::Server::builder()
        .add_service(calculate::calculate_server::CalculateServer::new(Server {}))
        .serve_with_incoming(listener).await.expect("Failed to start grpc");
}
        "#;

        let path = self.native_hub_dir().join("src").join("lib.rs");

        std::fs::write(path, content)?;

        Ok(())
    }
}

pub struct ProtobufCompiler {
    exe: PathBuf,
    cwd: PathBuf,
    env_path: Vec<PathBuf>,
}

impl ProtobufCompiler {
    fn check_version(&self) -> anyhow::Result<()> {
        let mut cmd = self.compiler();
        cmd.arg("--version");

        exec_command(&mut cmd)
    }

    fn add_dart_plugin_path(&mut self) -> anyhow::Result<()> {
        let home_dir = home_dir().context("Failed to query home dir")?;
        self.env_path.push(home_dir.join(".pub-cache").join("bin"));

        Ok(())
    }

    fn compiler(&self) -> process::Command {
        let mut cmd = process::Command::new(self.exe.as_path());
        cmd.current_dir(self.cwd.as_path());

        let path = std::env::var_os("PATH").unwrap_or_default();

        let paths = std::env::split_paths(&path);

        let mut env_path = self.env_path.clone();

        env_path.extend(paths);

        let paths = env_path
            .iter()
            .map(|v| v.as_os_str())
            .collect::<Vec<_>>()
            .join(std::ffi::OsStr::new(env_spec()));

        cmd.env("PATH", paths);

        cmd
    }

    fn compile_dart(
        &self,
        file: &[impl AsRef<Path>],
        output: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let mut cmd = self.compiler();

        for i in file {
            cmd.arg(i.as_ref());
        }

        cmd.arg(format!(
            "--dart_out=grpc:{}",
            output.as_ref().to_string_lossy()
        ));

        exec_command(&mut cmd)
    }
}

fn env_spec() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            ":"
        } else if #[cfg(windows)] {
            ";"
        } else {
            panic!("Unsupport os")
        }
    }
}

struct CrosscallConfig {
    generate_dart: bool,
    generate_rust: bool,
    proto: Vec<Protobuf>,
}

#[derive(Serialize, Deserialize)]
struct Protobuf {
    file: Vec<String>,
    #[serde(default = "default_dart_out_path")]
    dart_output: PathBuf,
    rust_output: Option<PathBuf>,
    include: Vec<PathBuf>,
}

fn default_dart_out_path() -> PathBuf {
    PathBuf::new().join("lib").join("rpc")
}

fn main() -> anyhow::Result<()> {
    let cmd = Command::parse();

    match cmd.sub {
        SubCommand::New { dart, project_name } => {
            new_project(dart.unwrap_or(PathBuf::new().join("flutter")), project_name)?
        }
        _ => todo!(),
    }

    Ok(())
}

fn new_project(dart: PathBuf, project_name: String) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut flutter = Flutter::new(dart, current_dir.clone());

    flutter.check_version()?;

    flutter.create_project(&project_name)?;

    let temp = Template::new(current_dir.join(project_name));

    temp.create_workspace_cargo_toml()?;
    temp.create_native_hub()?;
    temp.create_native_hub_cargo_toml()?;
    temp.create_native_hub_build_rs()?;
    temp.create_native_hub_lib_rs()?;

    temp.create_crosscall_toml()?;

    temp.create_proto_dir()?;

    temp.create_calculate()?;

    Ok(())
}
