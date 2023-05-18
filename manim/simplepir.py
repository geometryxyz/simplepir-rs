from manim import *
from manim_slides import Slide
from layouts import title_and_content_slide, hero_slide
from matmul import gen_mat_mul_slide, gen_matrix, mat_mul
import numpy as np

def textbox(text):
    result = VGroup()
    box = Rectangle()
    text = Text(text).move_to(box.get_center())
    result.add(box, text)
    return result


def e_note():
    return Tex(r"Note: $e < \frac{1}{2} \cdot \lfloor q / p \rfloor$, so it gets rounded away")

def legend_tex():
    return Tex(r"$\mu$ is the plaintext mod $p$\\$A$ is a public matrix mod $q$\\$s$ is a secret vector mod q\\$c$ is the ciphertext mod q\\$\lfloor q / p \rfloor$ is $q / p$ rounded down")


def example_data():
    db = [[1, 1], [1, 0]]
    a = [[1, 2], [3, 4]]
    hint = mat_mul(db, a)

    s = [[1], [0]]
    e = [[1], [1]]
    delta_u = [[0], [3]]
    qu = (np.array(mat_mul(a, s))+ np.array(e) + np.array(delta_u)) % 7
    ans = np.array(mat_mul(db, qu)) % 7

    return {
        "db": db,
        "a": a,
        "e": e,
        "s": s,
        "hint": hint,
        "qu": qu,
        "ans": ans,
        "delta_u": delta_u
    }


class SimplePIR(Slide):
    def slide_0(self):
        title = Title("Private Information Retrieval", font_size=80)
        title.center()

        text = Paragraph("Koh Wei Jie\nMay 2023\nTwitter: @weijiek", font_size=25, alignment="right")
        text.to_edge(DR)

        self.add(title, text)


    def slide_1(self):
        self.slide_user_server(
            "Problem Statement",
            r"What's the balance of \texttt{0xabcd}?",
            "It's 123 ETH",
            r"The Server now knows that the User is interested in \texttt{0xabcd}!"
        )

    def slide_2(self):
        self.slide_user_server(
            "Ideal solution",
            "What's the balance of XXX?",
            "It's YYY",
            r"The User learns that \texttt{0xabcd} has 123 ETH but the \\Server doesn't learn which address the user is interested in!"
        )

    def slide_user_server(self, title_text, request_text, response_text, note_text):
        # Title
        title = Title(title_text)
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Request
        request = Tex(request_text)
        r_arrow = Arrow(start=LEFT, end=RIGHT)
        r_arrow.next_to(request, DOWN)

        r_arrow.generate_target()

        # Response
        response = Tex(response_text)
        response.next_to(r_arrow, DOWN, buff=1)

        l_arrow = Arrow(start=RIGHT, end=LEFT)
        l_arrow.next_to(response, DOWN)

        # Note
        note = Tex(note_text)
        note.next_to(l_arrow, DOWN)

        # Server
        server = Text("Server")
        server.to_edge(RIGHT)

        # Group to center elements
        group = Group(request, r_arrow, response, l_arrow)
        group.center()

        # Animation
        self.add(user, server)
        self.play(FadeIn(r_arrow), FadeIn(request))
        self.add(r_arrow)
        self.wait(0.1)
        self.add(l_arrow)
        self.play(FadeIn(l_arrow), FadeIn(response))
        self.add(l_arrow)
        self.wait(0.1)
        self.add(note)


    def slide_3(self):
        # Title
        title = Title("Potential use cases")
        self.add(title)

        # Bulleted list
        blist = BulletedList(
            "Private block explorers",
            "Private anti-phishing site blockers",
            "Private contact lookups",
            "Private precomputed witness data for ZK protocols"
        )
        self.add(blist)


    def slide_4(self):
        # Title
        title = Title("Building blocks")
        self.add(title)

        blist = BulletedList(
            "Matrices",
            "Regev encryption / Learning with Errors",
            "SimplePIR",
            "DoublePIR"
        )
        self.add(blist)


    def slide_5(self):
        # Title
        title = Title("Matrix multiplication")
        self.add(title)

        m0_data = [[1, 2, 3], [4, 5, 6]]
        m1_data = [[1, 0], [0, 1], [1, 0]]

        gen_mat_mul_slide(self, m0_data, m1_data)

    def slide_6(self):
        # Title
        title = Title("Regev encryption")
        self.add(title)

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

    def slide_7(self):
        # Title
        title = Title("Learning with Errors (LWE)")
        self.add(title)

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

        note = Tex(r"$e$ is a noise vector of small elements.\\Now, it's difficult to recover $s$!")
        note.to_edge(DOWN)

        g = Group(a, a_label, times, s, s_label, plus, e, e_label, eq, b, b_label)
        g.center()
        self.add(g)
        self.add(title)
        self.add(note)

        e_rect = SurroundingRectangle(Group(e, e_label)).scale(1.25)
        self.add(e_rect)

    def slide_8(self):
        # Title
        title = Title("Regev encryption and decryption")
        self.add(title)

        legend = legend_tex()
        legend.center()
        self.add(legend)

    def regev(self, show_sr):
        # Title
        title = Title("Regev encryption and decryption")
        self.add(title)

        label_0 = Text("Encryption")

        encrypt = Tex(r"$c = A \times s + e + \lfloor q / p \rfloor \cdot \mu$")
        encrypt.next_to(label_0, DOWN)

        a_s = SurroundingRectangle(encrypt[0][2:5])

        label_1 = Text("Decryption")
        label_1.next_to(encrypt, DOWN, buff=0.5)

        decrypt = Tex(r"$\mu = c - A \times s\; \mathsf{mod}\; q$\\rounded to the nearest multiple of $\lfloor q / p \rfloor$")
        decrypt.next_to(label_1, DOWN)
        subtract_a_s = SurroundingRectangle(decrypt[0][4:7])

        note = Tex(r"$A \times s$ can only be found knowing $s$")
        note.next_to(decrypt, DOWN, buff=0.5)

        group = Group(label_0, encrypt, label_1, decrypt, note)

        if show_sr:
            group.add(a_s, subtract_a_s)

        group.center()
        self.add(group)


    def slide_9(self):
        self.regev(False)


    def slide_10(self):
        self.regev(True)

    def slide_11(self):
        title = Title("Homomorphic addition")
        self.add(title)

        content = Tex(
            r"""
            $c_0 = \mathsf{encrypt}(p_0, s, e_0)$\\
            $c_1 = \mathsf{encrypt}(p_1, s, e_1)$\\
            $c_0 + c1 = \mathsf{encrypt}(p_0 + p1, s, e_0 + e1)$
            """
        )

        warning = Tex(
            r"""
            $e_0 + e_1$ is noise growth and must be tracked\\
            Note that $c_0 + c_1$ also adds matrix $A$
            """
        )
        warning.next_to(content, DOWN)

        group = Group(content)
        group.center()
        self.add(content)

        r_1 = SurroundingRectangle(content[0][38:43])
        r_2 = SurroundingRectangle(content[0][52:57])
        self.play(FadeIn(r_1))
        self.play(FadeIn(r_2))

        self.add(warning)

    def slide_12(self):
        title = Title("Homomorphic multiplication")
        self.add(title)

        content = Tex(
            r"""
            $c_0 = \mathsf{encrypt}(p_0, s, e_0)$\\
            $c_0 \cdot p_1 = \mathsf{encrypt}(p_0 \cdot p_1, s, e_0 \cdot p_1)$\\
            Note that $c_0 \cdot p_1$ also multiplies matrix $A$
            """
        )
        group = Group(content)
        group.center()
        self.add(content)

        r_1 = SurroundingRectangle(content[0][19:24])
        r_2 = SurroundingRectangle(content[0][33:38])
        self.play(FadeIn(r_1))
        self.play(FadeIn(r_2))

    def slide_13(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        qu = Matrix([[1], [0]])
        qu.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(qu, RIGHT)

        result = Matrix([[3], [5]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We can multiply the by $[1, 0]$ to get the first column")
        note.to_edge(DOWN)

        group = Group(m, times, qu, eq, result)
        group.center()
        self.add(group, note)

    def slide_14(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        qu = Matrix([[0], [1]])
        qu.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(qu, RIGHT)

        result = Matrix([[4], [6]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We can multiply the matrix by $[0, 1]$ to get the second column")
        note.to_edge(DOWN)

        group = Group(m, times, qu, eq, result)
        group.center()
        self.add(group, note)

    def slide_15(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        qu = Matrix([["\mathsf{e}(1)\,"], ["\mathsf{e}(0)"]])
        qu.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(qu, RIGHT)

        result = Matrix([["\mathsf{???}\;"], ["\mathsf{???}"]])
        result.next_to(eq, RIGHT)

        note = Tex(r"What if we multiply by encrypted $1$s and $0$s?")
        note.to_edge(DOWN)

        group = Group(m, times, qu, eq, result)
        group.center()
        self.add(group, note)

    def slide_16(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        qu = Matrix([["\mathsf{e}(1)\,"], ["\mathsf{e}(0)"]])
        qu.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(qu, RIGHT)

        result = Matrix([["\mathsf{e}(3)\;"], ["\mathsf{e}(5)"]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We'll get an encrypted row of the data.")
        note.to_edge(DOWN)

        group = Group(m, times, qu, eq, result)
        group.center()
        self.add(group, note)

    def slide_17(self):
        title = Title("SimplePIR")
        self.add(title)

        db = Matrix([[3, 4], [5, 6]])
        db_label = Tex("$\mathsf{db}$")
        db_label.next_to(db, UP)

        times = Tex(r"$\times$")
        times.next_to(db, RIGHT)

        qu = Matrix([["\mathsf{e}(1)\,"], ["\mathsf{e}(0)"]])
        qu.next_to(times, RIGHT)

        qu_label = Tex("$\mathsf{qu}$")
        qu_label.next_to(qu, UP)

        note = Text("Let's apply this idea to perform PIR!")
        note.to_edge(DOWN)
        self.add(note)

        group = Group(db, db_label, times, qu, qu_label)
        group.center()
        self.add(group)

    def slide_18(self):
        # Title
        title = Title("SimplePIR")
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Hint
        hint = Tex(r"$\mathsf{hint}_c = \mathsf{db} \times A$")
        hint_arrow = Arrow(start=RIGHT, end=LEFT)
        hint_arrow.next_to(hint, DOWN)

        # Note
        note = Tex(r"$\mathsf{hint}_c$ is sent beforehand.")
        note.to_edge(DOWN)

        # Server
        server = Text("Server")
        server.to_edge(RIGHT)

        # Animation
        self.add(user, server)
        self.play(FadeIn(hint_arrow), FadeIn(hint))
        self.add(hint_arrow)
        self.add(note)

    def slide_19(self):
        # Title
        title = Title("SimplePIR")
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Query
        query = Tex(r"$\mathsf{qu} = [\mathsf{e}(1)], [\mathsf{e(0)}]$")
        query.next_to(title, DOWN, buff=1)

        query_arrow = Arrow(start=LEFT, end=RIGHT)
        query_arrow.next_to(query, DOWN)

        # Answer
        answer = Tex(r"$\mathsf{ans} = [\mathsf{e}(3), \mathsf{e(4)}]$")
        answer.next_to(query_arrow, DOWN)

        answer_arrow = Arrow(start=RIGHT, end=LEFT)
        answer_arrow.next_to(answer, DOWN)

        # Note
        note = Tex(
            """
            The user can now decrypt the answer to get the value they want.\\
            Since their query is encrypted, the server doesn't know which value it is!
            """,
            font_size=40
        )
        note.to_edge(DOWN)

        # Server
        server = Text("Server")
        server.to_edge(RIGHT)

        # Animation
        self.add(user, server)
        self.add(query_arrow)
        self.play(FadeIn(query_arrow), FadeIn(query))
        self.add(query_arrow)
        self.wait(0.1)
        self.add(answer_arrow)
        self.play(FadeIn(answer_arrow), FadeIn(answer))
        self.add(answer_arrow)
        self.add(note)

    def slide_20(self):
        # Title
        title = Title("SimplePIR by hand")
        self.add(title)

        """
        q = 7
        p = 2
        m = 2
        n = 2
        floor(q/p) = 3

        A = n x m = 2 x 2
        db = m x m = 2 x 2
        """

        data = example_data()
        db_data = data["db"]
        a_data = data["a"]
        hint_data = data["hint"]

        db = Matrix(db_data)
        times = Tex(r"$\times$")
        times.next_to(db, RIGHT)

        a = Matrix(a_data)
        a.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(a, RIGHT)

        hint = Matrix(hint_data)
        hint.next_to(eq, RIGHT)

        group = Group(a, db, hint, times, eq)
        group.center()

        db_label = Tex(r"$\mathsf{db}$")
        db_label.next_to(db, UP)
        self.add(db_label)

        a_label = Tex(r"$A$")
        a_label.next_to(a, UP)
        self.add(a_label)

        hint_label = Tex(r"$\mathsf{hint}$")
        hint_label.next_to(hint, UP)
        self.add(hint_label)

        note = Tex(r"e.g. we want the element at row 1, col 1\\Let $q=7, p=2$\\Server computes $\mathsf{hint} = \mathsf{db} \times A$")
        note.to_edge(DOWN)
        self.add(note)

        self.add(group)


    def slide_21(self):
        # Title
        title = Title("SimplePIR by hand")
        self.add(title)

        data = example_data()
        a_data = data["a"]
        s_data = data["s"]
        e_data = data["e"]
        delta_u_data = data["delta_u"]

        a = Matrix(a_data)

        times = Tex(r"$\times$")
        times.next_to(a, RIGHT)
        
        s_data = data["s"]
        s = Matrix(s_data)
        s.next_to(times, RIGHT)

        plus_1 = Tex(r"$+$")
        plus_1.next_to(s)

        e = Matrix(e_data)
        e.next_to(plus_1, RIGHT)

        plus_2 = Tex(r"$+$")
        plus_2.next_to(e)

        delta_u = Matrix(delta_u_data)
        delta_u.next_to(plus_2, RIGHT)

        delta_u_label = Tex(r"$\lfloor 7 / 2 \rfloor \cdot [0, 1]$", font_size=35)

        eq = Tex(r"$=$")
        eq.next_to(delta_u, RIGHT)

        qu_data = data["qu"]
        qu_data %= 7
        qu = Matrix(qu_data)
        qu.next_to(eq, RIGHT)

        qu_label = Tex(r"$\mathsf{qu}$")

        note = Tex(r"User computes the query\\Note that we mod $q = 7$", font_size=50)
        note.to_edge(DOWN)
        self.add(note)

        group = Group(a, times, s, plus_1, e, plus_2, delta_u, eq, qu)
        group.center()
        self.add(group)

        a_label = Tex(r"$\mathsf{A}$")
        a_label.next_to(a, UP)
        s_label = Tex(r"$\mathsf{s}$")
        s_label.next_to(s, UP)
        e_label = Tex(r"$\mathsf{e}$")
        e_label.next_to(e, UP)
        delta_u_label.next_to(delta_u, UP)
        qu_label.next_to(qu, UP)
        self.add(a_label, s_label, e_label, delta_u_label, qu_label)


    def slide_22(self):
        # Title
        title = Title("SimplePIR by hand")
        self.add(title)

        data = example_data()

        db_data = data["db"]
        db = Matrix(db_data)
        times = Tex(r"$\times$")
        times.next_to(db, RIGHT)

        qu_data = data["qu"]
        qu_data %= 7
        qu = Matrix(qu_data)
        qu.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(qu, RIGHT)
        
        ans_data = data["ans"]
        ans = Matrix(ans_data)
        ans.next_to(eq, RIGHT)

        group = Group(db, times, qu, eq, ans)
        group.center()
        self.add(group)

        db_label = Tex("$\mathsf{db}$")
        db_label.next_to(db, UP)

        qu_label = Tex("$\mathsf{qu}$")
        qu_label.next_to(qu, UP)

        ans_label = Tex("$\mathsf{ans}$")
        ans_label.next_to(ans, UP)

        note = Tex(r"Server computes the homomorphically encrypted row", font_size=50)
        note.to_edge(DOWN)
        self.add(note)

        self.add(db_label, qu_label, ans_label)


    def slide_23(self):
        # Title
        title = Title("SimplePIR by hand")
        self.add(title)

        data = example_data()
        db_data = data["db"]
        a_data = data["a"]
        s_data = data["s"]
        hint_data = [data["hint"][1]]
        ans_data = [data["ans"][1]]

        ans = Matrix(ans_data)

        minus = Tex(r"$-$")
        minus.next_to(ans)

        hint = Matrix(hint_data)
        hint.next_to(minus)

        times = Tex(r"$\times$")
        times.next_to(hint)

        s = Matrix(s_data)
        s.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(s, RIGHT)

        result_data = [np.array(ans_data) - np.array(mat_mul(hint_data, s_data))][0]
        result = Matrix(result_data)

        result.next_to(eq)

        group = Group(ans, minus, hint, times, s, eq, result)
        group.center()
        self.add(group)

        note = Tex("$[" + str(result_data[0][0]) + r"]$ rounded to the nearest multiple of 3 \\and divided by 3 = 0 mod 2 = 0.\\$\mathsf{db}[1][1]$ does equal $0$. QED.")
        note.to_edge(DOWN)
        self.add(note)

        ans_label = Tex("$\mathsf{ans}[1:]$")
        ans_label.next_to(ans, UP)
        hint_label = Tex("$\mathsf{hint}[1:]$")
        hint_label.next_to(hint, UP)
        s_label = Tex("$\mathsf{s}$")
        s_label.next_to(s, UP)
        result_label = Tex("$\hat{d}$")
        result_label.next_to(result, UP)

        self.add(ans_label, hint_label, s_label, result_label)


    def slide_24(self):
        # Title
        title = Title("DoublePIR")
        self.add(title)

        simplepir_label = Text("SimplePIR")
        simplepir_elems = Tex(r"$\mathsf{db}$, $\mathsf{hint}_c = \mathsf{db} \times A_1$")
        simplepir_elems.next_to(simplepir_label, RIGHT)

        doublepir_label = Text("DoublePIR")
        doublepir_label.next_to(simplepir_label, DOWN)
        doublepir_elems = Tex(r"$\mathsf{db}$, $\mathsf{hint}_s$, $\mathsf{hint}_c = \mathsf{hint}_s \times A_2$")
        doublepir_elems.next_to(doublepir_label, RIGHT)

        note = Tex(r"i.e. recursively apply SimplePIR upon $\mathsf{hint}_c$")
        note.to_edge(DOWN)
        
        group = Group(simplepir_label, simplepir_elems, doublepir_label, doublepir_elems)
        group.center()
        self.add(group, note)


    def slide_25(self):
        self.slide_benchmark(
            "SimplePIR on a 1GB database",
            "121MB hint",
            "242KB query",
            "Benchmarks - 10GB/s/core throughput"
        )


    def slide_26(self):
        self.slide_benchmark(
            "DoublePIR on a 1GB database",
            "16MB hint",
            "345KB query",
            "Benchmarks - 7.4GB/s/core throughput"
        )

        
    def slide_27(self):
        # Title
        title = Title("Links")
        self.add(title)

        # Bulleted list
        blist = BulletedList(
            r"SimplePIR paper: https://eprint.iacr.org/2022/949.pdf",
            r"PIR from scratch: https://blintzbase.com/posts/pir-and-fhe-from-scratch",
            r"Sample code: https://geometryresearch.github.io/simplepir-rs",
            font_size=40
        )

        blist.center()
        self.add(blist)


    def slide_benchmark(self, title, hint, query, note):
        # Title
        title = Title(title)
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Hint
        hint = Tex(hint)
        hint_arrow = Arrow(start=RIGHT, end=LEFT)
        hint_arrow.next_to(hint, DOWN)

        # Query
        query = Tex(query)
        query.next_to(hint_arrow, DOWN, buff=1)
        query_arrow = Arrow(start=LEFT, end=RIGHT)
        query_arrow.next_to(query, DOWN)

        # Note
        note = Tex(note)
        note.to_edge(DOWN)

        # Server
        server = Text("Server")
        server.to_edge(RIGHT)

        group = Group(user, server)
        group.center()
        self.add(note)

        group2 = Group(hint, hint_arrow, query, query_arrow)
        group2.center()

        # Animation
        self.add(user, server, hint, query, hint_arrow, query_arrow)
        self.add(group)


    def construct(self):
        self.slide_0()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_1()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_2()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_3()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_4()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_5()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_6()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_7()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_8()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_9()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_10()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_11()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_12()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_13()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_14()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_15()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_16()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_17()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_18()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_19()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_20()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_21()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_22()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_23()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_24()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_25()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_26()
        self.wait(0.1)
        self.next_slide()
        self.clear()

        self.slide_27()
        self.wait(0.1)
        self.next_slide()
        self.clear()
