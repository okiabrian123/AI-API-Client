
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App,  HttpServer};
use handlers::handle_generate_request;
use API_AI::models::AppState;
use std::path::PathBuf;
use std::{env, fs, io};
use rustls::{ pki_types, ServerConfig};
use pki_types::{CertificateDer, PrivateKeyDer};
mod handlers;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}
// Fungsi middleware untuk memeriksa otentikasi
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    env_logger::init();
     // Ambil direktori kerja saat ini
     let exe_path = env::current_exe().expect("Failed to get executable path");
     // Mendapatkan direktori induk dari executable
     let exe_dir = exe_path.parent().expect("Failed to get parent directory");


    let mut certs_path =  PathBuf::from(exe_dir);
    certs_path.push("certs/mycert.pem");

    let mut key_path =  PathBuf::from(exe_dir);
    key_path.push("certs/mykey.pem");
    
    let chatgpt_api_key = "add key to here".to_string();
    let gemini_api_key= "add key to here".to_owned();
    let data = web::Data::new(AppState::new(chatgpt_api_key,gemini_api_key));
    let certs = load_certs(certs_path.to_str().expect("error"))?;

    // Load private key.
    let key: PrivateKeyDer<'_> = load_private_key(key_path.to_str().expect("error"))?;
    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| error(e.to_string()))?;


    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Content-Type"])
                    .max_age(3600),
            )
            .service(
                web::resource("/api/generate")
                    .route(web::post().to(handle_generate_request))
                    .route(web::post().to(handle_generate_request))
            ) 
            .service(
                web::resource("/")
                    .route(web::get().to(|| async {
                        actix_files::NamedFile::open("./static/index.html")
                            .unwrap()
                    }))
            )
            .service(
                Files::new("/static", "./static").show_files_listing(), // Serve other static files
            )

    })
    .bind_rustls_0_23("127.0.0.1:1011", server_config)?
    .run()
    .await
}



fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    rustls_pemfile::certs(&mut reader).collect()
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}
