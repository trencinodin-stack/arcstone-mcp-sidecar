use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(crate) trait ProtectedActuator {
    fn actuate(&mut self, target: &Path, payload: &[u8]) -> Result<(), String>;
}

pub(crate) struct FileActuator;

impl ProtectedActuator for FileActuator {
    fn actuate(&mut self, target: &Path, payload: &[u8]) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(target)
            .map_err(|e| format!("protected actuation failed opening {}: {e}", target.display()))?;

        file.write_all(payload)
            .map_err(|e| format!("protected actuation write failed: {e}"))?;
        file.sync_all()
            .map_err(|e| format!("protected actuation sync failed: {e}"))?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) struct FailingActuator;

#[cfg(test)]
impl ProtectedActuator for FailingActuator {
    fn actuate(&mut self, _target: &Path, _payload: &[u8]) -> Result<(), String> {
        Err("induced actuator failure".to_string())
    }
}
