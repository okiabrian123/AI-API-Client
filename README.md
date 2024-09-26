# AI-API-Client

This project is an API client built with Rust using Actix Web, providing a secure HTTPS server that interacts with ChatGPT and Gemini APIs. It uses Rustls for handling SSL/TLS and includes CORS support.

## Features
- Secure HTTPS server with self-signed certificates using Rustls.
- Supports API requests to ChatGPT and Gemini.
- Serves static files, including an HTML frontend.
- Configurable API keys for ChatGPT and Gemini.

## Prerequisites

Ensure you have the following installed:
- **Rust** (with Cargo)
- **OpenSSL** (for certificate generation)
- **SSL Certificates** (used in Rustls for HTTPS)

You can generate self-signed certificates by running:

```bash
openssl req -x509 -newkey rsa:4096 -keyout mykey.pem -out mycert.pem -days 365 -nodes
```
Once generated, place mycert.pem and mykey.pem inside the certs folder at the root of the project.

## Usage
### 1. Clone the repository
```bash
git clone https://github.com/okiabrian123/AI-API-Client.git
cd AI-API-Client
```
### 2. Add API Keys
Replace chatgpt_api_key and gemini_api_key in main.rs with your actual API keys.
```rust
let chatgpt_api_key = "your-chatgpt-api-key".to_string();
let gemini_api_key = "your-gemini-api-key".to_owned();
```
### 3. Running the Server
To run the server:

```bash
cargo run
This starts the server on https://127.0.0.1:1011. You can access it from your browser, but due to the self-signed certificates, you may need to configure your browser to trust them.
```
### 4. Access the API
POST /api/generate: Sends a request to either ChatGPT or Gemini APIs based on the provided prompt.
```bash
curl -X POST -H "Content-Type: application/json" \
-d '{"prompt": "Your prompt"}' \
https://127.0.0.1:1011/api/generate --insecure
```
### 5. Serving Static Files
Static files are served from the static folder. Place any additional frontend files like index.html in this directory.</br >
https://127.0.0.1:1011

## Notes
- ChatGPT API Key: This key is required for making requests to OpenAI's ChatGPT API.
- Gemini API Key: This key is required for making requests to the Gemini API.
- Self-Signed Certificates: If you are using self-signed certificates, you may need to configure your web browser to trust the certificate in order to access the server securely.
