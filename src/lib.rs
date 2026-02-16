use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;

#[http_component]
async fn handle_wasm_hello(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Prefer Spin-provided path info header, fall back to uri()
    let raw_path: &str = req
        .header("spin-path-info")
        .and_then(|hv| hv.as_str())
        .unwrap_or(req.uri());


    // Remove query string if present
    let raw_path = raw_path.split('?').next().unwrap_or(raw_path);

    // Normalize: ensure leading slash
    let path = if raw_path.starts_with('/') {
        raw_path.to_string()
    } else {
        format!("/{}", raw_path)
    };

    println!("Raw uri: {}", req.uri());
    println!("Normalized path: {}", path);

    match path.as_str() {
        "/hello" => Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("Hello from WebAssembly 👋")
            .build()),

        "/quote" => {
            let outbound_req = Request::builder()
                .method(Method::Get)
                .uri("http://127.0.0.1:3001/quote.json")
                .build();

            let outbound_resp: Response = spin_sdk::http::send(outbound_req).await?;
            let body = String::from_utf8_lossy(outbound_resp.body()).to_string();

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(body)
                .build())
        }

        _ => Ok(Response::builder()
            .status(404)
            .header("content-type", "text/plain")
            .body(format!("Not found. Got path: {}", path))
            .build()),
    }
}
