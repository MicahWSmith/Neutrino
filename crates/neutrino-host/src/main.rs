use std::{borrow::Cow, env, fs, path::{Path, PathBuf}, sync::Arc};

use neutrino_bridge::exports::neutrino::core::ipc::{Request, Response};
use serde::{Deserialize, Serialize};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wasmtime::{component::Component, Config, Engine, Store};
use wasmtime_wasi::{p2, ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
use wry::{http::Response as HttpResponse, WebViewBuilder};

#[derive(Deserialize)]
struct UiRequest {
    action: String,
    #[serde(default)]
    payload: String,
}

#[derive(Serialize)]
struct UiResponse {
    status: u16,
    body: String,
    error: Option<String>,
}

struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

struct ComponentRuntime {
    engine: Engine,
    component: Component,
}

impl ComponentRuntime {
    fn new(engine: &Engine, component_path: impl AsRef<Path>) -> Result<Self, String> {
        let component = Component::from_file(engine, component_path)
            .map_err(|error| error.to_string())?;
        Ok(Self {
            engine: engine.clone(),
            component,
        })
    }

    fn invoke(&self, request: Request) -> Result<Response, String> {
        let mut linker = wasmtime::component::Linker::<HostState>::new(&self.engine);
        p2::add_to_linker_sync(&mut linker).map_err(|error| error.to_string())?;
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(
            &self.engine,
            HostState {
                wasi,
                table: ResourceTable::new(),
            },
        );
        let instance = linker
            .instantiate(&mut store, &self.component)
            .map_err(|error| error.to_string())?;
        let ipc = instance
            .get_export_index(&mut store, None, "neutrino:core/ipc@0.1.0")
            .ok_or_else(|| "missing neutrino:core/ipc@0.1.0 export".to_owned())?;
        let invoke_export = instance
            .get_export_index(&mut store, Some(&ipc), "invoke")
            .ok_or_else(|| "missing invoke export in neutrino:core/ipc@0.1.0".to_owned())?;
        let invoke = instance
            .get_typed_func::<(Request,), (Response,)>(&mut store, invoke_export)
            .map_err(|error| error.to_string())?;
        invoke
            .call(&mut store, (request,))
            .map(|result| result.0)
            .map_err(|error| error.to_string())
    }
}

fn resource_dir() -> PathBuf {
    if let Ok(path) = env::var("NEUTRINO_RESOURCE_DIR") {
        return PathBuf::from(path);
    }
    if let Ok(executable) = env::current_exe() {
        if let Some(contents) = executable.parent().and_then(Path::parent) {
            let resources = contents.join("Resources");
            if resources.is_dir() {
                return resources;
            }
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resources = resource_dir();
    let component_path = env::var_os("NEUTRINO_COMPONENT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let bundled = resources.join("neutrino_core.wasm");
            if bundled.is_file() {
                bundled
            } else {
                PathBuf::from("target/wasm32-wasip2/release/neutrino_core.wasm")
            }
        });
    let ui_dir = resources.join("ui");
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Arc::new(Engine::new(&config)?);
    let runtime = Arc::new(ComponentRuntime::new(&engine, &component_path)?);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Neutrino")
        .build(&event_loop)?;
    let web_runtime = Arc::clone(&runtime);
    let web_ui_dir = ui_dir.clone();

    let _webview = WebViewBuilder::new()
        .with_custom_protocol("app".into(), move |_, request| {
            let path = request.uri().path();
            if path == "/" || path == "/index.html" {
                return HttpResponse::builder()
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Cow::Owned(fs::read(web_ui_dir.join("index.html")).unwrap_or_default()))
                    .unwrap();
            }
            if path == "/main.js" {
                return HttpResponse::builder()
                    .header("Content-Type", "text/javascript; charset=utf-8")
                    .body(Cow::Owned(fs::read(web_ui_dir.join("main.js")).unwrap_or_default()))
                    .unwrap();
            }
            if path == "/invoke" {
                let input: Result<UiRequest, _> = serde_json::from_slice(request.body());
                let result = input
                    .map_err(|error| error.to_string())
                    .and_then(|input| {
                        web_runtime
                            .invoke(Request {
                                action: input.action,
                                payload: input.payload,
                            })
                    });
                let body = match result {
                    Ok(Response { status, body, error }) => serde_json::to_vec(&UiResponse { status, body, error })
                        .unwrap_or_else(|_| b"{\"status\":500}".to_vec()),
                    Err(error) => serde_json::to_vec(&UiResponse {
                        status: 500,
                        body: String::new(),
                        error: Some(error),
                    })
                    .unwrap(),
                };
                return HttpResponse::builder()
                    .header("Content-Type", "application/json")
                    .body(Cow::Owned(body))
                    .unwrap();
            }
            HttpResponse::builder()
                .status(404)
                .body(Cow::Owned(Vec::new()))
                .unwrap()
        })
        .with_url("app://localhost/index.html")
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
