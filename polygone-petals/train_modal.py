import modal
from pathlib import Path

# --- Configuration ---
APP_NAME = "soe-orret-training"
MODEL_ID = "Qwen/Qwen2.5-7B-Instruct"
NFS_PATH = "/models"

image = (
    modal.Image.debian_slim(python_version="3.11")
    .pip_install(
        "torch",
        "transformers",
        "accelerate",
        "datasets",
        "peft",
        "trl",
        "safetensors",
        "sentencepiece",
        "einops",
        "huggingface_hub",
    )
)

app = modal.App(APP_NAME)
nfs = modal.NetworkFileSystem.from_name("soe-orret-models", create_if_missing=True)

@app.function(
    image=image,
    gpu="A100",
    network_file_systems={NFS_PATH: nfs},
    timeout=36000,  # 10 hours
)
def train_a2d(data_path: str, steps: int = 6000):
    import torch
    import torch.nn.functional as F
    from transformers import AutoModelForCausalLM, AutoTokenizer
    
    print(f"Starting A2D conversion for {MODEL_ID}")
    
    # Logic from SOE-ORRET Guide
    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
    if tokenizer.mask_token is None:
        tokenizer.add_special_tokens({'mask_token': '[MASK]'})
        
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID, 
        torch_dtype=torch.bfloat16,
        device_map="auto"
    )
    model.resize_token_embeddings(len(tokenizer))
    
    # Patch attention to bidirectional (A2D recipe)
    patched = 0
    for name, module in model.named_modules():
        if hasattr(module, 'is_causal'):
            module.is_causal = False
            patched += 1
    print(f"Patched {patched} modules: causal -> bidirectional")

    # [Scaffold for training loop]
    # In a real run, we would load datasets from data_path and run the MDLM loss loop.
    # Checkpoints stored in NFS_PATH/orret-dllm-7b
    
    print("Training task initialized. Persistence at /models/orret-dllm-7b")

@app.function(image=image)
def build_check():
    """Verify that dependencies are correct."""
    import torch
    import transformers
    print(f"Environment ready. Torch: {torch.__version__}, Transformers: {transformers.__version__}")

@app.local_entrypoint()
def main(data: str = "./datasets/train.jsonl", steps: int = 6000):
    build_check.remote()
    print("Launching A2D Training on Modal GPU...")
    # train_a2d.remote(data, steps)
