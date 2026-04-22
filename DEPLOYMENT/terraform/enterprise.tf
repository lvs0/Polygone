terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  provider "aws" {
    region = var.aws_region
  }

  # Variables
  variable "aws_region" {
    description = "AWS region for deployment"
    type        = string
    default     = "eu-west-3"
  }

  variable "domain_name" {
    description = "Domain name for Polygone deployment"
    type        = string
  }

  variable "admin_email" {
    description = "Admin email for SSL certificates"
    type        = string
  }

  variable "enable_monitoring" {
    description = "Enable monitoring stack"
    type        = bool
    default     = true
  }

  variable "enable_backup" {
    description = "Enable backup system"
    type        = bool
    default     = true
  }

  # VPC Configuration
  resource "aws_vpc" "polygone" {
    cidr_block           = "10.0.0.0/16"
    enable_dns_hostnames = true
    enable_dns_support   = true

    tags = {
      Name        = "polygone-vpc"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Internet Gateway
  resource "aws_internet_gateway" "polygone" {
    vpc_id = aws_vpc.polygone.id

    tags = {
      Name        = "polygone-igw"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Route Tables
  resource "aws_route_table" "public" {
    vpc_id = aws_vpc.polygone.id

    route {
      cidr_block = "0.0.0.0/0"
      gateway_id   = aws_internet_gateway.polygone.id
    }

    tags = {
      Name        = "polygone-public-rt"
      Environment = "production"
      Project     = "polygone"
    }
  }

  resource "aws_route_table_association" "public" {
    subnet_id      = aws_subnet.public[*].id
    route_table_id = aws_route_table.public.id
  }

  # Subnets
  resource "aws_subnet" "public" {
    count                   = 3
    vpc_id                  = aws_vpc.polygone.id
    cidr_block              = "10.0.${count.index + 1}.0/24"
    availability_zone       = "${var.aws_region}${count.index}a"
    map_public_ip          = true

    tags = {
      Name        = "polygone-public-subnet-${count.index}"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Security Group
  resource "aws_security_group" "polygone" {
    name        = "polygone-sg"
    description = "Security group for Polygone Enterprise"
    vpc_id      = aws_vpc.polygone.id

    # Polygone Core UDP ports
    ingress {
      description = "Polygone Core Network"
      from_port   = 4001
      to_port     = 4001
      protocol    = "udp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    ingress {
      description = "Polygone Core Network"
      from_port   = 4002
      to_port     = 4002
      protocol    = "udp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    ingress {
      description = "Polygone Core Network"
      from_port   = 4003
      to_port     = 4003
      protocol    = "udp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # Polygone Petals API
    ingress {
      description = "Polygone Petals API"
      from_port   = 4003
      to_port     = 4003
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    ingress {
      description = "Polygone Petals API"
      from_port   = 4004
      to_port     = 4004
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # Polygone Hide SOCKS5
    ingress {
      description = "Polygone Hide"
      from_port   = 1080
      to_port     = 1080
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # MAX Assistant API
    ingress {
      description = "MAX Assistant API"
      from_port   = 8000
      to_port     = 8000
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # NEXUS Web Interface
    ingress {
      description = "NEXUS Web"
      from_port   = 3000
      to_port     = 3000
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # Monitoring
    ingress {
      description = "Monitoring"
      from_port   = 9090
      to_port     = 9090
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    # Load Balancer
    ingress {
      description = "Load Balancer"
      from_port   = 80
      to_port     = 80
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    ingress {
      description = "Load Balancer HTTPS"
      from_port   = 443
      to_port     = 443
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }

    egress {
      description = "Allow all outbound traffic"
      from_port   = 0
      to_port     = 0
      protocol    = "-1"
      cidr_blocks = ["0.0.0.0/0"]
    }

    tags = {
      Name        = "polygone-sg"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # ECS Cluster
  resource "aws_ecs_cluster" "polygone" {
    name = "polygone"

    setting {
      name  = "containerInsights"
      value = "enabled"
    }

    capacity_providers {
      name = "FARGATE"
    }
  }

  # Task Definitions
  resource "aws_ecs_task_definition" "polygone-core" {
    family                   = "polygone"
    network_mode             = "awsvpc"
    requires_compatibilities     = ["FARGATE"]
    cpu                      = "1024"
    memory                   = "2048"
    execution_role_arn       = aws_iam_role.polygone_task.arn
    container_definitions    = jsonencode([
      {
        name      = "polygone-core"
        image     = "polygone/core:v2.0.0"
        essential = true
        portMappings = [
          {
            containerPort = 4001
            hostPort      = 4001
            protocol      = "udp"
          },
          {
            containerPort = 4002
            hostPort      = 4002
            protocol      = "udp"
          },
          {
            containerPort = 4003
            hostPort      = 4003
            protocol      = "udp"
          }
        ]
        environment = [
          {
            name  = "NODE_ID"
            value = "ecs-${aws_subnet.public[0].id}"
          },
          {
            name  = "BOOTSTRAP_NODES"
            value = "${join(",", aws_instance.polygone.*.private_ip)}"
          },
          {
            name  = "LOG_LEVEL"
            value = "info"
          }
        ]
        logConfiguration = {
          logDriver = "awslogs"
          options = {
            "awslogs-group"         = "/ecs/polygone-core"
            "awslogs-region"        = var.aws_region
            "awslogs-stream-prefix" = "ecs"
          }
        }
      }
    ])
  })

  resource "aws_ecs_task_definition" "polygone-petals" {
    family                   = "polygone"
    network_mode             = "awsvpc"
    requires_compatibilities     = ["FARGATE"]
    cpu                      = "4096"
    memory                   = "8192"
    execution_role_arn       = aws_iam_role.polygone_task.arn
    container_definitions    = jsonencode([
      {
        name      = "polygone-petals"
        image     = "polygone/petals:v2.0.0"
        essential = true
        portMappings = [
          {
            containerPort = 4003
            hostPort      = 4003
            protocol      = "tcp"
          },
          {
            containerPort = 4004
            hostPort      = 4004
            protocol      = "tcp"
          }
        ]
        environment = [
          {
            name  = "LAYERS"
            value = "0-15"
          },
          {
            name  = "MODEL_PATH"
            value = "/models/llama-7b"
          },
          {
            name  = "MAX_CONCURRENT_REQUESTS"
            value = "10"
          },
          {
            name  = "ENABLE_TRAINING"
            value = "true"
          }
        ]
        resourceRequirements = {
          "type" = "GPU"
          "value" = "1"
        }
        logConfiguration = {
          logDriver = "awslogs"
          options = {
            "awslogs-group"         = "/ecs/polygone-petals"
            "awslogs-region"        = var.aws_region
            "awslogs-stream-prefix" = "ecs"
          }
        }
      }
    ])
  })

  # ECS Services
  resource "aws_ecs_service" "polygone-core" {
    name            = "polygone-core"
    cluster         = aws_ecs_cluster.polygone.id
    task_definition = aws_ecs_task_definition.polygone-core.arn
    desired_count   = 3
    launch_type     = "FARGATE"

    network_configuration {
      subnets         = aws_subnet.public[*].id
      security_groups  = [aws_security_group.polygone.id]
      assign_public_ip = true
    }

    load_balancer {
      target_group_arn = aws_lb_target_group.polygone-core.arn
      container_name    = "polygone-core"
      container_port   = 4000
    }

    depends_on = [aws_lb_target_group.polygone-core]
  }

  resource "aws_ecs_service" "polygone-petals" {
    name            = "polygone-petals"
    cluster         = aws_ecs_cluster.polygone.id
    task_definition = aws_ecs_task_definition.polygone-petals.arn
    desired_count   = 2
    launch_type     = "FARGATE"

    network_configuration {
      subnets         = aws_subnet.public[*].id
      security_groups  = [aws_security_group.polygone.id]
      assign_public_ip = true
    }

    load_balancer {
      target_group_arn = aws_lb_target_group.polygone-petals.arn
      container_name    = "polygone-petals"
      container_port   = 4003
    }

    depends_on = [aws_lb_target_group.polygone-petals]
  }

  # Load Balancer
  resource "aws_lb" "polygone" {
    name               = "polygone-lb"
    internal           = false
    load_balancer_type = "application"
    security_groups     = [aws_security_group.polygone.id]
    subnets            = aws_subnet.public[*].id

    enable_deletion_protection = false

    tags = {
      Name        = "polygone-lb"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Target Groups
  resource "aws_lb_target_group" "polygone-core" {
    name     = "polygone-core-tg"
    port     = 4000
    protocol = "HTTP"
    vpc_id   = aws_vpc.polygone.id

    health_check {
      enabled = true
      path    = "/health"
      matcher = "200"
    }

    tags = {
      Name        = "polygone-core-tg"
      Environment = "production"
      Project     = "polygone"
    }
  }

  resource "aws_lb_target_group" "polygone-petals" {
    name     = "polygone-petals-tg"
    port     = 4003
    protocol = "HTTP"
    vpc_id   = aws_vpc.polygone.id

    health_check {
      enabled = true
      path    = "/health"
      matcher = "200"
    }

    tags = {
      Name        = "polygone-petals-tg"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Listener Rules
  resource "aws_lb_listener" "polygone-core" {
    load_balancer_arn = aws_lb.polygone.arn
    port              = "4000"
    protocol          = "HTTP"

    default_action {
      type             = "forward"
      target_group_arn = aws_lb_target_group.polygone-core.arn
    }
  }

  resource "aws_lb_listener" "polygone-petals" {
    load_balancer_arn = aws_lb.polygone.arn
    port              = "4003"
    protocol          = "HTTP"

    default_action {
      type             = "forward"
      target_group_arn = aws_lb_target_group.polygone-petals.arn
    }
  }

  resource "aws_lb_listener" "polygone-https" {
    load_balancer_arn = aws_lb.polygone.arn
    port              = "443"
    protocol          = "HTTPS"

    certificate_arn   = aws_acm_certificate.polygone.arn
    default_action {
      type             = "forward"
      target_group_arn = aws_lb_target_group.polygone-core.arn
    }
  }

  # SSL Certificate
  resource "aws_acm_certificate" "polygone" {
    domain_name       = var.domain_name
    validation_method = "DNS"

    tags = {
      Name        = "polygone-ssl"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Route 53 DNS
  resource "aws_route53_zone" "polygone" {
    name = var.domain_name

    tags = {
      Name        = "polygone-dns"
      Environment = "production"
      Project     = "polygone"
    }
  }

  resource "aws_route53_record" "polygone-a" {
    zone_id = aws_route53_zone.polygone.id
    name    = "api"
    type    = "A"

    alias {
      name                   = aws_lb.polygone.dns_name
      zone_id                = aws_lb.polygone.zone_id
      evaluate_target_health = true
    }
  }

  resource "aws_route53_record" "polygone-cname" {
    zone_id = aws_route53_zone.polygone.id
    name    = "petals"
    type    = "CNAME"

    ttl     = 300
    records = [aws_route53_record.polygone-a.fqdn]
  }

  resource "aws_route53_record" "polygone-hide" {
    zone_id = aws_route53_zone.polygone.id
    name    = "hide"
    type    = "CNAME"

    ttl     = 300
    records = [aws_route53_record.polygone-a.fqdn]
  }

  # IAM Role
  resource "aws_iam_role" "polygone_task" {
    name = "polygone-task-role"

    assume_role_policy = jsonencode({
      Version = "2012-10-17",
      Statement = [
        {
          Effect = "Allow",
          Principal = {
            Service = "ecs-tasks.amazonaws.com"
          },
          Action = [
            "logs:CreateLogGroup",
            "logs:CreateLogStream",
            "logs:PutLogEvents"
          ],
          Resource = "arn:aws:logs:*:*"
        }
      ]
    })

    tags = {
      Name        = "polygone-task-role"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # CloudWatch Log Groups
  resource "aws_cloudwatch_log_group" "polygone-core" {
    name = "/ecs/polygone-core"

    retention_in_days = 30

    tags = {
      Name        = "polygone-core-logs"
      Environment = "production"
      Project     = "polygone"
    }
  }

  resource "aws_cloudwatch_log_group" "polygone-petals" {
    name = "/ecs/polygone-petals"

    retention_in_days = 30

    tags = {
      Name        = "polygone-petals-logs"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # S3 Buckets
  resource "aws_s3_bucket" "polygone-data" {
    bucket = "polygone-data-${var.domain_name}"

    acl    = "private"

    versioning {
      enabled = true
    }

    server_side_encryption_configuration {
      rule {
        apply_server_side_encryption_by_default = true
        sse_algorithm = "AES256"
      }
    }

    tags = {
      Name        = "polygone-data-bucket"
      Environment = "production"
      Project     = "polygone"
    }
  }

  resource "aws_s3_bucket" "polygone-backups" {
    bucket = "polygone-backups-${var.domain_name}"

    acl    = "private"

    lifecycle_rule {
      id     = "backup_retention"
      enabled = true

      transition {
        days          = 30
        storage_class = "STANDARD_IA"
      }

      transition {
        days          = 90
        storage_class = "GLACIER"
      }

      expiration {
        days = 2555
      }
    }

    tags = {
      Name        = "polygone-backups-bucket"
      Environment = "production"
      Project     = "polygone"
    }
  }

  # Outputs
  output "load_balancer_dns" {
    description = "DNS name of the load balancer"
    value       = aws_lb.polygone.dns_name
  }

  output "api_endpoint" {
    description = "API endpoint URL"
    value       = "https://${aws_route53_record.polygone-a.fqdn}"
  }

  output "petals_endpoint" {
    description = "Petals endpoint URL"
    value       = "https://${aws_route53_record.polygone-cname.fqdn}"
  }

  output "hide_endpoint" {
    description = "Hide endpoint URL"
    value       = "socks5://${aws_route53_record.polygone-hide.fqdn}:1080"
  }

  output "s3_bucket_name" {
    description = "S3 bucket for data storage"
    value       = aws_s3_bucket.polygone-data.id
  }

  output "backup_bucket_name" {
    description = "S3 bucket for backups"
    value       = aws_s3_bucket.polygone-backups.id
  }
}
