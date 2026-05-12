fn main() {
    println!("cargo:rerun-if-env-changed=RENDEZVOUS_SERVER");
    println!("cargo:rerun-if-env-changed=RS_PUB_KEY");

    let rendezvous_server = std::env::var("RENDEZVOUS_SERVER")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "rs-ny.rustdesk.com".to_owned());
    println!("cargo:rustc-env=RENDEZVOUS_SERVER={rendezvous_server}");

    let rs_pub_key = std::env::var("RS_PUB_KEY")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=".to_owned());
    println!("cargo:rustc-env=RS_PUB_KEY={rs_pub_key}");

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
