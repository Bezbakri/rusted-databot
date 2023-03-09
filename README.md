# rusted-databot
# Discord bot that gets some server stats.
# Group Name: Oxidation
Group member names and NetIDs
- Andrew Loh acloh2
- Christopher Chan zychan2
- Rishabh Bezbarua rb33
- Oliver Rogalski oliverr3

Project Introduction
We chose this project because Discord is something that everyone uses and we wanted to incorporate insightful statistics/visualizations that users can see about their servers.
Discord bot that will provide server specific statistics about:
- Member count
  - % Change
  - Absolute change
- Message count
  - % Change
  - Absolute change
  - Which Channel has most Messages
- Voice hours
  - % Change
  - Absolute change
  - Which Channel has most hours
- Data Representation
  - Ranking boards
  - Displays specific users and their score for respective component
  - Graphs
  - Bar/Line of Component against time

# Technical Overview
- First checkpoint: gather and process data from discord
  - Accessing Server Data
  - Messages
  - Voice Hours
  - Member count
  - Server Data processing
  - Statistical calculations
- Second checkpoint: Implement the discord bot commands
  - Data representation (graphs, tables etc.)
  - Interaction with Bot using Commands
  
# Possible Challenges
Gathering data from the server continuously
Working with plotting the data using visualization crates
# References
(https://docs.statbot.net/)
