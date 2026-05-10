# Astra LLM Foundation

> A production-grade platform for building custom GPT-style tokenizers, transformer architectures, and Hugging Face-compatible LLM systems.

---

# Overview

Astra LLM Foundation is an end-to-end AI systems engineering project focused on building modern Large Language Model (LLM) infrastructure from the ground up.

This repository is designed for:

- Learning how modern AI systems actually work internally
- Building custom GPT-style tokenizers
- Training transformer-based language models
- Experimenting with byte-level BPE tokenization
- Exporting Hugging Face-compatible models and tokenizers
- Creating scalable AI tooling and infrastructure

The architecture combines:

- Rust for high-performance tokenizer systems
- Python + PyTorch for model training
- Next.js for visualization and tooling
- Hugging Face ecosystem compatibility

---

# Core Goals

## 1. Build a Modern Tokenizer

Implement a production-style tokenizer pipeline:

```txt
RAW TEXT
   ↓
Input Validation
   ↓
Unicode Normalization
   ↓
Text Cleaning
   ↓
Whitespace Processing
   ↓
Pre-tokenization
   ↓
Byte Encoding
   ↓
Subword Segmentation (BPE)
   ↓
Vocabulary Lookup
   ↓
Special Token Injection
   ↓
Padding / Truncation
   ↓
Attention Mask Generation
   ↓
FINAL TOKEN IDS
```

---

## 2. Build a Custom Transformer Stack

Train transformer-based models compatible with:

- Hugging Face Transformers
- safetensors
- tokenizer.json
- config.json

---

## 3. Understand LLM Internals

This project is intentionally built from foundational concepts:

- tokenization
- embeddings
- attention
- transformer blocks
- training pipelines
- inference systems

The goal is deep understanding, not only API usage.

---

# Tech Stack

| Layer          | Technology       |
| -------------- | ---------------- |
| Monorepo       | pnpm + Turborepo |
| Frontend       | Next.js 15       |
| Styling        | Tailwind CSS     |
| UI Components  | Radix UI         |
| Tokenizer Core | Rust             |
| AI Training    | PyTorch          |
| API            | FastAPI          |
| Vector Search  | Qdrant           |
| Database       | PostgreSQL       |
| Cache / Queue  | Redis            |
| Model Format   | safetensors      |
| Compatibility  | Hugging Face     |

---

# Repository Structure

```txt
astra-llm-foundation/
│
├── apps/
│   ├── dashboard/         # Next.js dashboard
│   ├── api/               # FastAPI inference API
│   └── trainer/           # PyTorch training system
│
├── packages/
│   ├── tokenizer-core/    # Rust tokenizer engine
│   ├── tokenizer-ts/      # TypeScript bindings
│   ├── shared-config/     # Shared configuration
│   ├── shared-types/      # Shared types/interfaces
│   └── ui/                # Shared UI components
│
├── datasets/              # Training datasets
├── models/                # Exported models
├── checkpoints/           # Training checkpoints
├── logs/                  # Logs
├── infra/                 # Infrastructure configs
├── scripts/               # Utility scripts
│
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
└── README.md
```

---

# System Architecture

```txt
Raw Text
↓
Tokenizer Pipeline
↓
Token IDs
↓
Embedding Layer
↓
Transformer Blocks
↓
Logits
↓
Next Token Prediction
```

---

# Tokenizer Architecture

The tokenizer system follows a modern GPT-style pipeline.

---

## 1. Input Validation

Ensures:

- valid string input
- UTF-8 safety
- non-empty content
- max length validation

---

## 2. Unicode Normalization

Standardizes unicode representations.

Example:

```js
text.normalize("NFC");
```

---

## 3. Text Cleaning

Removes:

- control characters
- malformed unicode
- zero-width characters

---

## 4. Whitespace Processing

Normalizes whitespace:

```txt
"hello     world"
↓
"hello world"
```

---

## 5. Pre-tokenization

Regex-based token splitting.

Example:

```txt
"hello world!"
↓
["hello", " world", "!"]
```

---

## 6. Byte Encoding

Converts UTF-8 text into bytes.

Supports:

- multilingual text
- emojis
- arbitrary unicode

---

## 7. BPE (Byte Pair Encoding)

Performs subword segmentation.

Example:

```txt
technology
↓
tech + nology
```

---

## 8. Vocabulary Lookup

Maps tokens to IDs:

```txt
"hello" → 15339
```

---

## 9. Special Tokens

Supported:

```txt
<PAD>
<UNK>
<BOS>
<EOS>
<USER>
<ASSISTANT>
<SYSTEM>
```

---

## 10. Attention Mask Generation

Example:

```txt
IDs:
[2, 412, 9182, 3, 0, 0]

Mask:
[1, 1, 1, 1, 0, 0]
```

---

# Hugging Face Compatibility

Astra LLM Foundation is designed to export:

```txt
tokenizer.json
tokenizer_config.json
special_tokens_map.json
vocab.json
merges.txt
config.json
model.safetensors
```

This allows models to be used directly with:

```python
from transformers import AutoTokenizer, AutoModel
```

---

# Philosophy

This project follows:

```txt
Custom Internal Architecture
+
Standard External Compatibility
```

Meaning:

- Internal systems can be fully custom
- Export format remains Hugging Face-compatible

This enables:

- experimentation
- optimization
- interoperability
- ecosystem support

---

# Development Roadmap

---

## Phase 1 — Tokenizer Foundation

### Goals

- input validation
- unicode normalization
- whitespace cleanup
- regex pretokenization

---

## Phase 2 — Byte-Level Encoding

### Goals

- UTF-8 byte encoding
- byte mapping
- byte decoding

---

## Phase 3 — BPE Engine

### Goals

- merge training
- vocabulary building
- subword tokenization

---

## Phase 4 — HF Export Layer

### Goals

- tokenizer.json export
- vocab.json export
- merges.txt export

---

## Phase 5 — Transformer Architecture

### Goals

- embeddings
- positional encoding
- self-attention
- transformer blocks

---

## Phase 6 — Training Pipeline

### Goals

- dataset loading
- batching
- training loop
- checkpointing

---

## Phase 7 — Inference API

### Goals

- generation API
- streaming
- batching
- KV cache

---

## Phase 8 — Dashboard & Visualization

### Goals

- tokenizer visualizer
- vocab explorer
- attention maps
- embedding explorer

---

# Installation

---

## 1. Clone Repository

```bash
git clone https://github.com/USERNAME/astra-llm-foundation.git

cd astra-llm-foundation
```

---

## 2. Install pnpm

```bash
npm install -g pnpm
```

---

## 3. Install Dependencies

```bash
pnpm install
```

---

# Dashboard Setup

```bash
cd apps/dashboard

pnpm dev
```

---

# Rust Tokenizer Setup

```bash
cd packages/tokenizer-core

cargo build
```

---

# Python Trainer Setup

```bash
cd apps/trainer

python -m venv .venv

source .venv/bin/activate
```

Install dependencies:

```bash
pip install -r requirements.txt
```

---

# API Setup

```bash
cd apps/api

uvicorn app.main:app --reload
```

---

# Docker Infrastructure

Start services:

```bash
docker compose up
```

Services:

- PostgreSQL
- Redis
- Qdrant

---

# Recommended Learning Order

If you're learning AI systems engineering:

---

## Step 1

Tokenizer fundamentals.

Learn:

- unicode
- bytes
- vocabulary
- BPE

---

## Step 2

Embeddings.

Understand:

```txt
token IDs → vectors
```

---

## Step 3

Attention mechanism.

Learn:

- query
- key
- value
- self-attention

---

## Step 4

Transformer architecture.

---

## Step 5

Training systems.

---

## Step 6

Inference optimization.

---

# Long-Term Vision

Astra LLM Foundation aims to become:

- a tokenizer research platform
- an LLM experimentation platform
- a transformer engineering playground
- a production-ready AI infrastructure stack

Potential future features:

- multimodal support
- RAG pipelines
- distributed training
- quantization
- long-context transformers
- agent systems
- inference optimization

---

# Contributing

Contributions are welcome.

Areas of interest:

- tokenizer research
- BPE optimization
- transformer architectures
- inference systems
- visualization tooling
- multilingual NLP

---

# License

MIT License

---

# Final Note

Modern AI systems are built on top of tokenization.

Understanding tokenizers deeply leads to understanding:

- embeddings
- transformers
- training
- inference
- language modeling itself

This repository exists to explore that foundation properly.
