group "default" {
  targets = ["app-service", "auth-service"]
}

target "app-service" {
  context = "./app-service"
  tags = []
}

target "auth-service" {
  context = "./auth-service"
  tags = []
}