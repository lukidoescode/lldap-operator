#!/usr/bin/env bash
set -euo pipefail

CLUSTER_NAME="lldap-dev"
OPERATOR_IMAGE="lldap-operator:dev"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

if ! docker info > /dev/null 2>&1; then
  echo "Error: Docker is not running. Please start Docker Desktop."
  exit 1
fi

if ! command -v kind > /dev/null 2>&1; then
  echo "Error: kind is not installed. Install it from https://kind.sigs.k8s.io/"
  exit 1
fi

if ! command -v kubectl > /dev/null 2>&1; then
  echo "Error: kubectl is not installed."
  exit 1
fi

if ! command -v helm > /dev/null 2>&1; then
  echo "Error: helm is not installed."
  exit 1
fi

if kind get clusters 2>/dev/null | grep -q "^${CLUSTER_NAME}$"; then
  echo "Cluster '${CLUSTER_NAME}' already exists."
else
  echo "Creating kind cluster '${CLUSTER_NAME}'..."
  kind create cluster --name "$CLUSTER_NAME"
fi

echo "Building operator Docker image..."
docker build -t "$OPERATOR_IMAGE" "$PROJECT_DIR"

echo "Loading image into kind cluster..."
kind load docker-image "$OPERATOR_IMAGE" --name "$CLUSTER_NAME"

echo "Deploying lldap test instance..."
kubectl apply -f "$PROJECT_DIR/tests/e2e/fixtures/lldap-credentials.yaml"
kubectl apply -f "$PROJECT_DIR/tests/e2e/fixtures/lldap-deployment.yaml"
echo "Waiting for lldap to be ready..."
kubectl wait --for=condition=available deployment/lldap --timeout=120s

echo "Installing CRDs..."
kubectl apply -f "$PROJECT_DIR/charts/lldap-resources-operator/crds/"

echo ""
echo "Local development environment is ready."
echo ""
echo "To install the operator via Helm:"
echo "  helm install lldap-operator $PROJECT_DIR/charts/lldap-resources-operator \\"
echo "    --set image.repository=lldap-operator \\"
echo "    --set image.tag=dev \\"
echo "    --set image.pullPolicy=Never \\"
echo "    --set lldap.url=http://lldap:17170"
echo ""
echo "Or run the operator locally against the cluster:"
echo "  cargo run --bin lldap-operator"
echo ""
echo "To tear down:"
echo "  kind delete cluster --name $CLUSTER_NAME"
