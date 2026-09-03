# Source: https://math.stackexchange.com/questions/2078609/whats-the-fairest-turn-sequence-for-n-players & https://creators.spotify.com/pod/profile/a-problem-squared/episodes/140--ABBABAAB-and-AI-Bah-e3n15fs


import functools
from itertools import count





@functools.lru_cache(maxsize=None)
def binomial_coeff(n, k):

    if n < 0:
        return 0
    if k > n // 2:
        return binomial_coeff(n, n - k)
    if k < 0:
        return 0
    if k == 0:
        return 1
    if k == 1:
        return n
    return binomial_coeff(n-1, k-1) + binomial_coeff(n-1, k)


def turns(player_count):
    # first n turns are always just the the players in standard order
    sequence = [turn for turn in range(player_count)]
    yield from sequence
    # setting up state to where it would be after first n turns plus some house keeping
    players = [[1, 0] for _ in range(player_count)]
    column_turn_number = [player_count, 1]
    column = 1
    sign = -1
    # once that's done, for every turn,
    for turn_number in count(player_count):
        # start with updating the current column to the most recent state
        for turn in range(column_turn_number[column], turn_number):
            players[sequence[turn]][column] += sign * binomial_coeff(turn, column)
            column_turn_number[column] += 1
        # then search for all occurrences of the minimum value in that column
        target_column = [player[column] for player in players]
        min_in_column = min(target_column)
        duplicates = [duplicate[0] for duplicate in
                      filter(lambda x: x[1] == min_in_column, enumerate(target_column))]
        # if there are more than one, then
        while len(duplicates) > 1:
            column += 1  # move to the next column and update it
            sign *= -1  # and update the sign accordingly
            try:
                for turn in range(column_turn_number[column], turn_number):
                    players[sequence[turn]][column] += sign * binomial_coeff(turn, column)
                    column_turn_number[column] += 1
            except IndexError:  # if it doesn't exist yet, just add a new column and try again
                for player in players:
                    player.append(0)
                    column_turn_number.append(column)
                for turn in range(column_turn_number[column], turn_number):
                    players[sequence[turn]][column] += sign * binomial_coeff(turn, column)
                    column_turn_number[column] += 1
            target_column = [players[index][column] for index in duplicates]
            min_in_column = min(target_column)
            duplicates = [duplicate[0] for duplicate in
                          filter(lambda x: x[1] == min_in_column, zip(duplicates, target_column))]
        turn = duplicates[0]  # once all duplicates are gone, the remaining value is your next turn
        yield turn  # return the turn,
        sequence.append(turn)  # store it for future reference, and
        column, sign = 0, 1  # reset the column and sign
