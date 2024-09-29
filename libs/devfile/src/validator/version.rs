use jsonschema::Validator;
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DevFileVersion {
    V200,
    V210,
    V220,
    V221,
    V222,
    V230,
}

impl DevFileVersion {
    #[tracing::instrument(level = "trace")]
    pub fn validate(yaml: String) -> Option<DevFileVersion> {
        let schema_version = DevFileVersion::extract_schema_version(yaml.clone());

        match schema_version {
            Some(version) => {
                let dev_file_value: serde_json::Value =
                    match serde_json::from_str(&version.get_def()) {
                        Ok(val) => val,
                        Err(err) => {
                            tracing::error!(?err, "Couldn't parse schema",);
                            return None;
                        }
                    };
                let compiled = match Validator::new(&dev_file_value) {
                    Ok(validator) => validator,
                    Err(err) => {
                        tracing::error!(?err, "Couldn't compile schema");
                        return None;
                    }
                };
                let json_value: serde_json::Value = match serde_yaml::from_str(&yaml) {
                    Ok(yaml_to_json) => yaml_to_json,
                    Err(err) => {
                        tracing::error!(?err, "Couldn't parse to json");
                        return None;
                    }
                };
                let result = compiled.validate(&json_value);
                match result {
                    Ok(_) => Some(version),
                    Err(_) => {
                        tracing::error!("Couldn't validate yaml");
                        return None;
                    }
                }
            }
            None => None,
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_def(&self) -> String {
        match &self {
            DevFileVersion::V200 => include_str!("devfile.V200.json").to_string(),
            DevFileVersion::V210 => include_str!("devfile.V220.json").to_string(),
            DevFileVersion::V220 => include_str!("devfile.V220.json").to_string(),
            DevFileVersion::V221 => include_str!("devfile.V221.json").to_string(),
            DevFileVersion::V222 => include_str!("devfile.V222.json").to_string(),
            DevFileVersion::V230 => include_str!("devfile.V230.json").to_string(),
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn extract_schema_version(yaml: String) -> Option<DevFileVersion> {
        let dev_file = serde_yaml::Deserializer::from_str(&yaml);
        let dev_file_value = match Value::deserialize(dev_file) {
            Ok(val) => val,
            Err(err) => {
                tracing::error!(?err, "Couldn't parse yaml");
                return None;
            }
        };
        let schema_version = match dev_file_value.get("schemaVersion") {
            Some(version) => version.as_str(),
            None => {
                tracing::error!("No schemaVersion found, invalid devfile");
                return None;
            }
        };
        match schema_version {
            Some("2.0.0") => {
                tracing::trace!("Found : 2.0.0");
                Some(DevFileVersion::V200)
            }
            Some("2.1.0") => {
                tracing::trace!("Found : 2.1.0");
                Some(DevFileVersion::V210)
            }
            Some("2.2.0") => {
                tracing::trace!("Found : 2.2.0");
                Some(DevFileVersion::V220)
            }
            Some("2.2.1") => {
                tracing::trace!("Found : 2.2.1");
                Some(DevFileVersion::V221)
            }
            Some("2.2.2") => {
                tracing::trace!("Found : 2.2.2");
                Some(DevFileVersion::V222)
            }
            Some("2.3.0") => {
                tracing::trace!("Found : 2.3.0");
                Some(DevFileVersion::V230)
            }
            Some(ver) => {
                tracing::error!("Unknown version : {}", ver);
                None
            }
            None => {
                tracing::error!("Invalid version format, should be a string");
                None
            }
        }
    }
}
