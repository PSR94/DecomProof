# Prior art and adjacent categories

DecomProof deliberately spans categories that are usually solved independently. This review was performed before implementation and prioritizes official project documentation.

| Project/product | Category | What it does | Overlap | Limitation relative to DecomProof | Useful idea | DecomProof difference |
|---|---|---|---|---|---|---|
| Knip | JS/TS dead-code detection | Finds unused files, exports and dependencies, with framework-aware entry points | Static reachability | Does not establish runtime, data, schedule, external-consumer or infrastructure quiescence | Precise, actionable static findings | Static findings are one evidence family, never a deletion verdict |
| Unleash | Feature-flag lifecycle | Tracks stale flags and cleanup lifecycle using usage metrics and expected lifetimes | Retirement lifecycle and runtime usage | Scope is feature flags; it does not prove broad service/API/data/infra readiness | Lifecycle states and stale-age policy | One target/evidence model across component kinds |
| RFC 8594 Sunset | API decommissioning contract | Standard HTTP `Sunset` response header advertises when a resource is expected to become unresponsive | Public API retirement | It communicates intent; it does not discover consumers or prove quiescence | Explicit sunset windows | Treats contract state as evidence alongside actual use |
| OpenTelemetry service graph connector | Runtime dependency mapping | Derives inter-service topology metrics from traces | Runtime consumer/dependency discovery | Trace coverage can be sampled/incomplete and says little about code, data, contracts or scheduled rare use | Evidence-backed graph edges | Preserves coverage uncertainty and fuses graph edges with other evidence |
| Backstage Catalog Graph | Service catalog / dependency graph | Models component, API and resource relations such as dependsOn and consumesApi | Dependency topology and lifecycle metadata | Primarily a catalog/mental model rather than exhaustive real-time source of truth | Typed directional relationships | Graph edges are derived from concrete evidence and cite evidence IDs |
| Cloud Custodian | Cloud governance / unused-resource cleanup | Policy engine for cloud resources, including cost and garbage-collection use cases | Infrastructure cleanup policy | Cloud-resource focused; actions can remediate/delete resources | Policy-as-code and explicit filters | Read-only by default and decommission verdict covers non-cloud evidence |
| Terraform | IaC lifecycle | Plans/destroys resources after configuration references are removed | Infrastructure decommissioning | Validating Terraform references does not establish application/user safety | Dependency validation before destroy | Terraform presence is a blocker/evidence signal, not the only criterion |
| Burrow | Kafka consumer monitoring | Reads Kafka consumer offsets/group metadata and evaluates consumer group health | Event consumer evidence | Kafka-specific and focused on consumer health | Consumer-group metadata as structured evidence | Event evidence is fused with static/contracts/data/runtime signals |
| onwardpg | PostgreSQL safe migrations | Plans expand/deploy/drain/contract compatibility windows | Safe schema retirement | Focused on schema migration execution rather than broad component decommission | Drain/convergence framing | Observation windows apply to all target kinds and produce a proof artifact |

## Deliberate product boundary

Finding a reference and cleaning one category are well-served problems. DecomProof's differentiator is one developer-centric workflow that asks whether the available evidence is sufficient under policy to support removal, records what remains unknown, and verifies after removal.

## Sources

- https://knip.dev/
- https://docs.getunleash.io/concepts/feature-flags
- https://www.rfc-editor.org/rfc/rfc8594.html
- https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/connector/servicegraphconnector
- https://backstage.io/docs/features/software-catalog/creating-the-catalog-graph/
- https://cloudcustodian.io/
- https://developer.hashicorp.com/terraform/language/resources/destroy
- https://github.com/linkedin/Burrow/wiki/Consumer-Kafka
- https://github.com/jokull/onwardpg
