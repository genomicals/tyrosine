
things = [
    ["queso", "papel", "gato"],
    ["refri", "arbol", "silla"],
    ["mesa", "papel", "sopa"],
    ["refri", "ensalada", "mesa", "perro"],
    ["papel", "arbol", "silla", "perro"],
]

print(things)

class Word:
    name: str
    left: set[str]
    right: set[str]
    left_labels: None | set[str]
    right_labels: None | set[str]

    def __init__(self, name: str):
        self.name = name
        self.left = set()
        self.right = set()
        self.left_labels = None
        self.right_labels = None

    def __repr__(self) -> str:
        #return "Word()"
        return f"\n<WORD {self.name},\n\tLEFT: {self.left.__str__()},\n\tRIGHT: {self.right.__str__()}>\n\tLEFT_LABELS: {self.left_labels}\n\tRIGHT_LABELS: {self.right_labels}\n" 

    #def __str__(self) -> str:
        #return self.left.__str__() + self.right.__str__()
        #return "a"


all_words: dict[str, Word] = dict()

for thing in things:
    for i in range(len(thing)):
        if thing[i] not in all_words:
            all_words[thing[i]] = Word(thing[i])
        if i != 0:
            all_words[thing[i]].left.add(thing[i-1])
        if i != len(thing)-1:
            all_words[thing[i]].right.add(thing[i+1])

#print(all_words)

left_edge = list(filter(lambda x: len(x.left) == 0, all_words.values()))
right_edge = list(filter(lambda x: len(x.right) == 0, all_words.values()))


def optimize_left(node: Word):
    print(f"optimizing_left: {node.name}")
    if len(node.left) == 0:
        node.left_labels = set()
        #print(f"optimizing_left skip: {node.name}")
        return
    cur_words: set[Word] = set()
    new_left_labels: set[str] = set()

    # prepare all subsidiary nodes
    for left_id in node.left:
        left_word = all_words[left_id]
        if left_word.left_labels == None:
            optimize_left(left_word)
        cur_words.add(left_word)
        new_left_labels = new_left_labels.union(left_word.left_labels)
        #print(f"{node.name}\tlabels for {left_word.name} {left_word.left_labels}")
        new_left_labels.add(left_id)

    #if node.left_labels != None:
    #    print("YOOOOOOOo")

    # update our left labels
    node.left_labels = new_left_labels
    #print(f"{node.name} {node.left_labels}")

    # optimize our left side
    for left in cur_words:
        for other_left in cur_words:
            if other_left is left: #don't check same word
                continue
            if left.name in other_left.left_labels:
                node.left.remove(left.name)
                print("OP")
                break


def optimize_right(node: Word):
    #print(f"optimizing_right: {node.name}")
    if len(node.right) == 0:
        node.right_labels = set()
        return
    cur_words: set[Word] = set()
    new_right_labels: set[str] = set()

    # prepare all subsidiary nodes
    for right_id in node.right:
        right_word = all_words[right_id]
        if right_word.right_labels == None:
            optimize_right(right_word)
        cur_words.add(right_word)
        new_right_labels = new_right_labels.union(right_word.right_labels)
        new_right_labels.add(right_id)

    # update our right labels
    node.right_labels = new_right_labels

    # optimize our right side
    for right in cur_words:
        for other_right in cur_words:
            if other_right is right: #don't check same word
                continue
            if right.name in other_right.right_labels:
                node.right.remove(right.name)
                print("OP")
                break
        

for left in right_edge:
    optimize_left(left)
for right in left_edge:
    optimize_right(right)



#print(left_edge)
#print(right_edge)
print(all_words)



