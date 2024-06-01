#!/bin/bash

read -p "tx count: " count

for (( i = 0; i < $count; i++ ))
do
	RAND_FROM=$(openssl rand -hex 32)
	RAND_TO=$(openssl rand -hex 32)
	RAND_INSTRUCTION=$(openssl rand -hex 32)
	
	curl --location --request PUT "localhost:8080/api/transaction" \
		--header "Content-Type: application/json" \
		--data '{
		   "from": "'$RAND_FROM'",
		   "to": "'$RAND_TO'",
		   "instruction": "'$RAND_INSTRUCTION'"
		}
	}'
done
