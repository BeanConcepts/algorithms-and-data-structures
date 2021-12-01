## Hello,

### Let me understand you better by answering this question.

**What is BigO notation for time complexity of the following code? Assuming that we can create unlimited number of Aws VPCs**
```
variable "vpc_names" {
  type = set(string)
  default = ["one", "two", "three", "four", "five", "six"]
}

resource "aws_vpc" "first_set" {
  for_each = var.vpc_names
  cidr_block = "10.0.0.0/16"
}

resource "aws_vpc" "second_set" {
  for_each = var.vpc_names
  cidr_block = "10.0.0.0/16"
}

resource "aws_vpc" "third_set" {
  for_each = var.vpc_names
  cidr_block = "10.0.0.0/16"
}
```
Choose only the best answer.
1. I have no idea what `BigO notation` is or what this code does. [Select](./intro)
2. I understand this is `Terraform code` and it creates 18 different VPCs but what is BigO? [Select](./intro)
3. This is basically a for loop, so it must be O(n) [Select](./intro_3)
4. There are 6 elements in each loop, so `O(6)` is a more accurate answer. [Select](./intro)
5. The loop is repeated 3 times, so `O(18)` is more accurate thatn `O(6)`. [Select](./intro)
6. BigO does not care about the actual number of input elements, so it should be O(n), and because the loop is repeated 3 times, the best answer should be O(3n). [Select](./intro)
7. None of them is correct. [Select](./intro)
