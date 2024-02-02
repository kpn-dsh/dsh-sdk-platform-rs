#!/bin/bash

echo "fullImageName: $1"
echo "USER_ID: $7"
echo "GROUP_ID: $7"

# Reading the original service_config JSON
cfgJson=$(cat service_config.json)
echo "Original service configuration JSON:"
echo "$cfgJson"

# Modifying the JSON with jq
serviceConfigJson=$(jq \
  --arg img "$1" \
  --arg uid "$7" \
  --arg gid "$7" \
  --arg topic "$8" \
  '.image = $img | .user = ($uid + ":" + $gid)' <<< "$cfgJson")

echo "Modified service configuration JSON:"
echo "$serviceConfigJson"

# Escaping newlines for Azure DevOps variable compatibility
serviceConfigJsonEscaped=$(echo "$serviceConfigJson" | tr '\n' ' ')

echo "serviceConfigJsonEscaped: $serviceConfigJsonEscaped"

# Getting Token
response=$(curl -s -X POST "$2" \
  --header 'Content-type: application/x-www-form-urlencoded' \
  --data-urlencode 'grant_type=client_credentials' \
  --data-urlencode "client_id=$3" \
  --data-urlencode "client_secret=$4")

AUTH_TOKEN=""

if echo "$response" | jq -e .access_token >/dev/null 2>&1; then
    AUTH_TOKEN="Bearer $(echo $response | jq -r '.access_token')"
else
    echo "Error: Response is not a valid JSON or missing access token."
    echo "$response"
fi

echo "AUTH_TOKEN: $AUTH_TOKEN"

# Deploy Service Configuration
deploy_response=$(curl -s -X PUT \
  "$5/$6/application/dsh-sdk-example/configuration" \
  -H "accept: */*" \
  -H "Authorization: $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d "$serviceConfigJsonEscaped")

# Check for success or failure
if echo "$deploy_response" | jq . >/dev/null 2>&1; then
    echo "Service configuration deployed successfully."
else
    echo "Error deploying service configuration."
    echo "$deploy_response"
    exit 1
fi