use morannon_config::{ClientId, Configuration, ModelId, ProviderId};

#[test]
fn load_example_config() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(
        concat!(env!("CARGO_MANIFEST_DIR"), "/examples/morannon.toml")
    )
    .expect("example config not found");

    let config: Configuration = toml::from_str(&contents)?;
    let daemon = &config.daemon;

    assert_eq!(daemon.bind_address.to_string(), "127.0.0.1");
    assert_eq!(daemon.bind_port, 3019);

    let client1 = daemon.clients.get(&ClientId::from("my-client-1"))
        .expect("my-client-1 not found");
    assert_eq!(client1.key, "local-abc123");
    assert_eq!(client1.models, [ModelId::from("model-1"), ModelId::from("model-2")]);

    let client2 = daemon.clients.get(&ClientId::from("my-client-2"))
        .expect("my-client-2 not found");
    assert_eq!(client2.key, "local-def456");
    assert_eq!(client2.models, [ModelId::from("model-1")]);

    let model1 = daemon.models.get(&ModelId::from("model-1"))
        .expect("model-1 not found");
    assert_eq!(String::from(model1.provider.clone()), "openai");
    assert_eq!(model1.upstream_name, "gpt-4o");
    assert_eq!(model1.downstream_name, "smart-model");

    let model2 = daemon.models.get(&ModelId::from("model-2"))
        .expect("model-2 not found");
    assert_eq!(String::from(model2.provider.clone()), "anthropic");
    assert_eq!(model2.upstream_name, "claude-haiku-4-5");
    assert_eq!(model2.downstream_name, "fast-model");

    let openai = daemon.providers.get(&ProviderId::from("openai"))
        .expect("openai not found");
    assert_eq!(openai.api_base, "https://api.openai.com");
    assert_eq!(openai.api_key, "XYZ456");

    let anthropic = daemon.providers.get(&ProviderId::from("anthropic"))
        .expect("anthropic not found");
    assert_eq!(anthropic.api_base, "https://api.anthropic.com");
    assert_eq!(anthropic.api_key, "ABC123");

    Ok(())
}
