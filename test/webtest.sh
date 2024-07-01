#!/bin/bash

####
#Test a variety of our web requests.
#
# GET "/ride/<ride_id>"
echo "Testing GET /ride/1"
curl -s http://localhost:8000/ride/1
echo
echo

echo "Testing POST /ride/"
curl -X POST -H "Content-Type: application/json" -d @test_post.json -s http://localhost:8000/ride/
echo
echo
