class FieldElement:
    def __init__(self, num, order):
        if num >= order or num < 0:
            error = 'Num {} not in field range 0 to {}'.format(num, order - 1)
            raise ValueError(error)
        self.num = num
        self.order = order

    def __repr__(self):
        return 'FieldElement_{}({})'.format(self.order, self.num)

    def __eq__(self, other):
        if other is None:
            return False
        return self.num == other.num and self.order == other.order




if __name__ == "__main__":
    a = FieldElement(7, 13)
    b = FieldElement(6, 13)
    print('Does {} equal {}?'.format(a, b))
    print(a == b)

