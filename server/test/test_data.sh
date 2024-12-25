# curl --trace-ascii request_two_files.txt -F title=binary_file_title -F description=binary_file_description -F data=@911.jpg -F data=@912.jpg http://localhost:8000/api/ride_data
curl -F title=binary_file_title -F description=binary_file_description -F data=@911.jpg -F data=@912.jpg http://localhost:8000/api/ride_data
