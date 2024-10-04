# How to install?
1. **Download the dependencies.**
> The only dependency is rust.  
> On a Unix-like OS (Linux and Mac) just type `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` in terminal.  
> For other OS, search for it.  

2. **Create a Discord bot.**
> Go to [Discord developer portal](https://discord.com/developers/applications).  
> Log in with your account.  
> Click on "New Application  
> Choose a bot name and team.  
> In the "Bot" menu, tick all the "Privileged Gateway Intents".  
> This should be enough, personalise as you wish.  

3. **Invite your bot to your server.**
> In the "Installation" menu, in the "Install Link" section, click on the link.  
> This will take you to a discord page, choose your server and its done!  

4. **Download the source code.**  
> Download it from github.  
> Unzip it if you have to.  

5. **Build the source.**
> into the source file (must contain 'cargo.toml')  
> Type `cargo build', then press enter and wait for the build.  

6. **Run the bot.**
> In the "bot" menu, take your bot token from the "Token" section.  
> For security reasons, tokens can only be viewed once, when they are created. If you forget or lose access to your token, you will need to create a new one.  
> Type `DISCORD_TOKEN="Your token" ./target/debug/rphellper`.  
> Your bot is now ready to run!  

# How RPHellper works?