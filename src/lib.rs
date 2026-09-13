mod actuator;
pub mod authorization;
pub mod evidence;
pub mod gate;
pub mod issuer;
pub mod request;

pub use authorization::{AuthorizationRecord, AuthorizationState};
pub use evidence::{ActuationOutcome, EvidenceContext, ExecutionEvidence};
pub use gate::{execute_request, Decision, DenyReason, ExecutionResult, ExperimentPaths};
pub use issuer::issue_authorization;
pub use request::{ExecutionRequest, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG};
