use proto::{
    management::{
        sandbox_control_server::{SandboxControl, SandboxControlServer},
        CreateRequest, CreateResponse, DestroyRequest, DestroyResponse, Mount, RunCommandRequest,
        RunCommandResponse
    },
    FILE_DESCRIPTOR_SET
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;
use uuid::Uuid;

#[derive(Debug)]
struct Sandbox {
    mounts: Vec<Mount>,
}

#[derive(Debug, Default)]
struct SandboxService {
    sandboxes: Arc<RwLock<HashMap<String, Sandbox>>>,
}

#[tonic::async_trait]
impl SandboxControl for SandboxService {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let request = request.into_inner();
        let uuid = Uuid::new_v4().to_string();
        let sandbox = Sandbox {
            mounts: request.mounts,
        };

        self.sandboxes.write().await.insert(uuid.clone(), sandbox);

        Ok(Response::new(CreateResponse { uuid }))
    }

    async fn destroy(
        &self,
        request: Request<DestroyRequest>,
    ) -> Result<Response<DestroyResponse>, Status> {
        let request = request.into_inner();
        let mut sandboxes = self.sandboxes.write().await;

        if sandboxes.remove(&request.uuid).is_some() {
            Ok(Response::new(DestroyResponse {}))
        } else {
            Err(Status::not_found("Sandbox not found"))
        }
    }

    async fn run_command(
        &self,
        request: Request<RunCommandRequest>,
    ) -> Result<Response<RunCommandResponse>, Status> {
        let request = request.into_inner();
        let sandboxes = self.sandboxes.read().await;

        if !sandboxes.contains_key(&request.uuid) {
            return Err(Status::not_found("Sandbox not found"));
        }

        // Simulated command execution
        let stdout = format!("Ran command: {:?}", request.args);
        let stderr = String::new();
        let exit_code = 0;

        Ok(Response::new(RunCommandResponse {
            exit_code,
            stdout,
            stderr,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;
    let addr = "0.0.0.0:50051".parse()?;
    let service = SandboxService::default();

    println!("macaronid server listening on {}", addr);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(SandboxControlServer::new(service))
        // .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
