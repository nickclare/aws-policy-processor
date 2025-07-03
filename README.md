# AWS Policy Processor

A utility that performs a number of preprocessing and validation steps on AWS "IAM-like" policy documents. This includes:

- IAM policies
- Resource policies
- Service and Resource control policies

Initially the goal is to re-implement AWS's [service-control-policy-preprocessor](https://github.com/aws-samples/service-control-policy-preprocessor)
example scripts, but also to expand the scope to other IAM-like policies, and potentially add some additional functionality 
