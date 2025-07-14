use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("target/generated")?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("target/generated")
        .compile_protos(
            &[
                "proto/medpass/records/v1/record.proto",
                "proto/medpass/users/v1/users.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
