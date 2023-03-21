resource "aws_iam_role" "lambda_role" {
  name = "lambda_role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Action = "sts:AssumeRole"
      }
    ]
  })
}

resource "aws_iam_policy" "lambda_policy" {
  name = "lambda_policy"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "lambda:InvokeFunction"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "*"
      }
    ]
  })
}

resource "aws_iam_policy" "dynamodb_policy" {
    name = "dynamodb_policy"
    policy = jsonencode({
        Version = "2012-10-17"
        Statement = [
        {
            Effect = "Allow"
            Action = [
            "dynamodb:PutItem",
            "dynamodb:GetItem",
            "dynamodb:UpdateItem",
            "dynamodb:DeleteItem",
            "dynamodb:Scan",
            "dynamodb:Query",
            "dynamodb:BatchGetItem",
            "dynamodb:BatchWriteItem"
            ]
            Resource = "*"
        },
        ]
    })
}

resource "aws_iam_role_policy_attachment" "lambda_policy_attachment" {
    policy_arn = aws_iam_policy.lambda_policy.arn
    role = aws_iam_role.lambda_role.name
}

resource "aws_iam_role_policy_attachment" "dynamodb_policy_attachment" {
    policy_arn = aws_iam_policy.dynamodb_policy.arn
    role = aws_iam_role.lambda_role.name
  
}