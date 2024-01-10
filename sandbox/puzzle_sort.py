
subsequences = [
    ["queso", "papel", "gato"],
    ["refri", "arbol", "silla"],
    ["mesa", "papel", "sopa"],
    ["refri", "ensalada", "mesa", "perro"],
    ["papel", "arbol", "silla", "perro"],
]

print(subsequences)

class Word:
    """Stores a word's dependencies and other info."""

    name: str
    visited: bool
    left: set[str]
    right: set[str]

    def __init__(self, name: str):
        self.name = name
        self.visited = False
        self.left = set()
        self.right = set()

    def __repr__(self) -> str:
        #return "Word()"
        #return f"\n<WORD {self.name},\n\tLEFT: {self.left.__str__()},\n\tRIGHT: {self.right.__str__()}>\n\tLEFT_LABELS: {self.left_labels}\n\tRIGHT_LABELS: {self.right_labels}\n" 
        return f"\n<WORD {self.name},\n\tLEFT: {self.left.__str__()},\n\tRIGHT: {self.right.__str__()}>\n" 

    #def __str__(self) -> str:
        #return self.left.__str__() + self.right.__str__()
        #return "a"

all_words: dict[str, Word] = dict()

# create a Word object for each word and build its dependencies
for subseq in subsequences:
    # iterate through all words in this subsequence
    for i in range(len(subseq)):
        if subseq[i] not in all_words: # check if we haven't created this Word yet
            all_words[subseq[i]] = Word(subseq[i])
        if i != 0: # only push left dependency if we aren't on left edge
            all_words[subseq[i]].left.add(subseq[i-1])
        if i != len(subseq)-1: # only push right dependency if we aren't on right edge
            all_words[subseq[i]].right.add(subseq[i+1])

print(all_words)


def build(all_words: dict[str, Word]) -> None | list[str]:
    # Kahn's Algorithm for topological sort

    stack = [] # Stack of words with their left words all visited (or with no left words)

    # Put all words with no left words into the stack
    for name in all_words:
        word = all_words[name]
        has_left = False
        for left_name in word.left:
            if not all_words[left_name].visited:
                has_left = True
        if not has_left:
            stack.append(name)

    print(stack)

    result = []

    while stack:
        cur = stack.pop()
        word = all_words[cur]
        if word.visited: 
            # If word is already in results, skip it
            continue
    
        result.append(cur)
        word.visited = True
        
        # We do not need to search the entire list of words
        # For any word w, the right contains every word that has w to its left
        for right_name in word.right:
            # Remove current word from the left of all other words if it exists
            all_words[right_name].left.discard(cur)

            # Add to stack if no more left words
            if not all_words[right_name].left:
                stack.append(right_name)

    # If any word has a non-empty left set, a word has been visited without its left being fully visited
    for word in all_words.values():
        if word.left:
            print("Boo bad input this is not solveable")
            return None
        
    return result

print(build(all_words))

# Trivial test for cyclic dependencies
a = Word("a")
a.left.add("a")
test1 = dict()
test1["a"] = a
print(build(test1))
