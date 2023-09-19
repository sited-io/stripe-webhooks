fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MEDIA_PROTOS: &[&str] = &[
        "service-apis/proto/peoplesmarkets/media/v1/media_subscription.proto",
    ];

    const INCLUDES: &[&str] = &["service-apis/proto"];

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .build_client(true)
        .compile(MEDIA_PROTOS, INCLUDES)?;

    Ok(())
}
