#!/usr/bin/env bash

GRPC_SERVER="localhost:9099"
PROTO_DIR="proto/medpass/records/v1"
PROTO_FILE="$PROTO_DIR/record.proto"

FILE_PATH="scripts/dev/fake_blood_test_analysis.pdf"

# Encode file as base64 for JSON
FILE_DATA_B64=$(base64 -i "$FILE_PATH" | tr -d '\n')

# Create JSON payload with base64 encoded data
read -r -d '' PAYLOAD <<EOF
{
  "record": {
    "type": "RECORD_TYPE_LAB_RESULT",
    "title": "Blood Test Result",
    "subtitle": "Comprehensive blood panel",
    "date": "2024-06-03T18:00:00Z",
    "tags": ["blood", "lab", "test"],
    "facility_name": "Acme Labs",
    "user_id": "0197a7d1-af1b-7448-a304-d80e498bed32",
    "file_data": "$FILE_DATA_B64"
  }
}
EOF

#echo "Payload being sent:"
#echo "$PAYLOAD" | jq .

grpcurl \
  -plaintext \
  -proto "$PROTO_FILE" \
  -import-path . \
  -d "$PAYLOAD" \
  "$GRPC_SERVER" \
  medpass.records.v1.RecordsService/CreateRecord
