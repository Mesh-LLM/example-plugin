//! Example plugin for Mesh LLM.
//!
//! A minimal external plugin: it binds a loopback HTTP health server,
//! declares its capabilities to the Mesh host, and answers host health
//! checks. Replace the greeting, capabilities and routes with your own
//! feature surface. See the repository README for the release workflow.

mod server;

use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use mesh_llm_plugin::{
    PluginMetadata, PluginRuntime, PluginStartupPolicy, capability, plugin_server_info,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
struct Args {
    /// Greeting the health endpoint answers with. Useful as a template for
    /// plugin-specific configuration supplied by environment or arguments.
    #[arg(
        long,
        env = "EXAMPLE_GREETING",
        default_value = "hello from the example plugin"
    )]
    greeting: String,
}

async fn run_async(args: Args) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind example health listener")?;
    let address = listener.local_addr()?;
    let url = format!("http://{address}");
    tokio::spawn(async move {
        if let Err(error) = axum::serve(
            listener,
            server::router(server::ServerState {
                greeting: args.greeting,
            }),
        )
        .await
        {
            tracing::error!(%error, "example HTTP server stopped");
        }
    });
    let health_url = format!("{url}/health");
    let plugin = mesh_llm_plugin::plugin! {
        metadata: PluginMetadata::new(
            "example", VERSION,
            plugin_server_info(
                "mesh-example", VERSION, "Example plugin",
                "Minimal example plugin: a loopback HTTP surface and health reporting.",
                Some("Point [[plugin]].command at the mesh-example binary in your Mesh configuration."),
            ),
        ),
        startup_policy: PluginStartupPolicy::Any,
        provides: [
            capability("endpoint:inference"),
        ],
        // The declared provider is a toy protocol for the template. Real
        // plugins publish the protocol for their task (see the ONNX plugin's
        // document_ocr_v1) and serve it from the HTTP surface above.
        inference: [
            mesh_llm_plugin::inference::provider("example", url.clone())
                .protocol("example_v1")
                .supports_streaming(false),
        ],
        health: move |_context| {
            let health_url = health_url.clone();
            Box::pin(async move {
                let response = reqwest::Client::builder()
                    .timeout(Duration::from_secs(2))
                    .build()?
                    .get(health_url)
                    .send()
                    .await?
                    .error_for_status()?;
                Ok(format!("example HTTP status {}", response.status()))
            })
        },
    };
    PluginRuntime::run(plugin).await
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(run_async(args))
}
