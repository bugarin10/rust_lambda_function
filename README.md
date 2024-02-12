# Rust lambda function in AWS

In this repo two lambda Rust function were built.
In the first lambda function, the one which was deployed in AWS filter and count the amount of players that each Football clubs in MLS that earn more than ```N``` quantity.
The data is from from the [MLS](https://mlsplayers.org/resources/salary-guide).

An AIM role was created with the following permissions:

AWSLambdaBasicExecutionRole
AmazonAPIGatewayAdministrator
AmazonAPIGatewayInvokeFullAccess
AmazonAPIGatewayPushToCloudWatchLogs
AmazonS3ObjectLambdaExecutionRolePolicy
AWSCodeDeployRoleForLambda
AWSLambda_FullAccess
AWSLambdaDynamoDBExecutionRole
AWSLambdaInvocation-DynamoDB
CloudWatchLambdaInsightsExecutionRolePolicy
CloudWatchLogsFullAccess

This is the result using the URL to know the amount of players earning more than 700k USD per year an AWS CLI:

<img src="https://github.com/bugarin10/rust_lambda_function/blob/main/static/filtering_running.png" alt="AWS CLI">

The second one was constructed to have hands on Rust coding learning from basics. Thus the idea was to follow the Algorithms book by Cormen et al. and make and insertion-sort following the next pseudocode:


>1. For i from 2 to n
>2.   Key = A[i]
>3.   j = i - 1
>4.   While j > 0 and A[j] > Key
>5.     A[j+1] = A[j]
>6.     j = j - 1
>7.   End While
>8.   A[j+1] = Key
>9. End For


[Image]


[Image]


