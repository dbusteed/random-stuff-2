another cron monitoring system (POC)

i was struggling to get a service account email in my organization which was preventing me from using Healthchecks, so I started to blueprint a smaller alerting system that would notify users thru MS Teams...but then i ended up getting that email address and just used Healthchecks.

my plan was as follows:
- the jobs/checks/tasks are listed in `checks.yaml`. the "group" and "name" need to be unique. users would edit this file directly to add a check. There isn't really a UI for this, it'd be a lot more hands on server work
- a flask API would listen for any requests (similar to Healthchecks) and update some local db
- another script would run every so often and check the local db and the `checks.yaml` to see if it needs to send any alerts
