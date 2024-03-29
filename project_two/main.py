from constraint import Problem, AllDifferentConstraint


class MyProblem(Problem):

    property_domain: dict[str, set[str]]

    def __init__(self, *args, props, domains, **kwargs):
        super().__init__(*args, **kwargs)
        self.property_domain = {prop: domain for prop, domain in zip(props, domains)}

    def link(self, el1, el2, arg_func: callable, props=None):
        #props must be manually passed to this function if your domains are not pairwise disjoint
        props = props or [self.find_prop(el) for el in (el1, el2)]

        match arg_func:
            case MyProblem.next_to:
                bi_imp_next_to = MyProblem.bi_implication_next_to(el1, el2)
                self.addConstraint(bi_imp_next_to, MyProblem.next_to(*props))

                bi_imp_next_to = MyProblem.bi_implication_next_to(el2, el1)
                self.addConstraint(bi_imp_next_to, MyProblem.next_to(*props[::-1]))

            case MyProblem.left_of:
                bi_imp = MyProblem.bi_implication(el1, el2)
                self.addConstraint(lambda a1, a2: not (el1 in (a1, a2) and el2 in (a1, a2)),
                                   (f"{props[0]}{1}", f"{props[1]}{5}"))
                # left_of leaves prop(el1)0 and prop(el2)5 unconstrained
                # this constraint states that:
                #   if variable: prop(el1)1 == el1 then variable: prop(el2)5 != el2 and vice versa
                self.multi_constraint(bi_imp, arg_func(*props))
            case MyProblem.right_of:
                bi_imp = MyProblem.bi_implication(el1, el2)
                self.addConstraint(lambda a1, a2: not (el1 in (a1, a2) and el2 in (a1, a2)),
                                   (f"{props[0]}{5}", f"{props[1]}{1}"))
                # right_of leaves prop(el1)5 and prop(el2)0 unconstrained
                # this constraint states that:
                #   if variable: prop(el1)5 == el1 then variable: prop(el2)1 != el2 and vice versa
                self.multi_constraint(bi_imp, arg_func(*props))

            case MyProblem.house_wise_groups:
                bi_imp = MyProblem.bi_implication(el1, el2)
                self.multi_constraint(bi_imp, arg_func(*props))

    def find_prop(self, el):
        # this only works for pairwise disjoint domains - which is true for our case
        for prop, domain in self.property_domain.items():
            if el in domain:
                return prop

    def multi_constraint(self, f, args):
        for arg in args:
            self.addConstraint(f, arg)

    @staticmethod
    def next_to(prop1, prop2):
        return f"{prop2}{1}", f"{prop1}{2}", f"{prop2}{3}", f"{prop1}{4}", f"{prop2}{5}"

    @staticmethod
    def left_of(prop1, prop2):
        return [(f"{prop1}{i}", f"{prop2}{i+1}") for i in range(1, 5)]

    @staticmethod
    def right_of(prop1, prop2):
        return [(f"{prop1}{i+1}", f"{prop2}{i}") for i in range(1, 5)]

    @staticmethod
    def house_wise_groups(prop1, prop2):
        return [(f"{prop1}{i}", f"{prop2}{i}") for i in range(1, 6)]

    @staticmethod
    def bi_implication(el1, el2):
        def inner(var1, var2):
            return (el1 == var1) == (el2 == var2)

        return inner

    @staticmethod
    def bi_implication_next_to(el1, el2):
        def inner(a1, a2, a3, a4, a5):

            if el1 not in (a2, a4) and el2 not in (a1, a3, a5):
                return True

            # consider csp.link("Chesterfield", "Fox", MyProblem.next_to)
            # this will call Bi_implication_next_to twice.
            # once with el1 = "Chesterfield", el2 = "Fox" denoted bi_imp1
            # once with el1 = "Fox", el2 = "Chesterfield" denoted bi_imp2
            # the correct assignment A is {"s2":"Chesterfield", "p1":"Fox"}
            # bi_imp2 is called on the arguments s1, p2, s3, p4, s5
            # note that given the assignment A neither "Chesterfield" nor "Fox" will be passed to bi_imp2
            # however that does not mean our current assignment is invalid
            # to account for this we check if el1 not in (a2, a4) and el2 not in (a1, a3, a5) and return true if so

            return (a1 == el2 and a2 == el1) or (a3 == el2 and a2 == el1) \
                or (a3 == el2 and a4 == el1) or (a5 == el2 and a4 == el1)

        return inner

def organize_sol(sol, props):
    return [[sol[f"{prop}{i}"] for prop in props] for i in range(1, 6)]


def main():
    # house = (color, nationality, drink, smoke, pet)
    # there are five houses

    props = ['c', 'n', 'd', 's', 'p']
    domains = [
        ["yellow", "blue", "red", "ivory", "green"],
        ["Norwegian", "Ukrainian", "Englishman", "Spaniard", "Japanese"],
        ["Water", "Tea", "Milk", "OrangeJuice", "Coffee"],
        ["Kools", "Chesterfield", "OldGold", "LuckyStrike", "Parliament"],
        ["Fox", "Horse", "Snails", "Dog", "Zebra"]
    ]

    csp = MyProblem(props=props, domains=domains)

    # initialize variables
    for i in range(1, 6):
        for prop, domain in zip(props, domains):
            if f"{prop}{i}" not in {"d3", "n1"}:
                csp.addVariable(f'{prop}{i}', domain)
        # variable names will be of the form regex: (c|n|d|s|p)(1|2|3|4|5)

    csp.addVariable("n1", ["Norwegian"])
    csp.addVariable("d3", ["Milk"])

    for prop in props:
        varss = [f"{prop}{i}" for i in range(1, 6)]
        csp.addConstraint(AllDifferentConstraint(), varss)

    links = [
        ("Englishman", "red", MyProblem.house_wise_groups),
        ("Spaniard", "Dog", MyProblem.house_wise_groups),
        ("Coffee", "green", MyProblem.house_wise_groups),
        ("Ukrainian", "Tea", MyProblem.house_wise_groups),
        ("green", "ivory", MyProblem.right_of),
        ("OldGold", "Snails", MyProblem.house_wise_groups),
        ("yellow", "Kools", MyProblem.house_wise_groups),
        ("Chesterfield", "Fox", MyProblem.next_to),
        ("Kools", "Horse", MyProblem.next_to),
        ("LuckyStrike", "OrangeJuice", MyProblem.house_wise_groups),
        ("Japanese", "Parliament", MyProblem.house_wise_groups),
        ("Norwegian", "blue", MyProblem.next_to),
    ]


    for args in links:
        csp.link(*args)

    sol = csp.getSolution()
    if sol is None:
        print("No solution - you fucked up")
        return 1

    sol = zip(range(1, 6), organize_sol(sol, props))
    print(*sol, sep="\n")


if __name__ == "__main__":
    main()



