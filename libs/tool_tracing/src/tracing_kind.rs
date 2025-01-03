use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::level::VerboseLevel;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TracingKind {
    File,
    Console,
    Otel,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tracing {
    pub kind: TracingKind,
    pub name: String,
    pub level: VerboseLevel,
    #[serde(default)]
    pub additional: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_kind() {
        let tracing = Tracing {
            kind: TracingKind::File,
            name: "file".to_string(),
            level: VerboseLevel::DEBUG,
            additional: HashMap::new(),
        };

        let serialized = serde_json::to_string(&tracing).unwrap();
        let deserialized: Tracing = serde_json::from_str(&serialized).unwrap();

        assert_eq!(tracing, deserialized);
    }

    #[test]
    fn test_tracing_kind_debug() {
        let tracing = Tracing {
            kind: TracingKind::File,
            name: "file".to_string(),
            level: VerboseLevel::DEBUG,
            additional: HashMap::new(),
        };

        assert_eq!(
            format!("{:?}", tracing),
            "Tracing { kind: File, name: \"file\", level: DEBUG, additional: {} }"
        );
    }
}
