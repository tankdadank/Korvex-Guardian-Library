use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MESSAGES_FR: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Core messages
    m.insert("core.configtest.ok", "Guardian Core : la configuration est valide.");
    m.insert("core.configtest.invalid", "Guardian Core : la configuration est invalide.");
    m.insert("core.integrity.failed", "Guardian Core : échec du contrôle d'intégrité.");
    m.insert("core.extras.disabled", "Guardian Core : les fonctionnalités Korvex supplémentaires sont désactivées.");

    // Telemetry
    m.insert("telemetry.mode.off", "Mode de télémétrie : désactivé.");
    m.insert("telemetry.mode.minimal", "Mode de télémétrie : minimal.");
    m.insert("telemetry.mode.debug", "Mode de télémétrie : débogage.");

    // Build status
    m.insert("build.status.official", "État du build : officiel.");
    m.insert("build.status.unknown", "État du build : inconnu. Fonctionnalités supplémentaires désactivées.");
    m.insert("build.status.revoked", "État du build : révoqué. Fonctionnalités supplémentaires désactivées.");

    m
});
