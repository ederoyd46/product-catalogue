#[macro_export]
macro_rules! error_and_panic {
    ($message:expr) => {{
        log::error!("{}", $message);
        panic!($message);
    }};

    ($message:expr, $error:expr) => {{
        log::error!("{}: [{}]", $message, $error);
        panic!("{}: [{}]", $message, $error);
    }};
}

#[macro_export]
macro_rules! log_and_throw {
    ($message:expr) => {{
        log::error!("{}", $message);
        return Err(AppError {
            kind: "Application",
            message: $message,
        });
    }};

    ($message:expr, $error:expr) => {{
        let message = format!("{}: [{}]", $message, $error);
        log::error!("{}", message);
        return Err($error);
    }};
}

#[macro_export]
macro_rules! local_http {
    ($port:expr, $function:expr) => {{
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        log::info!("Starting HTTP server on port {}", $port);
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
macro_rules! aws_lambda_http_post {
    ($DTO:ty, $function:expr) => {{
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
        log::info!("Initialise Lambda");
        lambda_http::run(lambda_http::service_fn(
            move |event: lambda_http::Request| {
                let (parts, body) = event.into_parts();
                log::debug!("Received parts: {:?}", parts);

                let body_data = match body {
                    lambda_http::Body::Text(val) => val,
                    lambda_http::Body::Binary(val) => String::from_utf8(val).unwrap(),
                    lambda_http::Body::Empty => {
                        core::error_and_panic!("Invalid input, please use a string")
                    }
                };
                let value: $DTO = match serde_json::from_str(&body_data) {
                    Ok(item) => item,
                    Err(e) => core::error_and_panic!("Could not parse input to known type", e),
                };

                $function(value.clone())
            },
        ))
        .await?;
        Ok(())
    }};
}
#[macro_export]
macro_rules! aws_lambda_http_get {
    ($DQO:ty, $function:expr) => {{
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
        log::info!("Initialise Lambda");
        lambda_http::run(lambda_http::service_fn(
            move |event: lambda_http::Request| {
                let (parts, body) = event.into_parts();
                log::debug!("Received event: {:?}", parts);
                log::info!("GET request {:?}", parts.uri);
                let mut chars = parts.uri.path().chars();
                chars.next();
                $function(<$DQO>::new(String::from(chars.as_str()), None))
            },
        ))
        .await?;
        Ok(())
    }};
}
