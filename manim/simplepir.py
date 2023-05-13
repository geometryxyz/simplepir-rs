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
        self.wait(0.5)
        self.add(l_arrow)
        self.play(FadeIn(l_arrow), FadeIn(response))
        self.play(MoveToTarget(l_arrow))
        self.wait(0.5)
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
        title = Title("SimplePIR")
        self.add(title)

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
