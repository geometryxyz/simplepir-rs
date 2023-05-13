from manim import *
import numpy as np

def gen_matrix(rows, label_font_size=30):
    m = Matrix(rows)
    return m

def matrix_dimension_txt(m):
    num_cols = len(m)
    num_rows = len(m.get_rows())
    return "(" + str(num_rows) + " x " + str(num_cols) + ")"

def mat_mul(m0, m1):
    a = np.array(m0)
    b = np.array(m1)
    c = np.matmul(a, b)
    return c.tolist()

def gen_mat_mul_items(m0_data, m1_data):
    m2_data = mat_mul(m0_data, m1_data)

    m0 = gen_matrix(m0_data)
    times = Tex(r"$\times$")
    m1 = gen_matrix(m1_data)
    m2 = gen_matrix(m2_data)

    eq = Tex(r"$=$")

    times.next_to(m0, RIGHT)
    m1.next_to(times, RIGHT)
    eq.next_to(m1, RIGHT)
    m2.next_to(eq, RIGHT)

    group = Group(m0, m1, times, eq, m2)
    group.center()

    items = {
        "m0": m0,
        "times": times,
        "m1": m1,
        "eq": eq,
        "m2": m2,
        "group": group,
    }

    return items


def play_row_col_mul(slide, mm_items, left_row_index, right_col_index):
    i = left_row_index
    j = right_col_index
    m0 = mm_items["m0"]
    m1 = mm_items["m1"]
    m2 = mm_items["m2"]

    m0_sr = SurroundingRectangle(m0.get_rows()[i])
    m1_sr = SurroundingRectangle(m1.get_columns()[j])
    m2_sr = SurroundingRectangle(m2.get_entries()[i * 2 + j])

    slide.play(FadeIn(m0_sr), FadeIn(m1_sr))

    t0 = Transform(m0_sr, m2_sr)
    t1 = Transform(m1_sr, m2_sr)
    slide.play(t0, t1)
    slide.remove(m0_sr)
    slide.remove(m1_sr)
    slide.remove(m2_sr)

def gen_mat_mul_slide(slide, m0_data, m1_data):
    m0_data = [[1, 2, 3], [4, 5, 6]]
    m1_data = [[1, 0], [0, 1], [1, 0]]
    mm_items = gen_mat_mul_items(m0_data, m1_data)
    group = mm_items["group"]
    group.center()

    slide.add(group)

    for i in range(0, len(mm_items["m0"]) - 1):
        for j in range(0, len(mm_items["m1"]) - 1):
            play_row_col_mul(slide, mm_items, i, j)
