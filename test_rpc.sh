echo "test quote method"
curl -s -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "jsonrpc": "2.0",
           "method": "quote",
           "params": {
             "difficulty": 20
           },
           "id": 1
         }' http://localhost:1337 | jq

echo "test mine method"
curl -s -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "jsonrpc": "2.0",
           "method": "mine",
           "params": {
               "event": {
               "pubkey": "98590c0f4959a49f3524b7c009c190798935eeaa50b1232ba74195b419eaa2f2",
               "created_at": 1668680774,
               "kind": 1,
               "tags": [],
               "content": "hello world"
            },
            "difficulty": 15,
            "zap": "f481897ee877321783bb76133622b3cc344d691bb79cd6be88f44e819c3b2306"
           },
           "id": 1
         }' http://localhost:1337 | jq