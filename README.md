# teletxt
Telegram bot that uses the [netxt](https://github.com/gmelodie/netxt) format to save your `Day`s to a `.txt` file.

## Workflow
This is how you use the bot to get shit done:

1. Get it running (see `Hosting` section below)
2. Open a chat with it
3. For every day that you want, write your tasks, things you did, etc. following the example:
```
[2024-12-30]
Meeting with natalie
- chose color for website
- need to deploy it
Groceries
- garlic bread
- tomatoes
```
Obs: check `spec.md` in netxt for the full details on the grammar.

## Hosting
Because the bot needs to see your list, I've decided to not make it a service. Instead, you can run it for yourself and for people that trust you to hold their todo files.

1. Talk to [@BotFather](https://t.me/botfather) on Telegram to create a new bot. Save the `token` it gives you.
2. Export the token so the app sees it.
```bash
export TELOXIDE_TOKEN=YOUR_BOTS_TOKEN
export ALLOWED_USERS_FILE=/some/directory/allowed-users.txt # optional
export TODO_DIR=/some/directory/todos # optional
```

3. Make sure `allowed-users.txt` (with the users allowed to talk to the bot, including yourself, one per line) and the `todo` directory both exist.
Obs: as of right now, the users have to have usernames.

4. Run the bot.
```bash
RUST_LOG=info cargo run
```

5. Optional: Docker (Compose)

Create `.env` file with the following contents:
```bash
RUST_LOG=info
ALLOWED_USERS_FILE=/some/directory/allowed-users.txt
TODO_DIR=/some/directory/todos
TELOXIDE_TOKEN=YOUR_BOTS_TOKEN
```

Then bring container and volume up
```bash
sudo docker compose up
```

