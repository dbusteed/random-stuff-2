#!/bin/bash

#   creates a CSV of memory statistics for the system
#   and other processes. for example, replace "grep other"
#   with "grep my_process" to get memory usage stats.
#   best used over a long period of time, then you can
#   visualize the trends and stuff

echo "time,used_mem,other" > mem.csv

while true: do
	
	time=$(date +%R)
	
	printf "[ $time ] gathering memory stats\n"
	
	used_mem=$(free -m | grep "Mem" | awk '{print $3}')
	
	other=$(ps -o pid,%mem,command ax | grep other | grep -v grep | awk '{print $2}')

	echo "$time,$used_mem,$other" >> mem.csv

	sleep 60
done
