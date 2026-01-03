use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};

pub fn create_rate_limiter(
) -> tower_governor::GovernorLayer<'static, SmartIpKeyExtractor, _> {
    // 100 requests per minute per IP
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)  // ~120 per minute
            .burst_size(100)
            .finish()
            .unwrap(),
    );

    tower_governor::GovernorLayer {
        config: Box::leak(governor_conf),
    }
}
