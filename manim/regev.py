from manim import *

class Regev(Scene):
    def construct(self):
        title = Title("Regev encryption and decryption")

        encrypt = Tex(r"$c = A \times s + e + \lfloor q / p \rfloor \cdot \mu$")

        decrypt = Tex(r"$\mu = c - A \times s\; \mathsf{mod}\; q$\\rounded to the nearest multiple of $\lfloor q / p \rfloor$")
        decrypt.next_to(encrypt, DOWN)

        note = Tex(r"Elements of $A$ and $s$ are mod $q$\\Plaintext $\mu$ is mod $p$\\$A$ is public, $s$ is secret")
        note.to_edge(DOWN)

        encrypt_and_decrypt = Group(encrypt, decrypt)
        encrypt_and_decrypt.center()

        self.add(encrypt_and_decrypt)
        self.add(note)
        self.add(title)
