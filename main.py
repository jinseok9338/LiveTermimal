from opponent_hand import Opponent
from pprint import pprint

opponent = Opponent()
opponent.add_hand(6)
print(opponent.opponent_hand)
print(opponent.neutral_hand)
graph = opponent.make_graph()
paths = opponent.find_from_the_shortest_to_longest_path(graph,opponent.neutral_hand)
pprint(graph)
pprint(paths)

