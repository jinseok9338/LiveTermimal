from deck import Deck_and_hand
class Hand(Deck_and_hand):

    def _init__(self):
        Deck_and_hand.__init__(self)

    def add_hand(self,num,hand):
        for _ in range(num):
            card = self.draw_hand()
            hand.append(card)
            self.deck.remove(card)

    def make_graph(self):
        graph = {}
        self.opponent_hand.append(self.neutral_hand)
        for card in self.opponent_hand:
            list_in_graph = []
            for compare_card in self.opponent_hand:
                if card[1] == compare_card[1] or card[0] == compare_card[0]:
                    list_in_graph.append(compare_card)
            graph.update({card:list_in_graph})

        return graph



    def find_from_the_shortest_to_longest_path(self,graph, start, seen=None, path=None):
        if seen is None: seen = []
        if path is None: path = [start]

        seen.append(start)

        paths = []
        for t in graph[start]:
            if t not in seen:
                t_path = path + [t]
                paths.append(tuple(t_path))
                paths.extend(self.find_from_the_shortest_to_longest_path(graph, t, seen[:], t_path))
        return paths

    def my_turn(self):
        suit, number = input("Type like this : Heart 4 Don't type A,King,Queen,Jack They are 1,13,12,11 Or Draw").split(" ")
        if suit.upper() =="DRAW":
            self.add_hand(1,self.my_hand)
        elif (suit,number) not in self.deck:
            print("Invalid Input type Again")
            self.my_turn()

        else:
            self.my_hand.remove((suit,number))
            self.neutral_hand = (suit,number)
            self.my_turn()

    def your_turn(self):
        graph = self.make_graph()
        all_path = self.find_from_the_shortest_to_longest_path(graph,self.neutral_hand)
        all_path.sort(key=lambda x: len(x),reverse=True)
        longest_path = all_path[0]
        self.neutral_hand = longest_path[-1]
        for card in longest_path:
            self.opponent_hand.remove(card)
        self.add_hand(1,self.opponent_hand)


















