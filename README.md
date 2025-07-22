# Quote Bot

A Discord bot for keeping track of ICSSC Board's various quotes, with support for storage in SQLite and Notion databases.

### Commands
- /quote [who] [quote] [when (optional)]
  - Logs a quote, then stores it in the SQLite db before syncing it to the Notion document. 
- /leaderboard [sort_by]
  - Displays either the users who have been quoted the most or who have quoted others the most 

### Setup
- Configure env variables (see `.env.example`) using your Discord app token, the ID of the Discord guild you want to register slash commands on, and your Notion API key/DB id.
- Configure `user_map.json` to define names of each user and their names (for labelling in the Notion DB).
