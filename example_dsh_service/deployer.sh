#!/bin/bash

echo "Deploying service to DSH"

TOKEN_URL=$(case $2 in 
  dsh-dev) echo "https://auth.lz.lz-cp.dsh.np.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token";; 
  poc-dsh) echo "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token";; 
  *) echo "";; 
esac)

if [ -z "$TOKEN_URL" ]; then
  echo "Error: Undefined platform: $2. Please provide a valid platform."
  exit 1
fi

API_URL=$(case $2 in 
  dsh-dev) echo "https://api.dsh-dev.dsh.np.aws.kpn.com";; 
  poc-dsh) echo "https://api.poc-dsh.dsh.np.aws.kpn.com";; 
  *) echo "";; 
esac)

if [ -z "$API_URL" ]; then
  echo "Error: Undefined platform: $2. Please provide a valid platform."
  exit 1
fi

CONSOLE_URL=$(case $2 in 
  dsh-dev) echo "https://console.dsh-dev.dsh.np.aws.kpn.com/";; 
  poc-dsh) echo "https://console.poc.kpn-dsh.com/";; 
  *) echo "";; 
esac)

if [ -z "$CONSOLE_URL" ]; then
  echo "Error: Undefined platform: $2. Please provide a valid platform."
  exit 1
fi

FULL_IMAGE_NAME=$1
PLATFORM=$2
TENANT_NAME=$3 
TENANTUID=$4
TOPIC=$5
CLIENT_ID=$6
CLIENT_SECRET=$7
ENDPOINT="resources/v0/allocation/$TENANT_NAME/application/dsh-sdk-example/configuration"
DEPLOY_URL="$API_URL/$ENDPOINT"
FULL_CONSOLE_URL="$CONSOLE_URL/#/profiles/$TENANT_NAME/services/dsh-sdk-example/service"

if [ -z "$TOPIC" ]; then
  echo "Error: TOPIC is not provided or empty."
  exit 1
fi
if [ -z "$CLIENT_ID" ]; then
  echo "Error: CLIENT_ID is not provided or empty."
  exit 1
fi
if [ -z "$CLIENT_SECRET" ]; then
  echo "Error: CLIENT_SECRET is not provided or empty."
  exit 1
fi


echo Configuring service with the following parameters:
echo ""
echo "  fullImageName: $FULL_IMAGE_NAME"
echo "  tokenUrl: $TOKEN_URL"
echo "  deployUrl: $DEPLOY_URL"
echo "  tenantName: $TENANT_NAME"
echo "  clientId: $CLIENT_ID"
echo "  apiUrl: $API_URL"
echo "  USER_ID: $TENANTUID"
echo "  GROUP_ID: $TENANTUID"
echo "  topic: $TOPIC"
echo ""

# Reading the original service_config JSON
cfgJson=$(cat service_config.json)

# Modifying the JSON with jq
serviceConfigJson=$(jq \
  --arg img "$FULL_IMAGE_NAME" \
  --arg uid "$TENANTUID" \
  --arg gid "$TENANTUID" \
  --arg topics "$TOPIC" \
  '.image = $img | .user = ($uid + ":" + $gid) | .env.TOPICS = $topics' <<< "$cfgJson")

echo "To be deployed service configuration JSON:"
echo
echo "$serviceConfigJson"
echo

# Escaping newlines for Azure DevOps variable compatibility
serviceConfigJsonEscaped=$(echo "$serviceConfigJson" | tr '\n' ' ')

echo "Requesting access token from $TOKEN_URL"
response=$(curl -s -X POST "$TOKEN_URL" \
  --header 'Content-type: application/x-www-form-urlencoded' \
  --data-urlencode 'grant_type=client_credentials' \
  --data-urlencode "client_id=$CLIENT_ID" \
  --data-urlencode "client_secret=$CLIENT_SECRET")


AUTH_TOKEN=""


if echo "$response" | jq -e .access_token >/dev/null 2>&1; then
    AUTH_TOKEN="Bearer $(echo $response | jq -r '.access_token')"
    echo "Access token received successfully"
else
    echo "Error: Response is not a valid JSON or missing access token."
    echo "$response"
    exit 1
fi


# Deploy Service Configuration
deploy_response=$(curl -s -w "%{http_code}" -X PUT \
  $DEPLOY_URL \
  -H "accept: */*" \
  -H "Authorization: $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d "$serviceConfigJsonEscaped")

# Extract the HTTP status code
status_code=$(echo "$deploy_response" | tail -n1)

# Check for success or failure based on the status code
if [ "$status_code" = "202" ] ; then
    echo "HTTP Status Code: $status_code"
    echo "Service configuration started successfully on DSH."
    echo "See your service at $FULL_CONSOLE_URL"

else
    echo "Error deploying service configuration. HTTP Status Code: $status_code"
    echo "$deploy_response" | sed '$d'  # Remove the status code from the output
    exit 1
fi