from collections import namedtuple
import random

class Deck_and_hand:
    def __init__(self):
        self.card = namedtuple('Card', ['value', 'suit'])
        self.suits = ['hearts', 'diamonds', 'spades', 'clubs']
        self.deck = [self.card(value, suit) for value in range(1, 14) for suit in self.suits]
        self.my_hand = []
        self.opponent_hand = []
        self.neutral_hand = self.draw_hand()

    def draw_hand(self):
        card = random.choice(self.deck)
        return card


