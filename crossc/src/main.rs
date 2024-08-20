use std::{
    io::Write,
    path::{Path, PathBuf},
    process,
    sync::mpsc,
};

use anyhow::Context;
use clap::Parser;
use derive_new::new;
use home::home_dir;
use notify::Watcher;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Command {
    #[command(subcommand)]
    sub: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Gen {
        // #[arg(long)]
        // flutter: Option<PathBuf>,
        #[arg(long)]
        protoc: Option<PathBuf>,

        #[arg(long)]
        watch: bool,
    },
    // Tem {},
    New {
        #[arg(long)]
        flutter: Option<PathBuf>,

        #[arg(long)]
        protoc: Option<PathBuf>,

        project_name: String,
    },
    Check {
        #[arg(long)]
        flutter: Option<PathBuf>,

        #[arg(long)]
        protoc: Option<PathBuf>,
    },
}

// fn check_flutter_version(path: &str) -> anyhow::Result<()> {
//     let mut cmd = process::Command::new(path);
//     cmd.arg("--version");

//     let output = cmd.output()?;
//     if !output.status.success() {
//         anyhow::bail!(
//             "Failed to execute: {} --version:\n{}",
//             path,
//             String::from_utf8_lossy(&output.stderr)
//         );
//     }

//     Ok(())
// }

// fn create_flutter_project(path: &str, project_name: &str) -> anyhow::Result<()> {
//     let mut cmd = process::Command::new(path);
//     cmd.arg("create");
//     cmd.arg(project_name);

//     cmd.output()?;

//     Ok(())
// }

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

    fn add_package(&self, package: &str) -> anyhow::Result<()> {
        let mut cmd = self.command();

        cmd.arg("pub");
        cmd.arg("add");

        cmd.arg(package);

        self.exec_command(&mut cmd)
    }

    fn add_crosscall(&self) -> anyhow::Result<()> {
        let mut cmd = self.command();

        cmd.arg("pub");
        cmd.arg("add");
        cmd.arg(format!("crosscall:^{}", env!("CARGO_PKG_VERSION")));

        self.exec_command(&mut cmd)
    }

    fn cd(&mut self, path: PathBuf) {
        self.cwd = path;
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
                "Failed to exec <{}> in <{}>:\n{}",
                cmd_line,
                cwd.to_string_lossy(),
                String::from_utf8_lossy(&output.stderr)
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
            "{} {}",
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

    fn generate_native_hub_build_rs(&self, protobuf: &[Protobuf]) -> anyhow::Result<()> {
        let mut content = String::from("fn main() {\n");

        for i in protobuf {
            let mut include = vec![];
            let mut files = vec![];

            for file in i.file.iter() {
                let mut empty = true;
                for path in glob::glob(&file)? {
                    empty = false;

                    let path = path?;
                    let path = PathBuf::new().join("..").join("..").join(path);

                    if let Some(parent) = path.parent() {
                        include.push(parent.to_path_buf());
                    }

                    files.push(path);
                }
                if empty {
                    tracing::warn!("Path: {} doest't containt any file", file);
                }
            }
            for inc in i.include.iter() {
                include.push(PathBuf::new().join("..").join("..").join(inc));
            }
            let mut out = String::new();
            if let Some(ref output) = i.rust_output {
                out = format!(
                    ".out_dir(\"{}\")",
                    PathBuf::new()
                        .join("..")
                        .join("..")
                        .join(output)
                        .to_string_lossy()
                );
            }
            content.push_str(
                format!(
                    "   tonic_build::configure(){}.compile(&[{}], &[{}]).unwrap();\n",
                    out,
                    files
                        .iter()
                        .map(|v| format!("\"{}\"", v.to_string_lossy()))
                        .collect::<Vec<_>>()
                        .join(","),
                    include
                        .iter()
                        .map(|v| format!("\"{}\"", v.to_string_lossy()))
                        .collect::<Vec<_>>()
                        .join(","),
                )
                .as_str(),
            );
        }

        content.push_str("}\n");
        std::fs::write(self.native_hub_dir().join("build.rs"), content)?;
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
        let content = format!(r#"
[package]
name = "hub"
version = "0.1.0"
edition = "2021"

[dependencies]
crosscall = "{}"
tokio = {{ version = "*", features = ["full"] }}
prost = "*"
tonic = "*"

[build-dependencies]
tonic-build = "*"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

        "#, env!("CARGO_PKG_VERSION"));

        let path = self.native_hub_dir().join("Cargo.toml");

        std::fs::write(path, content)?;

        Ok(())
    }

    fn rewrite_main_dart(&self) -> anyhow::Result<()> {

        let content = 
        r#"
import 'package:crosscall/crosscall.dart';
import 'package:flutter/material.dart';
import './rpc/calculate.pbgrpc.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a purple toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  void _incrementCounter() {
    var channel = ClientChannel(0);

    var client = CalculateClient(channel);

    client.sum(Request(first: _counter, second: 1)).then((response) {
      setState(() {
        _counter = response.result;
      });
      channel
          .shutdown(); //The channel is reusable and you can selectively close it.
    });
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // TRY THIS: Try changing the color here to a specific color (to
        // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
        // change color while the other colors stay the same.
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: Column(
          // Column is also a layout widget. It takes a list of children and
          // arranges them vertically. By default, it sizes itself to fit its
          // children horizontally, and tries to be as tall as its parent.
          //
          // Column has various properties to control how it sizes itself and
          // how it positions its children. Here we use mainAxisAlignment to
          // center the children vertically; the main axis here is the vertical
          // axis because Columns are vertical (the cross axis would be
          // horizontal).
          //
          // TRY THIS: Invoke "debug painting" (choose the "Toggle Debug Paint"
          // action in the IDE, or press "p" in the console), to see the
          // wireframe for each widget.
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
        "#;
        let path = self.cwd.join("lib").join("main.dart");
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
    let listener = crosscall::MemoryListener::bind(Some(0)).unwrap();
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

#[derive(new)]
pub struct ProtobufCompiler {
    exe: PathBuf,
    cwd: PathBuf,

    #[new(default)]
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

    fn compile_dart_config(&self, protobuf: &[Protobuf]) -> anyhow::Result<()> {
        for i in protobuf {
            let mut include = vec![];
            let mut files = vec![];

            for file in i.file.iter() {
                let mut empty = true;
                for path in glob::glob(&file)? {
                    empty = false;

                    let path = path?;

                    if let Some(parent) = path.parent() {
                        include.push(parent.to_path_buf());
                    }

                    files.push(path);
                }
                if empty {
                    tracing::warn!("Path: {} doest't containt any file", file);
                }
            }
            let mut cmd = self.compiler();

            include.extend(i.include.clone());

            for inc in include.iter() {
                cmd.arg(format!("--proto_path={}", inc.to_string_lossy()));
            }

            for file in files {
                cmd.arg(file);
            }

            cmd.arg(format!(
                "--dart_out=grpc:{}",
                i.dart_output.to_string_lossy()
            ));

            std::fs::create_dir_all(&i.dart_output)?;

            exec_command(&mut cmd)?;
        }

        Ok(())
    }

    fn compile_dart(
        &self,
        file: &[impl AsRef<Path>],
        include: &[impl AsRef<Path>],
        output: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let mut cmd = self.compiler();

        let mut include = include.iter().map(|v| v.as_ref()).collect::<Vec<_>>();

        for i in file {
            cmd.arg(i.as_ref());
            if let Some(parent) = i.as_ref().parent() {
                include.push(parent);
            }
        }

        for i in include {
            cmd.arg(format!("--proto_path={}", i.to_string_lossy()));
        }

        cmd.arg(format!(
            "--dart_out=grpc:{}",
            output.as_ref().to_string_lossy()
        ));

        std::fs::create_dir_all(self.cwd.join(output.as_ref()))?;

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

#[derive(Serialize, Deserialize)]
struct CrosscallConfig {
    generate_dart: bool,
    generate_rust: bool,
    protobuf: Vec<Protobuf>,
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

    let format = tracing_subscriber::fmt::format()
        .without_time()
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_target(false);

    tracing_subscriber::fmt().event_format(format).init();

    match cmd.sub {
        SubCommand::New {
            flutter,
            protoc,
            project_name,
        } => {
            let flutter = match flutter {
                Some(flutter) => flutter,
                None => which::which("flutter")?,
            };
            let protoc = match protoc {
                Some(protoc) => protoc,
                None => which::which("protoc")?,
            };
            new_project(flutter, protoc, project_name)?
        }
        SubCommand::Gen {
            // flutter,
            protoc,
            watch,
        } => {
            let protoc = match protoc {
                Some(protoc) => protoc,
                None => which::which("protoc")?,
            };
            if watch {
                watch_project(protoc)?;
            } else {
                generate_project(protoc)?;
            }
        }
        SubCommand::Check { flutter, protoc } => {
            tracing::info!("Searching flutter ...");
            let flutter = match flutter {
                Some(flutter) => flutter,
                None => which::which("flutter")?,
            };

            tracing::info!("Searching protoc ...");

            let protoc = match protoc {
                Some(protoc) => protoc,
                None => which::which("protoc")?,
            };

            check_toolchain(flutter, protoc)?;
        }
        _ => todo!(),
    }

    Ok(())
}

fn watch_project(protoc: PathBuf) -> anyhow::Result<()> {
    loop {
        tracing::info!("Generating project");

        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("crosscall.toml");

        let config = std::fs::read(path)?;
        let config = String::from_utf8(config)?;

        let config: CrosscallConfig = toml::from_str(&config)?;

        if config.generate_dart {
            let compiler = ProtobufCompiler::new(protoc.clone(), current_dir.clone());
            compiler.compile_dart_config(&config.protobuf)?;
        }
        if config.generate_rust {
            let temp = Template::new(current_dir);
            temp.generate_native_hub_build_rs(&config.protobuf)?;
        }

        tracing::info!("Finished");
        let (sender, receiver) = mpsc::channel();

        let mut wathcer = notify::recommended_watcher(move |e| {
            if let Err(err) = sender.send(e) {
                tracing::error!("Internal error: {:?}", err);
                std::process::exit(1);
            }
        })?;

        let current_dir = std::env::current_dir()?;

        wathcer.watch(
            current_dir.join("crosscall.toml").as_path(),
            notify::RecursiveMode::NonRecursive,
        )?;

        for i in config.protobuf {
            let mut files = vec![];

            for file in i.file.iter() {
                let mut empty = true;
                for path in glob::glob(&file)? {
                    empty = false;

                    let path = path?;

                    files.push(path);
                }
                if empty {
                    tracing::warn!("Path: {} doest't containt any file", file);
                }
            }

            for i in files {
                wathcer.watch(&i, notify::RecursiveMode::NonRecursive)?;
            }

        }

        tracing::info!("Watching files ...");
        let event = receiver.recv()?;
        match event {
            Ok(event) => {
                tracing::trace!("Files change: {:?}", event);
            },
            Err(err) => {
                tracing::error!("Error occurr: {:?}", err);
                std::process::exit(1);
            },
        }
    }

    // Ok(())
}

fn check_toolchain(flutter: PathBuf, protoc: PathBuf) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let flutter = Flutter::new(flutter, current_dir.clone());

    tracing::info!("Checking flutter ...");
    flutter.check_version()?;

    tracing::info!("Checking protoc ...");

    let mut protoc = ProtobufCompiler::new(protoc, current_dir);

    protoc.check_version()?;

    let mut file = tempfile::NamedTempFile::new()?;

    let content = r#"
syntax = "proto3";
package protocol;
message Request {}
message Response {}

service Hello {
    rpc Hello(Request) returns (Response);
}
    "#;

    file.write_all(content.as_bytes())?;

    let dir = tempfile::tempdir()?;

    let path = dir.path();

    let mut include = Vec::with_capacity(1);
    if let Some(parent) = path.parent() {
        include.push(parent.to_path_buf());
    }

    tracing::info!("Compiling example protobuf ...");

    protoc.add_dart_plugin_path()?;
    protoc.compile_dart(&[file.path()], &include, dir.path())?;

    Ok(())
}

fn generate_project(protoc: PathBuf) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let path = current_dir.join("crosscall.toml");

    let config = std::fs::read(path)?;
    let config = String::from_utf8(config)?;

    let config: CrosscallConfig = toml::from_str(&config)?;

    if config.generate_dart {
        let compiler = ProtobufCompiler::new(protoc, current_dir.clone());
        compiler.compile_dart_config(&config.protobuf)?;
    }
    if config.generate_rust {
        let temp = Template::new(current_dir);
        temp.generate_native_hub_build_rs(&config.protobuf)?;
    }

    Ok(())
}

fn new_project(dart: PathBuf, protoc: PathBuf, project_name: String) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut flutter = Flutter::new(dart, current_dir.clone());

    tracing::info!("Checking flutter...");
    flutter.check_version()?;

    tracing::info!("Creating flutter project: {} ...", project_name);
    flutter.create_project(&project_name)?;

    flutter.cd(current_dir.join(&project_name));

    tracing::info!("Adding flutter dependency: crosscall ...");
    flutter.add_crosscall()?;

    tracing::info!("Adding flutter dependency: grpc ...");
    flutter.add_package("grpc")?;

    tracing::info!("Adding flutter dependency: protobuf ...");
    flutter.add_package("protobuf")?;


    let temp = Template::new(current_dir.join(&project_name));

    tracing::info!("Rewriting main.dart ...");
    temp.rewrite_main_dart()?;

    tracing::info!("Creating workspace Cargo.toml ...");
    temp.create_workspace_cargo_toml()?;

    tracing::info!("Creating rust crate ...");
    temp.create_native_hub()?;
    temp.create_native_hub_cargo_toml()?;
    temp.create_native_hub_build_rs()?;
    temp.create_native_hub_lib_rs()?;

    tracing::info!("Creating crosscall.toml ...");
    temp.create_crosscall_toml()?;

    tracing::info!("Creating calculate.proto ...");
    temp.create_proto_dir()?;

    temp.create_calculate()?;

    let mut protoc = ProtobufCompiler::new(protoc, current_dir.join(&project_name));

    protoc.add_dart_plugin_path()?;

    protoc.check_version()?;

    tracing::info!("Compiling protobuf ...");
    protoc.compile_dart(&["rpc/calculate.proto"], &[] as &[PathBuf], "lib/rpc")?;

    Ok(())
}
