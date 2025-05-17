use axum::response::{self, IntoResponse};

pub mod encryption;

pub async fn graphiql_sandbox() -> impl IntoResponse {
    let html = r##"
                    <!DOCTYPE html>
                                <html>
                                <head>
                                </head>
                                <body>
                                    <div id="sandbox" style="position:absolute;top:0;right:0;bottom:0;left:0"></div>
                                    <script src="/static/embeddable-sandbox.umd.production.min.js"></script>
                                    <script>
                                    new window.EmbeddedSandbox({
                                    target: "#sandbox",
                                    // Pass through your server href if you are embedding on an endpoint.
                                    // Otherwise, you can pass whatever endpoint you want Sandbox to start up with here.
                                    initialEndpoint: window.location.href,
                                    runTelemetry: false
                                    });
                                    </script>         
                                </body>
                            </html>
                    "##;
    return response::Html(html);
}