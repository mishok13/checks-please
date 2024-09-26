An ad-free alternative way of splitting your common expenses wisely ;)

High level API

POST /auth/login <- because we need to authenticate somehow?
POST /auth/logout <- is it even needed

GET /groups <-- get groups of expenses
GET /expenses <-- get expenses, filter by settlement and groups
POST /expenses <-- add expense
POST /groups <-- create group
PATCH /groups/:id <-- add users, remove users (only group creator or removing self?), mark group as settled/open/archived
