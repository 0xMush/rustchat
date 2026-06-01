# 🦀 Rust AI Chat

A concurrent AI chatbot built in Rust with OpenRouter API integration, conversation memory, and colored output.

## Features

- 💬 **Interactive chat loop** — keep talking until Ctrl+C
- 🧠 **Conversation memory** — AI remembers everything you've said
- 🎨 **Colored output** — user prompts in red, AI responses in green
- 🔐 **Environment variables** — API keys stored safely in `.env`
- ⚡ **Async HTTP requests** — non-blocking API calls with reqwest

## Prerequisites

- Rust (1.70+)
- Cargo
- OpenRouter API key

## Usage

```bash
git clone https://github.com/0xMush/rustchat/
cargo run
```

## Create a .env 
```bash
LLM_MODEL=nvidia/nemotron-3-super-120b-a12b:free
API_KEY=your_openrouter_api_key_here
```

