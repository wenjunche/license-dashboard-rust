# Deployment Guide

## Prerequisites

1. **AWS Account** with ECR repository created
2. **GitHub Secrets** configured:
   - `AWS_ACCESS_KEY_ID`
   - `AWS_SECRET_ACCESS_KEY`

## Setup AWS ECR

```bash
# Create ECR repository
aws ecr create-repository \
    --repository-name license-dashboard \
    --region us-east-1

# Get repository URI
aws ecr describe-repositories \
    --repository-names license-dashboard \
    --region us-east-1
```

## Configure GitHub Secrets

1. Go to your GitHub repository
2. Settings → Secrets and variables → Actions
3. Add the following secrets:
   - `AWS_ACCESS_KEY_ID`: Your AWS access key
   - `AWS_SECRET_ACCESS_KEY`: Your AWS secret key

## Deploy

1. Go to Actions tab in GitHub
2. Select "Manual Deploy to AWS"
3. Click "Run workflow"
4. Choose environment (staging/production)
5. Click "Run workflow"

## Docker Image Tags

Each build creates three tags:
- `<commit-sha>` - Specific version
- `latest` - Latest build
- `staging` or `production` - Environment-specific

## Local Docker Build

```bash
# Build image locally
docker build -t license-dashboard .

# Run container
docker run -p 3000:3000 \
    -e DATABASE_URL=postgresql://user:pass@host:5432/db \
    license-dashboard
```

## Environment Variables

Required environment variables in production:

```bash
DATABASE_URL=postgresql://user:password@host:5432/database
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=https://your-domain.com/auth/callback
JWT_SECRET=your-secret-key
RUST_LOG=info
HOST=0.0.0.0
PORT=3000
```

## ECS Deployment (Example)

After pushing to ECR, deploy to ECS:

```bash
# Update ECS service with new image
aws ecs update-service \
    --cluster your-cluster \
    --service license-dashboard \
    --force-new-deployment \
    --region us-east-1
```
