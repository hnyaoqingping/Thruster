use log::info;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::ssl_hyper_server::SSLHyperServer;
use thruster::App;
use thruster::ThrusterServer;
use thruster::{async_middleware, middleware_fn};
use thruster::{MiddlewareNext, MiddlewareResult};

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

#[middleware_fn]
async fn test_fn_404(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.body("404");
    Ok(context)
}

fn main() {
    env_logger::init();
    info!("Starting server...");

    let mut app = App::<HyperRequest, Ctx>::create(generate_context);

    app.get("/plaintext", async_middleware!(Ctx, [plaintext]));
    app.set404(async_middleware!(Ctx, [test_fn_404]));

    let mut server = SSLHyperServer::new(app);
    server.cert(include_bytes!("identity.p12").to_vec());
    server.cert_pass("asdfasdfasdf");
    server.start("0.0.0.0", 4321);
}
