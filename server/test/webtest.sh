#!/bin/bash

####
#Test a variety of our web requests.
####

# GET "/health" to check if the server is running.
echo "Testing GET /health"
curl -s http://localhost:8000/api/health
echo
echo

# GET "/ride/<ride_id>"
echo "Testing GET /ride/1"
curl -s http://localhost:8000/api/ride/1
echo
echo

# POST "/ride/" a new ride and save it to the DB.
echo "Testing POST /ride/"
curl -X POST -H "Content-Type: application/json" -d @test_post.json -s http://localhost:8000/api/ride/
echo
echo
