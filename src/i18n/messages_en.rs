use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MESSAGES_EN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Core messages
    m.insert("core.configtest.ok", "Guardian Core: configuration is valid.");
    m.insert("core.configtest.invalid", "Guardian Core: configuration is invalid.");
    m.insert("core.integrity.failed", "Guardian Core: integrity check failed.");
    m.insert("core.extras.disabled", "Guardian Core: Korvex extras are disabled.");

    // Telemetry
    m.insert("telemetry.mode.off", "Telemetry mode: off.");
    m.insert("telemetry.mode.minimal", "Telemetry mode: minimal.");
    m.insert("telemetry.mode.debug", "Telemetry mode: debug.");

    // Build status
    m.insert("build.status.official", "Build status: official.");
    m.insert("build.status.unknown", "Build status: unknown. Extras disabled.");
    m.insert("build.status.revoked", "Build status: revoked. Extras disabled.");

    m
});
