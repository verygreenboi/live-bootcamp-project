## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:8000 and http://localhost:3000

## Kubernetes Deployment

The application is deployed to Kubernetes using a CI/CD pipeline implemented with GitHub Actions. Below is a detailed description of the Kubernetes resources and deployment process.

### Kubernetes Resources

#### Namespace

The application is deployed in a dedicated Kubernetes namespace called `live-bootcamp-proj`:

```yaml
apiVersion: v1
kind: Namespace
metadata:
    name: live-bootcamp-proj
    labels:
        name: live-bootcamp-proj
```

#### Deployment

The application uses a single Deployment that contains both the app-service and auth-service containers:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: app-service
  namespace: live-bootcamp-proj
  labels:
    app: app-service
spec:
  replicas: 1
  selector:
    matchLabels:
      app: app-service
  template:
    metadata:
      labels:
        app: app-service
    spec:
      containers:
        - name: auth-service
          image: mrsmith9ja/auth-service
          ports:
            - containerPort: 3000
              protocol: TCP
        - name: app-service
          image: mrsmith9ja/app-service
          ports:
            - containerPort: 8000
              protocol: TCP
          env:
            - name: AUTH_SERVICE_URL
              value: "https://auth-service.lbc.verygreenboi.com"
```

Key features:
- Single replica of the deployment
- Two containers in the same pod:
  - auth-service running on port 3000
  - app-service running on port 8000
- Environment variable in app-service to connect to auth-service

#### Services

Two Kubernetes Services expose the application components:

```yaml
# app-service
apiVersion: v1
kind: Service
metadata:
    name: app-service
    namespace: live-bootcamp-proj
spec:
    selector:
        app: app-service
    ports:
      - protocol: TCP
        port: 8000
        targetPort: 8000
    type: ClusterIP

# auth-service
apiVersion: v1
kind: Service
metadata:
    name: auth-service
    namespace: live-bootcamp-proj
spec:
    selector:
        app: app-service
    ports:
      - protocol: TCP
        port: 3000
        targetPort: 3000
    type: ClusterIP
```

Key features:
- Both services are of type ClusterIP (internal access only)
- Both services select pods with the label `app: app-service`
- app-service exposes port 8000
- auth-service exposes port 3000

#### Ingress

An Ingress resource routes external traffic to the services:

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: live-bootcamp-proj-ingress
  namespace: live-bootcamp-proj
  annotations:
    kubernetes.io/tls-acme: "true"
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"
    nginx.ingress.kubernetes.io/backend-protocol: "HTTP"
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/enable-cors: "true"
spec:
  tls:
    - secretName: lbc-verygreenboi.com
      hosts:
        - app-service.lbc.verygreenboi.com
        - auth-service.lbc.verygreenboi.com
  rules:
    - host: app-service.lbc.verygreenboi.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: app-service
                port:
                  number: 8000
    - host: auth-service.lbc.verygreenboi.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: auth-service
                port:
                  number: 3000
```

Key features:
- Uses NGINX ingress controller
- Automatic TLS certificate management with Let's Encrypt
- Forces SSL/TLS redirection
- Enables CORS
- Routes traffic to:
  - app-service.lbc.verygreenboi.com → app-service on port 8000
  - auth-service.lbc.verygreenboi.com → auth-service on port 3000

### CI/CD Process

The application is deployed using a GitHub Actions workflow that handles versioning, building, and deploying:

#### Versioning

A unique version tag is generated for each deployment using the format:
```
YYYYMMDD-git_short_hash
```

For example: `20230428-a1b2c3d`

#### Building

1. The workflow builds and tests both app-service and auth-service using Cargo
2. Docker images are built using Docker Buildx for multi-platform support
3. Images are tagged with both `latest` and the version-specific tag
4. Images are pushed to Docker Hub:
   - mrsmith9ja/app-service:latest
   - mrsmith9ja/app-service:[VERSION]
   - mrsmith9ja/auth-service:latest
   - mrsmith9ja/auth-service:[VERSION]

#### Deploying

1. The Kubernetes deployment YAML is updated with the specific version tags
2. Kubernetes resources are applied in the following order:
   - namespace.yml
   - services.yml
   - app-service.deployment.yml
   - ingresses.yml

This ensures that the application is deployed with the correct version and all necessary resources are created in the proper order.
