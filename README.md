# Zombie Dice Game on RUST

Welcome to Zombie Dice Game!

Here are some important commands and notes before you play... To set the game at the begining use the following commands: To start a new game -> 'new' (write thе commands without any quotes) To quit -> 'quit'. 
Then add the number of players and their names.
You can choose a certain player to be an AI by writing 'ai' after the name (e.g. Mark ai).

After setting the game, use the following commands: To roll the dices and then continue your move write 'roll', to stop your move - 'stop'. There are helful messages and notes in the gameplay that help you decide which command to use.

Note that if you have rolled 👣 on some dices they will be included in your next roll if you decide to continue your move by using the command 'roll'. The dices to roll will again be 3 as dices from the dice cup will be added if necessary.

If there are less than 3 dices in the dice cup, it will be refilled. The player which has the most brains and those brains are >= 13 wins. If there are two or more players with the same result, the player who first reached the number of brains wins.

How to run:
The game can be run by the command 'cargo run' after the folder of the project is opened in the terminal.

Testing:
Some unit tests are available by the command 'cargo test'.