from manim import *
import numpy as np

from matmul import gen_matrix, mat_mul

class Asb(Scene):
    def construct(self):
        title = Title("Learning with errors (LWE)")

        a_data = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]
        a = gen_matrix(a_data)

        a_label = Tex("$A$")
        a_label.next_to(a, UP)

        times = Tex(r"$\times$")
        times.next_to(a, RIGHT)

        s_data = [["."], ["."], ["."]]
        s = gen_matrix(s_data)

        s_label = Tex("$s$")
        s.next_to(times, RIGHT)
        s_label.next_to(s, UP)

        eq = Tex(r"$=$")
        eq.next_to(s, RIGHT)

        b_data = [["."], ["."], ["."]]
        b = gen_matrix(b_data)
        b_label = Tex("$b$")
        b.next_to(eq, RIGHT)
        b_label.next_to(b, UP)

        note = Tex(r"It's easy to recover $s$ from $b$ knowing $A$.\\Note: each value is mod $q$.")
        note.to_edge(DOWN)

        g = Group(a, a_label, times, s, s_label, eq, b, b_label)
        g.center()

        self.add(g)
        self.add(title)
        self.add(note)

class LearningWithErrors(Scene):
    def construct(self):
        title = Title("Learning with errors (LWE)")

        a_data = [[".", ".", "."], [".", ".", "."], [".", ".", "."]]
        a = gen_matrix(a_data)

        a_label = Tex("$A$")
        a_label.next_to(a, UP)

        times = Tex(r"$\times$")
        times.next_to(a, RIGHT)

        s_data = [["."], ["."], ["."]]
        s = gen_matrix(s_data)

        s_label = Tex("$s$")
        s.next_to(times, RIGHT)
        s_label.next_to(s, UP)

        plus = Tex(r"$+$")
        plus.next_to(s, RIGHT)

        e_data = [["."], ["."], ["."]]
        e = gen_matrix(e_data)
        e_label = Tex("$e$")
        e.next_to(plus, RIGHT)
        e_label.next_to(e, UP)

        eq = Tex(r"$=$")
        eq.next_to(e, RIGHT)

        b_data = [["."], ["."], ["."]]
        b = gen_matrix(b_data)
        b_label = Tex("$b$")
        b.next_to(eq, RIGHT)
        b_label.next_to(b, UP)

        note = Tex(r"$e$ is a noise vector.\\It's difficult to recover $s$.")
        note.to_edge(DOWN)

        g = Group(a, a_label, times, s, s_label, plus, e, e_label, eq, b, b_label)
        g.center()
        self.add(g)
        self.add(title)
        self.add(note)
