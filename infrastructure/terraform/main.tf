terraform {
  required_version = ">= 1.8.0"
  required_providers { kubernetes = { source = "hashicorp/kubernetes", version = "~> 2.0" } }
}

variable "namespace" { type = string, default = "decomproof" }

resource "kubernetes_namespace_v1" "decomproof" { metadata { name = var.namespace } }

# This example intentionally provisions only the namespace. Deploy application resources with the
# Kubernetes manifests after supplying real image locations and secrets; no fake cloud resources are declared.
