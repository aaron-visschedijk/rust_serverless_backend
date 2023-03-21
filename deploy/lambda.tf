resource "aws_lambda_function" "backend_api" {
  function_name = "api"
  s3_bucket     = aws_s3_bucket.lambda_bucket.id
  s3_key        = "rust_serverless_backend/bootstrap.zip"

  handler = "bootstrap"
  runtime = "provided.al2"

  role = aws_iam_role.lambda_role.arn
}

resource "aws_lambda_function_url" "backend_api_url" {
  function_name      = aws_lambda_function.backend_api.function_name
  authorization_type = "NONE"

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["*"]
    allow_headers     = ["*"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}
