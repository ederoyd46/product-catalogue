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
