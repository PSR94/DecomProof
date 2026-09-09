# Evidence model

Every evidence item has a stable ID derived from signal, source, target and raw evidence hash. It records observed time, optional observation window, confidence, normalized values, optional consumer identity, artifact reference and notes.

Normalized fields are the policy-facing layer. Raw payloads are not blindly embedded; the model stores a hash and selected structured observations. Graph edges and blockers cite evidence IDs so a human can trace a verdict back to claims.

Consumer identity quality is one of `identified`, `partial` or `unknown`. Unknown active consumers can be configured as blocking and are never treated as proof of absence.
