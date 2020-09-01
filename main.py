from hand import Hand
from pprint import pprint

game = Hand()
game.add_hand(6,game.my_hand)
game.add_hand(6,game.opponent_hand)


while len(game.deck) !=0 or len(game.opponent_hand) !=0 or len(game.my_hand) !=0:
    game.my_turn()
    game.your_turn()

if len(game.opponent_hand) > len(game.my_hand):
    print("You win")
elif len(game.opponent_hand) < len(game.my_hand):
    print("You lost")
else:
    print("Draw")

