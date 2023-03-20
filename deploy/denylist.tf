resource "aws_dynamodb_table" "jwt_denylist" {
  name = "jwt_denylist"
  hash_key = "jwt"
  range_key = "exp"
  read_capacity = 1
  write_capacity = 1
  
  attribute {
    name = "jwt"
    type = "S"
  }

  attribute {
    name = "exp"
    type = "N"
  }

  ttl {
    attribute_name = "exp"
    enabled = true
  }
}