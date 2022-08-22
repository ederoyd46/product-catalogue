#[macro_export]
macro_rules! error_and_panic {
    ($message:expr) => {{
        error!("{}", $message);
        panic!($message);
    }};

    ($message:expr, $error:expr) => {{
        error!("{}: [{}]", $message, $error);
        panic!("{}: [{}]", $message, $error);
    }};
}

#[macro_export]
macro_rules! log_and_throw {
    ($message:expr) => {{
        error!("{}", $message);
        return Err(AppError {
            kind: "Application",
            message: $message,
        });
    }};

    ($message:expr, $error:expr) => {{
        let message = format!("{}: [{}]", $message, $error);
        error!("{}", message);
        return Err($error);
    }};
}

#[macro_export]
macro_rules! local_http {
    ($port:expr, $function:expr) => {{
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        log::info!("starting HTTP server on port {}", $port);

        // Start HTTP server
        HttpServer::new(move || {
            App::new()
                .service($function)
                .wrap(middleware::Logger::default())
        })
        .workers(1)
        .bind(("127.0.0.1", $port))?
        .run()
        .await
    }};
}
#[macro_export]
macro_rules! lambda_http_run {
    ($Dto:ty, $function:expr) => {{
        lambda_http::run(service_fn(move |event: Request| {
            let body = match event.body() {
                Body::Text(val) => val.as_ref(),
                Body::Binary(val) => std::str::from_utf8(val).unwrap(),
                Body::Empty => error_and_panic!("Invalid input, please use a string"),
            };
            let value: $Dto = match serde_json::from_str(body) {
                Ok(item) => item,
                Err(e) => error_and_panic!("Could not parse input to known type", e),
            };

            $function(value.clone())
        })).await?;
    }};
}
