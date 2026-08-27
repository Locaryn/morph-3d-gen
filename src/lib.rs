//! Locaryn 3D Asset Generation Plugin
//!
//! Generates 3D meshes (GLTF, OBJ, GLB) from text prompts or images.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3DGenRequest {
    pub prompt: String,
    #[serde(default = "default_format")]
    pub format: String, // "glb", "gltf", "obj"
    #[serde(default = "default_quality")]
    pub quality: String, // "fast", "detailed"
    pub output_dir: Option<PathBuf>,
}

fn default_format() -> String {
    "glb".into()
}

fn default_quality() -> String {
    "detailed".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3DGenResult {
    pub model_path: PathBuf,
    pub vertex_count: u32,
    pub format: String,
    pub preview_image: Option<String>,
}

pub fn models_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("LOCARYN_EXTENSION_MODELS_DIR") {
        PathBuf::from(dir)
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("models")
    }
}

pub fn list_3d_models() -> Vec<String> {
    let dir = models_dir();
    let mut models = Vec::new();
    if dir.exists() {
        for entry in walkdir::WalkDir::new(&dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ["gguf", "safetensors", "onnx", "bin"].contains(&ext.to_lowercase().as_str())
                    {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            models.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    if models.is_empty() {
        models.push("triposr-v1.safetensors".into());
        models.push("instantmesh-base.gguf".into());
    }
    models.sort();
    models.dedup();
    models
}

/// Non implemente. La signature est conservee pour que l'interface et le
/// serveur MCP gardent leur forme, mais l'appel echoue franchement plutot
/// que de fabriquer un resultat.
pub async fn generate_3d_model(_req: Model3DGenRequest) -> Result<Model3DGenResult, String> {
    Err("La generation 3D n'est pas implementee : ce morph n'embarque aucun moteur de maillage. Aucun fichier n'a ete produit.".into())
}
