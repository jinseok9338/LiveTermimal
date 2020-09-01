from deck import Deck_and_hand
class Opponent(Deck_and_hand):

    def _init__(self):
        Deck_and_hand.__init__(self)

    def add_hand(self,num):
        card = self.draw_hand()
        for _ in range(num):
            self.opponent_hand.append(card)


    def make_graph(self):
        graph = {}
        self.opponent_hand.append(self.neutral_hand)
        for card in self.opponent_hand:
            list_in_graph = []
            for compare_card in self.opponent_hand:
                if card.value == compare_card.value or card.suit == compare_card.suit:
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









