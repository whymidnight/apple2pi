#[derive(Debug, Clone, Copy)]
pub enum A2PiError {
    HandshakeFailureInPayload,
    HandshakeFailureRTSAcquire,
    HandshakeFailureWrite,
    HandshakeFailureRTSClear,

    InvalidKBPayload,
    InvalidKBInput,
    InvalidKBModifier,
}
