from manim import *
from manim_slides import Slide
from layouts import title_and_content_slide, hero_slide
from matmul import gen_mat_mul_slide, gen_matrix, mat_mul

def textbox(text):
    result = VGroup()
    box = Rectangle()
    text = Text(text).move_to(box.get_center())
    result.add(box, text)
    return result


def e_note():
    return Tex(r"Note: $e < \frac{1}{2} \cdot \lfloor q / p \rfloor$, so it gets rounded away")

def legend_tex() :
    return Tex(r"$\mu$ is the plaintext mod $p$\\$A$ is a public matrix mod $q$\\$s$ is a secret vector mod q\\$c$ is the ciphertext mod q\\$\lfloor q / p \rfloor$ is $q / p$ rounded down")

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
        r_arrow.target.shift(2 * RIGHT)
        self.add(r_arrow)

        # Response
        response = Tex(response_text)
        response.next_to(r_arrow, DOWN, buff=1)

        l_arrow = Arrow(start=RIGHT, end=LEFT)
        l_arrow.next_to(response, DOWN)
        l_arrow.generate_target()
        l_arrow.target.shift(2 * LEFT)

        # Note
        note = Tex(note_text)
        note.next_to(l_arrow, DOWN)

        # Server

        server = Text("Server")
        server.to_edge(RIGHT)

        # Animation

        self.add(user, server)
        self.play(FadeIn(r_arrow), FadeIn(request))
        self.play(MoveToTarget(r_arrow))
        self.wait(0.1)
        self.add(l_arrow)
        self.play(FadeIn(l_arrow), FadeIn(response))
        self.play(MoveToTarget(l_arrow))
        self.wait(0.1)
        self.add(note)


    def slide_3(self):
        # Title
        title = Title("Potential use cases")
        self.add(title)

        # Bulleted list
        blist = BulletedList(
            "Private block explorers",
            "Private medical encyclopedias",
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
            "Regev encryption",
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

    # def slide_6(self):
        # # Title
        # title = Title("Regev encryption")
        # self.add(title)

        # encrypt = Tex(r"$c = A \times s + e + \lfloor q / p \rfloor \cdot \mu$")

        # decrypt = Tex(r"$\mu = c - A \times s\; \mathsf{mod}\; q$\\rounded to the nearest multiple of $\lfloor q / p \rfloor$")
        # decrypt.next_to(encrypt, DOWN, buff=1)

        # note = Tex(r"Elements of $A$ and $s$ are mod $q$\\Plaintext $\mu$ is mod $p$\\$A$ is public, $s$ is secret")
        # note.to_edge(DOWN)
        # self.add(note)

        # encrypt_and_decrypt = Group(encrypt, decrypt)
        # encrypt_and_decrypt.center()
        # self.add(encrypt_and_decrypt)

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

        # note = e_note()
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

        q = Matrix([[1, 0]])
        q.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(q, RIGHT)

        result = Matrix([[3, 4]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We can multiply a matrix by $[1, 0]$ to get the first row")
        note.to_edge(DOWN)

        group = Group(m, times, q, eq, result)
        group.center()
        self.add(group, note)

    def slide_14(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        q = Matrix([[0, 1]])
        q.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(q, RIGHT)

        result = Matrix([[5, 6]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We can multiply a matrix by $[0, 1]$ to get the second row")
        note.to_edge(DOWN)

        group = Group(m, times, q, eq, result)
        group.center()
        self.add(group, note)

    def slide_15(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        q = Matrix([["\mathsf{e}(1)\,", "\mathsf{e}(0)"]])
        q.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(q, RIGHT)

        # result = Tex("?")
        result = Matrix([["\mathsf{???}\;", "\mathsf{???}"]])
        result.next_to(eq, RIGHT)

        note = Tex(r"What if we multiply by encrypted $1$s and $0$s?")
        note.to_edge(DOWN)

        group = Group(m, times, q, eq, result)
        group.center()
        self.add(group, note)

    def slide_16(self):
        title = Title("SimplePIR - intuition")
        self.add(title)

        m = Matrix([[3, 4], [5, 6]])

        times = Tex(r"$\times$")
        times.next_to(m, RIGHT)

        q = Matrix([["\mathsf{e}(1)\,", "\mathsf{e}(0)"]])
        q.next_to(times, RIGHT)

        eq = Tex(r"$=$")
        eq.next_to(q, RIGHT)

        result = Matrix([["\mathsf{e}(3)\;", "\mathsf{e}(4)"]])
        result.next_to(eq, RIGHT)

        note = Tex(r"We'll get an encrypted row of the data.")
        note.to_edge(DOWN)

        group = Group(m, times, q, eq, result)
        group.center()
        self.add(group, note)

    def slide_17(self):
        title = Title("SimplePIR")
        self.add(title)

        db = Matrix([[3, 4], [5, 6]])
        db_label = Text("db")
        db_label.next_to(db, UP)

        times = Tex(r"$\times$")
        times.next_to(db, RIGHT)

        q = Matrix([["\mathsf{e}(1)\,", "\mathsf{e}(0)"]])
        q.next_to(times, RIGHT)

        q_label = Text("q")
        q_label.next_to(q, UP)

        group = Group(db, db_label, times, q, q_label)
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

        hint_arrow.generate_target()
        hint_arrow.target.shift(2 * LEFT)
        self.add(hint_arrow)

        # Note
        note = Tex(r"$\mathsf{hint}_c$ is sent beforehand")
        # note.next_to(hint_arrow, DOWN)
        note.to_edge(DOWN)

        # Server

        server = Text("Server")
        server.to_edge(RIGHT)

        # Animation

        self.add(user, server)
        self.play(FadeIn(hint_arrow), FadeIn(hint))
        self.play(MoveToTarget(hint_arrow))
        self.add(note)

    def slide_19(self):
        # Title
        title = Title("SimplePIR")
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Query
        query = Tex(r"$\mathsf{q} = [\mathsf{e}(1), \mathsf{e(0)}]$")
        query.next_to(title, DOWN, buff=1)

        query_arrow = Arrow(start=LEFT, end=RIGHT)
        query_arrow.next_to(query, DOWN)
        query_arrow.generate_target()
        query_arrow.target.shift(2 * RIGHT)

        # Answer
        answer = Tex(r"$\mathsf{ans} = [\mathsf{e}(3), \mathsf{e(4)}]$")
        answer.next_to(query_arrow, DOWN)

        answer_arrow = Arrow(start=RIGHT, end=LEFT)
        answer_arrow.next_to(answer, DOWN)
        answer_arrow.generate_target()
        answer_arrow.target.shift(2 * LEFT)

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

        self.add(note)
        # Animation
        self.add(user, server)
        self.add(query_arrow)
        self.play(FadeIn(query_arrow), FadeIn(query))
        self.play(MoveToTarget(query_arrow))
        self.wait(0.1)
        self.add(answer_arrow)
        self.play(FadeIn(answer_arrow), FadeIn(answer))
        self.play(MoveToTarget(answer_arrow))

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
