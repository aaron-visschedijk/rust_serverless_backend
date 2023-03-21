resource "aws_dynamodb_table" "jwt_denylist" {
  name = "jwt_denylist"
  hash_key = "jwt"
  read_capacity = 1
  write_capacity = 1
  
  attribute {
    name = "jwt"
    type = "S"
  }

  ttl {
    attribute_name = "exp"
    enabled = true
  }
}