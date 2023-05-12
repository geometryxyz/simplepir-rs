from manim import *

def hero_slide(slide, title_text):
    title = Title(title_text)
    title.center()
    slide.add(title)

def title_and_content_slide(slide, title_text, content, add_content=True):
    title = Title(title_text)
    slide.add(title)
    if add_content:
        slide.add(content)
    content.center()
