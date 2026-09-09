resource "aws_lambda_function" "legacy_export" {
  function_name = "legacy-export"
  role          = aws_iam_role.legacy_export.arn
  handler       = "legacy.handler"
  runtime       = "python3.12"
}
