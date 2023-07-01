#[derive(Debug)]
pub enum A2PiError {
    HandshakeFailureInPayload,
    HandshakeFailureRTSAcquire,
    HandshakeFailureWrite,
    HandshakeFailureRTSClear,

    InvalidKBPayload,
    InvalidKBInput,
    InvalidKBModifier,
}
