def listSplit(lst, condition):
    acc = []
    for e in lst:
        if condition(e):
            yield acc
            acc = []
        else:
            acc.append(e)
    yield acc