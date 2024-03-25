# teletxt
Telegram bot that uses the [netxt](https://github.com/gmelodie/netxt) format to save your Day's todo to a .txt file.

## Workflow
This is how you use the bot to get shit done:

1. Get it running (see `Hosting` section below)
2. Open a chat with it
3. For every day that you want, write your tasks (you don't need to rewrite every day) following the example:
```
[2024-12-31]
- this is a task with no section

Interview
- interview candidate A
- write review on candidate B

Groceries
- tomatoes
- lettuce

Done
- this is a task that is done
```
Obs: check `spec.md` in netxt for the full details on the grammar.

## Hosting
Because the bot needs to see your todo list, I've decided to not make it a service. Instead, you can run it for yourself and for people that trust you to hold their todo files.

1. Talk to [@BotFather](https://t.me/botfather) on Telegram to create a new bot. Save the `token` it gives you.
2. Export the token so the app sees it.
```bash
export TELOXIDE_TOKEN=YOUR_BOTS_TOKEN
```
3. Run the bot.
```bash
cargo run
```

