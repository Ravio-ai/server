use tracing_subscriber::FmtSubscriber;

pub fn logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to Setup Loggin");
}
