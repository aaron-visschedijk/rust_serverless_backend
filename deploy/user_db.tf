resource "aws_dynamodb_table" "users" {
  name = "users"
  hash_key = "id"
  read_capacity = 1
  write_capacity = 1
  
  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "email"
    type = "S"
  }

  global_secondary_index {
    name = "email"
    hash_key = "email"
    projection_type = "ALL"
    write_capacity = 1
    read_capacity = 1
  }
}