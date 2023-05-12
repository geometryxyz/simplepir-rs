from manim import *
from manim_slides import Slide
from layouts import title_and_content_slide, hero_slide


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
