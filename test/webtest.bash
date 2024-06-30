#!/bin/bash

####
#Test a variety of our web requests.
#
# GET "/ride/<ride_id>"
echo "Testing /ride/1"
curl -s http://localhost:8000/ride/1
echo
echo
