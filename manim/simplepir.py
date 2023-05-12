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
        # Title
        title = Title("Problem Statement")
        self.add(title)

        # User
        user = Text("User")
        user.to_edge(LEFT)

        # Request

        request = Tex(r"What's the balance of \texttt{0xabcd}?")
        r_arrow = Arrow(start=LEFT, end=RIGHT)
        r_arrow.next_to(request, DOWN)

        r_arrow.generate_target()
        r_arrow.target.shift(2 * RIGHT)
        self.add(r_arrow)

        # Response
        response = Tex("It's 123 ETH")
        response.next_to(r_arrow, DOWN, buff=1)

        l_arrow = Arrow(start=RIGHT, end=LEFT)
        l_arrow.next_to(response, DOWN)
        l_arrow.generate_target()
        l_arrow.target.shift(2 * LEFT)

        # Note
        note = Tex(r"The Server now knows that the User is interested in \texttt{0xabcd}!")
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
        self.wait(5)
        self.next_slide()
        self.clear()
