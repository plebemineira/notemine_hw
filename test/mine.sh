N_WORKERS=3
DIFFICULTY=32
EVENT_JSON=event.json
RELAY_URL=wss://plebemineira.xyz
NSEC=nsec13ezg388stxfnxe72nc20428r7lrxzst85d60vxynk6zr57lpxu6svjam98

cargo run -- publish \
        --n-workers $N_WORKERS \
        --log-workers \
        --difficulty $DIFFICULTY \
        --event-json $EVENT_JSON \
        --relay-url $RELAY_URL \
        --nsec $NSEC